package tokenizers

actual class Tokenizer private constructor(private val ptr: Long) {
    actual fun encode(input: String, withSpecialTokens: Boolean): Encoding {
        val encodingPtr = try {
            NativeBridge.tokenizerEncode(ptr, input, withSpecialTokens)
        } catch (e: RuntimeException) {
            error(e.message!!)
        }
        return Encoding.fromPtr(encodingPtr)
    }

    actual fun encode(inputs: List<String>, withSpecialTokens: Boolean): List<Encoding> =
        try {
            NativeBridge.tokenizerEncodeBatch(ptr, inputs.toTypedArray(), withSpecialTokens)
                .map(Encoding::fromPtr)
        } catch (e: RuntimeException) {
            error(e.message!!)
        }

    protected fun finalize() {
        NativeBridge.releaseTokenizer(ptr)
    }

    actual companion object {
        actual fun fromPretrained(identifier: String): Tokenizer =
            try {
                Tokenizer(NativeBridge.newTokenizerFromPretrained(identifier))
            } catch (e: RuntimeException) {
                error(e.message!!)
            }

        actual fun fromFile(filename: String): Tokenizer =
            try {
                Tokenizer(NativeBridge.newTokenizerFromFile(filename))
            } catch (e: RuntimeException) {
                error(e.message!!)
            }

        actual fun fromBytes(bytes: ByteArray): Tokenizer =
            try {
                Tokenizer(NativeBridge.newTokenizerFromBytes(bytes))
            } catch (e: RuntimeException) {
                error(e.message!!)
            }
    }
}