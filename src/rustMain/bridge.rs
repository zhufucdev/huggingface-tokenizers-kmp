pub mod bridge {
    use tokenizers::{Encoding, Tokenizer};

    pub fn new_tokenizer_from_pretrained(identifier: &str) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_pretrained(identifier, None)?);
        Ok(Box::into_raw(b) as usize)
    }

    pub fn new_tokenizer_from_file(filename: &str) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_file(filename)?);
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

    pub fn encoding_get_tokens(ptr: usize) -> Option<Vec<String>> {
        let en = unsafe { (ptr as *mut Encoding).as_ref() }?;
        Some(Vec::from(en.get_tokens()))
    }

    pub fn encoding_get_ids(ptr: usize) -> Option<Vec<u32>> {
        let en = unsafe { (ptr as *mut Encoding).as_ref() }?;
        Some(Vec::from(en.get_ids()))
    }

    pub fn encoding_get_len(ptr: usize) -> Option<usize> {
        let en = unsafe { (ptr as *mut Encoding).as_ref() }?;
        Some(en.len())
    }
}
