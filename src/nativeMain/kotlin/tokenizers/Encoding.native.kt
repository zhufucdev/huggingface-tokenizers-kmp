@file:OptIn(ExperimentalForeignApi::class)

package tokenizers

import kotlinx.cinterop.*
import kotlinx.cinterop.get
import lib.encoding_get_tokens
import lib.encoding_get_ids
import lib.encoding_get_len
import lib.release_list

actual class Encoding private constructor(private val inner: CPointer<out CPointed>) {
    actual val tokens: List<String> by lazy {
        encoding_get_tokens(inner).useContents {
            try {
                error_msg?.use { throw NullPointerException(it.toKString()) }
                value.ptr?.reinterpret<CPointerVarOf<CPointer<ByteVar>>>()?.let {
                    (0 until value.len.toLong()).map { idx ->
                        it[idx]?.use { it.toKString() } ?: throw NullPointerException("index = $idx")
                    }
                } ?: throw NullPointerException()
            } finally {
                release_list(value.readValue(), sizeOf<ByteVar>().convert())
            }
        }
    }

    actual val ids: List<UInt> by lazy {
        encoding_get_ids(inner).useContents {
            try {
                error_msg?.use { throw NullPointerException(it.toKString()) }
                value.ptr?.reinterpret<UIntVar>()?.let {
                    (0 until value.len.toLong()).map { idx -> it[idx] }
                } ?: throw NullPointerException()
            } finally {
                release_list(value.readValue(), sizeOf<UIntVar>().convert())
            }
        }
    }

    actual val size: Int by lazy {
        encoding_get_len(inner).useContents {
            error_msg?.use { throw NullPointerException(it.toKString()) }
            value.toInt()
        }
    }

    companion object {
        internal fun fromC(ptr: CPointer<out CPointed>): Encoding = Encoding(ptr)
    }
}