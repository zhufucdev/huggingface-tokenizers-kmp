package tokenizers

internal object NativeBridge {
    init {
        platformLoadLib()
    }

    external fun newTokenizerFromPretrained(identifier: String): Long

    external fun newTokenizerFromFile(filename: String): Long

    external fun newTokenizerFromBytes(bytes: ByteArray): Long

    external fun tokenizerEncode(ptr: Long, input: String, addSpecialTokens: Boolean): Long

    external fun tokenizerEncodeBatch(ptr: Long, inputs: Array<String>, addSpecialTokens: Boolean): LongArray

    external fun encodingGetTokenAt(ptr: Long, index: Int): String

    external fun encodingGetIdAt(ptr: Long, index: Int): Int

    external fun encodingGetSequenceIdAt(ptr: Long, index: Int): Long

    external fun encodingGetAttentionMaskAt(ptr: Long, index: Int): Int

    external fun encodingGetLen(ptr: Long): Int

    external fun encodingEq(ptr: Long, otherPtr: Long): Boolean

    external fun releaseTokenizer(ptr: Long)

    external fun releaseEncoding(ptr: Long)
}
