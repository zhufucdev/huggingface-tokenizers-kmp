actual class Encoding private constructor(ptr: Long) {
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

    companion object {
        fun fromPtr(ptr: Long) = Encoding(ptr)
    }
}