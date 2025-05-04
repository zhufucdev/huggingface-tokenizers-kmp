@file:OptIn(ExperimentalForeignApi::class)

package tokenizers

import kotlinx.cinterop.*
import lib.*
import platform.posix.size_tVar
import kotlin.Any
import kotlin.Boolean
import kotlin.Int
import kotlin.NullPointerException
import kotlin.OptIn
import kotlin.String
import kotlin.TODO
import kotlin.UInt
import kotlin.ULong
import kotlin.collections.List
import kotlin.error
import kotlin.experimental.ExperimentalNativeApi
import kotlin.getValue
import kotlin.lazy
import kotlin.let
import kotlin.native.ref.createCleaner
import kotlin.takeIf

actual class Encoding internal constructor(private val inner: StableRef<CPointed>) {
    actual val tokens: List<String> by lazy {
        object : DelegatedList<String>(size) {
            override fun getSafe(index: Int): String =
                encoding_get_token_at(inner.asCPointer(), index.convert()).useContents {
                    error_msg?.use { throw NullPointerException(it.toKString()) }
                    value?.use { it.toKString() }
                    throw NullPointerException(ERROR_EMPTY_RESULT)
                }
        }
    }

    actual val ids: List<UInt> by lazy {
        object : DelegatedList<UInt>(size) {
            override fun getSafe(index: Int): UInt =
                encoding_get_id_at(inner.asCPointer(), index.convert()).useContents {
                    error_msg?.use { throw NullPointerException(it.toKString()) }
                    value
                }
        }
    }

    actual val sequenceIds: List<ULong?> by lazy {
        object : DelegatedList<ULong?>(size) {
            override fun getSafe(index: Int): ULong? =
                encoding_get_sequence_id_at(inner.asCPointer(), index.convert()).useContents {
                    error_msg?.use { throw NullPointerException(it.toKString()) }
                    value.let {
                        if (it > 0u) {
                            it - 1u
                        } else {
                            null
                        }
                    }
                }
        }
    }

    actual val attentionMask: List<UInt> by lazy {
        object : DelegatedList<UInt>(size) {
            override fun getSafe(index: Int): UInt =
                encoding_get_attention_mask_at(inner.asCPointer(), index.convert()).useContents {
                    error_msg?.use { throw NullPointerException(it.toKString()) }
                    value
                }
        }
    }

    actual val size: Int =
        encoding_get_len(inner.asCPointer()).useContents {
            error_msg?.use { throw NullPointerException(it.toKString()) }
            value.toInt()
        }

    actual override fun equals(other: Any?): Boolean =
        other is Encoding && encoding_eq(inner.asCPointer(), other.inner.asCPointer()).useContents {
            error_msg?.use { error(it.toKString()) }
            value
        }

    @OptIn(ExperimentalNativeApi::class)
    private val cleaner = createCleaner(inner) {
        release_encoding(it.asCPointer())
    }

    companion object {
        internal fun fromC(ptr: CPointer<out CPointed>): Encoding = Encoding(ptr.asStableRef())
    }

    actual override fun hashCode(): Int {
        var result = size
        result = 31 * result + tokens.hashCode()
        result = 31 * result + ids.hashCode()
        return result
    }
}