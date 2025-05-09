pub mod bridge {
    use std::path::Path;
    use tokenizers::{Encoding, Tokenizer};

    pub fn new_tokenizer_from_pretrained<F: AsRef<str>>(
        identifier: F,
    ) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_pretrained(identifier, None)?);
        Ok(Box::into_raw(b) as usize)
    }

    pub fn new_tokenizer_from_file<F: AsRef<Path>>(
        filename: F,
    ) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_file(filename)?);
        Ok(Box::into_raw(b) as usize)
    }

    pub fn new_tokenizer_from_bytes<P: AsRef<[u8]>>(bytes: P) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_bytes(bytes)?);
        Ok(Box::into_raw(b) as usize)
    }

    pub unsafe fn tokenizer_encode(
        ptr: usize,
        input: &str,
        add_special_tokens: bool,
    ) -> Option<Result<usize, tokenizers::Error>> {
        let tk = (ptr as *mut Tokenizer).as_mut()?;
        match tk.encode(input, add_special_tokens) {
            Ok(encoding) => {
                let b = Box::new(encoding);

                Some(Ok(Box::into_raw(b) as usize))
            }
            Err(err) => Some(Err(err)),
        }
    }

    pub unsafe fn tokenizer_encode_batch<'s, E>(
        ptr: usize,
        inputs: Vec<E>,
        add_special_tokens: bool,
    ) -> Option<Result<Vec<usize>, tokenizers::Error>>
    where
        E: Into<tokenizers::EncodeInput<'s>> + Send,
    {
        let tk = (ptr as *mut Tokenizer).as_mut()?;
        match tk.encode_batch(inputs, add_special_tokens) {
            Ok(encodings) => Some(Ok(encodings
                .into_iter()
                .map(|enc| Box::into_raw(Box::new(enc)) as usize)
                .collect())),
            Err(err) => Some(Err(err)),
        }
    }

    pub unsafe fn release_tokenizer(ptr: usize) {
        drop(Box::from_raw(ptr as *mut Tokenizer))
    }

    pub unsafe fn encoding_get_token_at(ptr: &usize, index: usize) -> Option<&String> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        Some(&en.get_tokens()[index])
    }

    pub unsafe fn encoding_get_id_at(ptr: &usize, index: usize) -> Option<u32> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        Some(en.get_ids()[index])
    }

    pub unsafe fn encoding_get_sequence_id_at(ptr: &usize, index: usize) -> Option<Option<usize>> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        Some(en.get_sequence_ids()[index])
    }

    pub unsafe fn encoding_get_attention_mask_at(ptr: &usize, index: usize) -> Option<u32> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        Some(en.get_attention_mask()[index])
    }

    pub unsafe fn encoding_get_len(ptr: &usize) -> Option<usize> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        Some(en.len())
    }

    pub unsafe fn encoding_eq(ptr: &usize, other_ptr: &usize) -> Option<bool> {
        let en = (*ptr as *mut Encoding).as_ref()?;
        let en_other = unsafe { (*other_ptr as *mut Encoding).as_ref() }?;
        Some(en == en_other)
    }

    pub unsafe fn release_encoding(ptr: usize) {
        drop(Box::from_raw(ptr as *mut Encoding))
    }
}
