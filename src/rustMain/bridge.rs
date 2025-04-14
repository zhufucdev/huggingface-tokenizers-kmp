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

    pub fn tokenizer_encode(
        ptr: usize,
        input: &str,
        add_special_tokens: bool,
    ) -> Option<Result<usize, tokenizers::Error>> {
        let tk = unsafe { (ptr as *mut Tokenizer).as_mut() }?;
        match tk.encode(input, add_special_tokens) {
            Ok(encoding) => {
                let b = Box::new(encoding);

                Some(Ok(Box::into_raw(b) as usize))
            }
            Err(err) => Some(Err(err)),
        }
    }

    pub fn tokenizer_encode_batch<'s, E>(
        ptr: usize,
        inputs: Vec<E>,
        add_special_tokens: bool,
    ) -> Option<Result<Vec<usize>, tokenizers::Error>>
    where
        E: Into<tokenizers::EncodeInput<'s>> + Send,
    {
        let tk = unsafe { (ptr as *mut Tokenizer).as_mut() }?;
        match tk.encode_batch(inputs, add_special_tokens) {
            Ok(encodings) => Some(Ok(encodings
                .into_iter()
                .map(|enc| Box::into_raw(Box::new(enc)) as usize)
                .collect())),
            Err(err) => Some(Err(err)),
        }
    }

    pub fn release_tokenizer(ptr: usize) {
        drop(unsafe { Box::from_raw(ptr as *mut Tokenizer) })
    }

    pub fn encoding_get_tokens(ptr: &usize) -> Option<&[String]> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        Some(en.get_tokens())
    }

    pub fn encoding_get_ids(ptr: &usize) -> Option<&[u32]> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        Some(en.get_ids())
    }

    pub fn encoding_get_sequence_ids(ptr: &usize) -> Option<Vec<Option<usize>>> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        Some(en.get_sequence_ids())
    }
    
    pub fn encoding_get_attention_mask(ptr: &usize) -> Option<&[u32]> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        Some(en.get_attention_mask())
    }

    pub fn encoding_get_len(ptr: &usize) -> Option<usize> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        Some(en.len())
    }

    pub fn encoding_eq(ptr: &usize, other_ptr: &usize) -> Option<bool> {
        let en = unsafe { (*ptr as *mut Encoding).as_ref() }?;
        let en_other = unsafe { (*other_ptr as *mut Encoding).as_ref() }?;
        Some(en == en_other)
    }

    pub fn release_encoding(ptr: usize) {
        drop(unsafe { Box::from_raw(ptr as *mut Encoding) })
    }
}
