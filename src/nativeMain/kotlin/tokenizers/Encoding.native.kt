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

actual class Encoding private constructor(private val inner: CPointer<out CPointed>) {
    private inline fun <reified T : CVariable, R> Result_List.readListResult(map: (T) -> R): List<R> =
        try {
            error_msg?.use { throw NullPointerException(it.toKString()) }
            value.ptr?.reinterpret<T>()?.let {
                (0 until value.len.toLong()).map { idx -> map(it[idx]) }
            } ?: throw NullPointerException()
        } finally {
            release_list(value.readValue(), sizeOf<T>().convert())
        }

    actual val tokens: List<String> by lazy {
        encoding_get_tokens(inner).useContents {
            readListResult<CPointerVarOf<CPointer<ByteVar>>, String> {
                it.value?.toKString() ?: throw NullPointerException()
            }
        }
    }

    actual val ids: List<UInt> by lazy {
        encoding_get_ids(inner).useContents {
            readListResult<UIntVar, UInt> { it.value }
        }
    }

    actual val sequenceIds: List<ULong?> by lazy {
        encoding_get_sequence_ids(inner).useContents {
            readListResult<size_tVar, ULong?> { it.value.takeIf { it > 0u }?.let { it - 1u } }
        }
    }

    actual val attentionMask: List<UInt> by lazy {
        encoding_get_attention_mask(inner).useContents {
            readListResult<UIntVar, UInt> { it.value }
        }
    }

    actual val size: Int by lazy {
        encoding_get_len(inner).useContents {
            error_msg?.use { throw NullPointerException(it.toKString()) }
            value.toInt()
        }
    }

    actual override fun equals(other: Any?): Boolean =
        other is Encoding && encoding_eq(inner, other.inner).useContents {
            error_msg?.use { error(it.toKString()) }
            value
        }

    @OptIn(ExperimentalNativeApi::class)
    private val cleaner = createCleaner(inner) {
        release_encoding(it)
    }

    companion object {
        internal fun fromC(ptr: CPointer<out CPointed>): Encoding = Encoding(ptr)
    }

    actual override fun hashCode(): Int {
        var result = size
        result = 31 * result + tokens.hashCode()
        result = 31 * result + ids.hashCode()
        return result
    }
}