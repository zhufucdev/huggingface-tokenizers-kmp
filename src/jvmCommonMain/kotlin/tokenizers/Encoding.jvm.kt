package tokenizers

actual class Encoding private constructor(private val ptr: Long) {
    actual val tokens: List<String> by lazy {
        object : DelegatedList<String>(size) {
            override fun getSafe(index: Int): String = NativeBridge.encodingGetTokenAt(ptr, index)
        }
    }

    actual val ids: List<UInt> by lazy {
        object : DelegatedList<UInt>(size) {
            override fun getSafe(index: Int): UInt =
                try {
                    NativeBridge.encodingGetIdAt(ptr, index).toUInt()
                } catch (e: RuntimeException) {
                    throw NullPointerException(e.message)
                }
        }
    }

    actual val sequenceIds: List<ULong?> by lazy {
        object : DelegatedList<ULong?>(size) {
            override fun getSafe(index: Int): ULong? =
                try {
                    NativeBridge.encodingGetSequenceIdAt(ptr, index)
                        .let {
                            if (it > 0) {
                                it.toULong() - 1u
                            } else {
                                null
                            }
                        }
                } catch (e: RuntimeException) {
                    throw NullPointerException(e.message)
                }
        }
    }

    actual val attentionMask: List<UInt> by lazy {
        object : DelegatedList<UInt>(size) {
            override fun getSafe(index: Int): UInt =
                try {
                    NativeBridge.encodingGetAttentionMaskAt(ptr, index).toUInt()
                } catch (e: RuntimeException) {
                    throw NullPointerException(e.message)
                }
        }
    }

    actual val size: Int = try {
        NativeBridge.encodingGetLen(ptr)
    } catch (e: RuntimeException) {
        throw NullPointerException(e.message)
    }

    actual override fun equals(other: Any?): Boolean =
        other is Encoding && NativeBridge.encodingEq(ptr, other.ptr)

    actual override fun hashCode(): Int {
        var result = size
        result = 31 * result + tokens.hashCode()
        result = 31 * result + ids.hashCode()
        return result
    }

    protected fun finalize() {
        NativeBridge.releaseEncoding(ptr)
    }

    companion object {
        fun fromPtr(ptr: Long) = Encoding(ptr)
    }
}