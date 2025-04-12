package tokenizers

actual class Encoding private constructor(private val ptr: Long) {
    actual val tokens: List<String> by lazy {
        try {
            NativeBridge.encodingGetTokens(ptr)
                .toList()
        } catch (e: RuntimeException) {
            throw NullPointerException(e.message)
        }
    }

    actual val ids: List<UInt> by lazy {
        try {
            NativeBridge.encodingGetIds(ptr)
                .map { it.toUInt() }
        } catch (e: RuntimeException) {
            throw NullPointerException(e.message)
        }
    }

    actual val size: Int by lazy {
        try {
            NativeBridge.encodingGetLen(ptr).toInt()
        } catch (e: RuntimeException) {
            throw NullPointerException(e.message)
        }
    }

    actual override fun equals(other: Any?): Boolean =
        other is Encoding && NativeBridge.encodingEq(ptr, other.ptr)

    actual override fun hashCode(): Int {
        var result = size
        result = 31 * result + tokens.hashCode()
        result = 31 * result + ids.hashCode()
        return result
    }

    companion object {
        fun fromPtr(ptr: Long) = Encoding(ptr)
    }
}