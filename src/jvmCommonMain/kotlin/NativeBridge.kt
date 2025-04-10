internal object NativeBridge {
    init {
        platformLoadLib()
    }

    external fun newTokenizerFromPretrained(identifier: String): Long

    external fun newTokenizerFromFile(filename: String): Long

    external fun tokenizerEncode(ptr: Long, content: String, addSpecialTokens: Boolean): Long

    external fun encodingGetTokens(ptr: Long): Array<String>

    external fun encodingGetIds(ptr: Long): IntArray
}
