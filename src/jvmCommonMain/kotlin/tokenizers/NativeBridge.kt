package tokenizers

internal object NativeBridge {
    init {
        platformLoadLib()
    }

    external fun newTokenizerFromPretrained(identifier: String): Long

    external fun newTokenizerFromFile(filename: String): Long

    external fun tokenizerEncode(ptr: Long, input: String, addSpecialTokens: Boolean): Long

    external fun tokenizerEncodeBatch(ptr: Long, inputs: Array<String>, addSpecialTokens: Boolean): LongArray

    external fun encodingGetTokens(ptr: Long): Array<String>

    external fun encodingGetIds(ptr: Long): IntArray

    external fun encodingGetLen(ptr: Long): Int
}
