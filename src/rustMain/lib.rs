mod bridge;
pub mod capi;
pub mod japi;

#[cfg(test)]
mod tests {
    use tokenizers::Tokenizer;

    #[test]
    fn encoding_test() {
        let tk = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();
        let encoding = tk.encode("Hey there!", true).unwrap();
        println!("len = {}", encoding.len());
        println!("ids = {:?}", encoding.get_ids());
        println!("seq_ids = {:?}", encoding.get_sequence_ids());
        println!("attention_mask = {:?}", encoding.get_attention_mask())
    }
}
