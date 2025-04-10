expect class Tokenizer {
    fun encode(content: String, addSpecialTokens: Boolean = false): Encoding

    companion object {
        fun fromPretrained(identifier: String): Tokenizer
        fun fromFile(filename: String): Tokenizer
    }
}