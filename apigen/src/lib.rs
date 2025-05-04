use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::ops::{Deref, DerefMut};
use syn::punctuated::{Pair, Punctuated};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, Expr, FnArg, GenericArgument, Pat, PathArguments, ReturnType,
    Type, TypePath,
};

#[proc_macro_attribute]
pub fn generate_extern_api(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return proc_macro::TokenStream::from(Error::from(e).write_errors()),
    };

    let params = match GenerateParams::from_list(&attr_args) {
        Ok(params) => params,
        Err(e) => return proc_macro::TokenStream::from(Error::from(e).write_errors()),
    };

    let input = parse_macro_input!(item as syn::ItemFn);
    let mut generated = quote! {
        #input
    };

    if params.c.unwrap_or(true) {
        let c = c_interop(&input, params.return_type);
        generated = quote! {
            #generated
            #c
        }
    }

    if params.jni.unwrap_or(true) {}

    generated.into()
}

fn c_interop(input: &syn::ItemFn, return_type: Option<String>) -> proc_macro2::TokenStream {
    let og_name = &input.sig.ident;
    let fn_name = format_ident!("c_{}", og_name);
    let mut arguments_pat = Vec::with_capacity(input.sig.inputs.len());
    for arg in input.sig.inputs.clone() {
        match arg {
            FnArg::Receiver(_) => return Error::unsupported_format("self").write_errors(),
            FnArg::Typed(t) => {
                if let Pat::Ident(_) = t.pat.deref() {
                    arguments_pat.push(t);
                } else {
                    return Error::unsupported_shape_with_expected(
                        t.pat.span().source_text().unwrap().as_str(),
                        &"identifier".to_string(),
                    )
                    .write_errors();
                }
            }
        }
    }
    let arguments = arguments_pat
        .clone()
        .into_iter()
        .map(|mut t| {
            if let Pat::Ident(id) = t.pat.deref_mut() {
                if let Some(text) = id.ident.span().source_text() {
                    if text.ends_with("ptr") {
                        t.ty = Box::new(Type::from_string("capi::CPointer").unwrap());
                    }
                }
            }
            t
        })
        .map(|e| Pair::Punctuated(FnArg::Typed(e), Comma::default()))
        .collect::<Punctuated<FnArg, Comma>>();
    let call_parameters = arguments_pat
        .iter()
        .map(|pat_type| match pat_type.clone().pat.deref() {
            Pat::Ident(pat_id) => {
                let id_str = pat_id.ident.to_string();
                if id_str.ends_with("ptr") {
                    Pair::Punctuated(
                        Expr::from_string(format!("{}.into()", id_str).as_str())
                            .unwrap()
                            .to_token_stream(),
                        Comma::default(),
                    )
                } else {
                    Pair::Punctuated(pat_id.to_token_stream(), Comma::default())
                }
            }
            _ => Pair::Punctuated(
                Error::unsupported_format(input.sig.inputs.span().source_text().unwrap().as_str())
                    .write_errors()
                    .to_token_stream(),
                Comma::default(),
            ),
        })
        .collect::<Punctuated<proc_macro2::TokenStream, Comma>>();
    let root_caller = quote! {
        #og_name(#call_parameters)
    };

    let BridgeResult { body, rtype } = match &input.sig.output {
        ReturnType::Default => root_caller.into(),
        ReturnType::Type(_, ty) => {
            if let Type::Path(ty) = ty.deref() {
                c_bridge(og_name.to_string().as_str(), ty, return_type, &root_caller)
            } else {
                return Error::unexpected_type(ty.span().source_text().unwrap().as_str())
                    .write_errors();
            }
        }
    };

    match rtype {
        None => {
            quote! {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #fn_name(#arguments) {
                    #body
                }
            }
        }
        Some(rt) => {
            quote! {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #fn_name(#arguments) -> #rt {
                    #body
                }
            }
        }
    }
}

fn c_bridge(
    function_name: &str,
    function_return_type: &TypePath,
    return_type: Option<String>,
    caller: &proc_macro2::TokenStream,
) -> BridgeResult {
    if let Some(seg) = function_return_type.path.segments.last() {
        match seg.ident.to_string().as_str() {
            "Option" => {
                let mut function_name_split = function_name.split('_');
                let error_msg = format!("Null {} pointer.", function_name_split.next().unwrap());

                match seg.arguments.require_inner_types() {
                    Some(inner_types) => match inner_types.first() {
                        None => Error::missing_field("Option<?>").into(),
                        Some(Type::Path(first_inner)) => {
                            let BridgeResult { body, rtype } = c_bridge(
                                function_name_split.next().unwrap_or(function_name),
                                first_inner,
                                return_type,
                                &format_ident!("v").into_token_stream(),
                            );

                            match rtype {
                                None => Error::missing_field("Option<?>").into(),
                                Some(rtype) => {
                                    let ty: Type = parse_quote!(#rtype);
                                    if let Type::Path(tp) = ty {
                                        if let Some(first_seg) = tp.path.segments.first() {
                                            if let Some(last_seg) = tp.path.segments.last() {
                                                if first_seg.ident == "capi"
                                                    && last_seg.ident == "Result"
                                                {
                                                    return BridgeResult::new(
                                                        quote! {
                                                            match #caller {
                                                                None => capi::Result::error_default(#error_msg),
                                                                Some(v) => #body
                                                            }
                                                        },
                                                        rtype,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    BridgeResult::new(
                                        quote! {
                                           match #caller {
                                               None => capi::Result::error_default(#error_msg),
                                               Some(v) => capi::Result::ok(#body)
                                           }
                                        },
                                        quote! {
                                            capi::Result<#rtype>
                                        },
                                    )
                                }
                            }
                        }
                        Some(ty) => {
                            Error::unexpected_type(ty.span().source_text().unwrap().as_str()).into()
                        }
                    },
                    None => Error::missing_field(
                        format!(
                            "first type argument in {}",
                            seg.span().source_text().unwrap()
                        )
                        .as_str(),
                    )
                    .into(),
                }
            }
            "Result" => match seg.arguments.require_inner_types() {
                None => Error::missing_field(
                    format!(
                        "first type argument in {}",
                        seg.span().source_text().unwrap()
                    )
                    .as_str(),
                )
                .into(),

                Some(inner_types) => match inner_types.first() {
                    None => Error::missing_field(
                        format!(
                            "first type argument in {}",
                            seg.span().source_text().unwrap()
                        )
                        .as_str(),
                    )
                    .into(),
                    Some(Type::Path(first_tp)) => c_bridge(
                        function_name,
                        first_tp,
                        return_type,
                        &format_ident!("v").to_token_stream(),
                    )
                    .map(
                        |body| {
                            quote! {
                               Ok(v) => capi::Result::ok(#body),
                               Err(err) => capi::Result::error_default(err)
                            }
                        },
                        |rtype| quote! {capi::Result<#rtype>},
                    ),
                    Some(Type::Tuple(first_tt)) => {
                        if first_tt.elems.len() == 0 {
                            BridgeResult::new(
                                quote! {
                                    Ok(()) => capi::Result::ok(true),
                                    Err(err) => capi::Result::error(err, false)
                                },
                                quote! {capi::Result<bool>},
                            )
                        } else {
                            Error::unexpected_type(first_tt.span().source_text().unwrap().as_str())
                                .into()
                        }
                    }
                    Some(unsupported) => {
                        Error::unexpected_type(unsupported.span().source_text().unwrap().as_str())
                            .into()
                    }
                },
            }
            .map_body(|body| {
                quote! {
                    match #caller {
                        #body
                    }
                }
            }),
            other => {
                if let Some(type_id) = function_return_type.path.get_ident() {
                    if return_type.is_some_and(|it| it == "raw_ptr") {
                        // when return type is `raw_ptr`, must result in usize
                        if type_id == "usize" {
                            BridgeResult::new(
                                quote! {
                                    capi::CPointer::from(#caller)
                                },
                                quote! {capi::CPointer},
                            )
                        } else {
                            Error::unexpected_type(other).into()
                        }
                    } else {
                        BridgeResult::new(quote! {#caller}, quote! {#type_id})
                    }
                } else {
                    Error::custom("only primitive types are supported").into()
                }
            }
        }
    } else {
        Error::unsupported_format(function_return_type.span().source_text().unwrap().as_str())
            .into()
    }
}

#[derive(FromMeta)]
struct GenerateParams {
    jni: Option<bool>,
    c: Option<bool>,
    return_type: Option<String>,
}

trait RequireInnerTypes {
    fn require_inner_types(&self) -> Option<Vec<&Type>>;
}

impl RequireInnerTypes for PathArguments {
    fn require_inner_types(&self) -> Option<Vec<&Type>> {
        match self {
            PathArguments::None => None,
            PathArguments::AngleBracketed(ab) => Some(
                ab.args
                    .iter()
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(inner_t) => Some(inner_t),
                        _ => None,
                    })
                    .collect::<Vec<&Type>>(),
            ),
            PathArguments::Parenthesized(_) => None,
        }
    }
}

struct BridgeResult {
    pub body: proc_macro2::TokenStream,
    pub rtype: Option<proc_macro2::TokenStream>,
}

impl BridgeResult {
    fn new_default_rtype(body: proc_macro2::TokenStream) -> Self {
        Self { body, rtype: None }
    }

    fn new(body: proc_macro2::TokenStream, rtype: proc_macro2::TokenStream) -> Self {
        Self {
            body,
            rtype: Some(rtype),
        }
    }

    fn map<F1, F2>(self, body: F1, rtype: F2) -> Self
    where
        F1: Fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream,
        F2: Fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    {
        Self {
            body: body(self.body),
            rtype: self.rtype.map(rtype),
        }
    }

    fn map_body<F>(self, body: F) -> Self
    where
        F: Fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    {
        Self {
            body: body(self.body),
            rtype: self.rtype,
        }
    }
}

impl Into<BridgeResult> for proc_macro2::TokenStream {
    fn into(self) -> BridgeResult {
        BridgeResult::new_default_rtype(self)
    }
}

impl Into<BridgeResult> for Error {
    fn into(self) -> BridgeResult {
        BridgeResult {
            body: self.write_errors(),
            rtype: None,
        }
    }
}
