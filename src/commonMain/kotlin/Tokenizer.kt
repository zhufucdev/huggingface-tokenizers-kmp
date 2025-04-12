expect class Tokenizer {
    fun encode(input: String, withSpecialTokens: Boolean = true): Encoding
    fun encode(inputs: List<String>, addSpecialTokens: Boolean = false): List<Encoding>

    companion object {
        fun fromPretrained(identifier: String): Tokenizer
        fun fromFile(filename: String): Tokenizer
    }
}