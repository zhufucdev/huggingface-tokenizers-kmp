pub mod bridge {
    use tokenizers::{Encoding, Tokenizer};

    pub fn new_tokenizer_from_pretrained(identifier: &str) -> Result<usize, tokenizers::Error> {
        let b = Box::new(Tokenizer::from_pretrained(identifier, None)?);
        Ok(Box::into_raw(b) as usize)
    }

    pub fn tokenizer_encode
    (
        ptr: usize,
        content: &str,
        add_special_tokens: bool,
    ) -> Option<Result<usize, tokenizers::Error>> {
        let tk = unsafe { (ptr as *mut Tokenizer).as_mut() }?;
        match tk.encode(content, add_special_tokens) {
            Ok(encoding) => {
                let b = Box::new(encoding);

                Some(Ok(Box::into_raw(b) as usize))
            }
            Err(err) => {
                Some(Err(err))
            }
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
}
