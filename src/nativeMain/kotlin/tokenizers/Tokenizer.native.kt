@file:OptIn(ExperimentalForeignApi::class)

package tokenizers

import kotlinx.cinterop.*
import lib.*
import kotlin.collections.List
import kotlin.experimental.ExperimentalNativeApi
import kotlin.native.ref.createCleaner

actual class Tokenizer private constructor(private val inner: StableRef<CPointed>) {
    actual fun encode(input: String, withSpecialTokens: Boolean): Encoding {
        tokenizer_encode(inner.asCPointer(), input, withSpecialTokens).useContents {
            error_msg?.use { error(it.toKString()) }
            value?.let { return Encoding.fromC(it) }
        }
        error(ERROR_EMPTY_RESULT)
    }

    actual fun encode(inputs: List<String>, addSpecialTokens: Boolean): List<Encoding> =
        memScoped {
            val inputsHeap = inputs.toCStringArray(this)
            tokenizer_encode_batch(inner.asCPointer(), inputsHeap, inputs.size, addSpecialTokens).useContents {
                readListResult<CPointerVarOf<CPointer<*>>, Encoding> {
                    Encoding.fromC(
                        it.value ?: throw NullPointerException()
                    )
                }
            }
        }

    @OptIn(ExperimentalNativeApi::class)
    private val cleaner = createCleaner(inner) {
        release_tokenizer(it.asCPointer())
    }

    actual companion object {
        actual fun fromPretrained(identifier: String): Tokenizer {
            new_tokenizer_from_pretrained(identifier).useContents {
                error_msg?.use { error(it.toKString()) }
                value?.let { return Tokenizer(it.asStableRef()) }
            }

            error(ERROR_EMPTY_RESULT)
        }

        actual fun fromFile(filename: String): Tokenizer {
            new_tokenizer_from_file(filename).useContents {
                error_msg?.use { error(it.toKString()) }
                value?.let { return Tokenizer(it.asStableRef()) }
            }

            error(ERROR_EMPTY_RESULT)
        }

        actual fun fromBytes(bytes: ByteArray): Tokenizer {
            bytes.usePinned { ba ->
                new_tokenizer_from_bytes(ba.addressOf(0), bytes.size).useContents {
                    error_msg?.use { error(it.toKString()) }
                    value?.let { return Tokenizer(it.asStableRef()) }
                }
            }

            error(ERROR_EMPTY_RESULT)
        }
    }
}