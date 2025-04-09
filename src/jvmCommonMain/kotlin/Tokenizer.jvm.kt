actual class Tokenizer private constructor(private val ptr: Long) {
    actual fun encode(content: String, addSpecialTokens: Boolean): Encoding {
        val encodingPtr = try {
            NativeBridge.tokenizerEncode(ptr, content, addSpecialTokens)
        } catch (e: RuntimeException) {
            error(e.message!!)
        }
        return Encoding.fromPtr(encodingPtr)
    }

    actual companion object {
        actual fun fromPretrained(identifier: String): Tokenizer =
            try {
                Tokenizer(NativeBridge.newTokenizerFromPretrained(identifier))
            } catch (e: RuntimeException) {
                error(e.message!!)
            }
    }
}