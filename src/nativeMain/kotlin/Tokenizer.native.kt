@file:OptIn(ExperimentalForeignApi::class)

import kotlinx.cinterop.*
import lib.new_tokenizer_from_file
import lib.new_tokenizer_from_pretrained
import lib.tokenizer_encode
import lib.tokenizer_encode_batch

actual class Tokenizer private constructor(private val inner: CPointer<out CPointed>) {
    actual fun encode(input: String, withSpecialTokens: Boolean): Encoding {
        tokenizer_encode(inner, input, withSpecialTokens).useContents {
            value?.let { return Encoding.fromC(it) }
            error_msg?.use { error(it.toKString()) }
        }
        error(ERROR_EMPTY_RESULT)
    }

    actual fun encode(inputs: List<String>, addSpecialTokens: Boolean): List<Encoding> =
        memScoped {
            val inputsHeap = allocArrayOf(inputs.map { it.utf8.ptr })
            tokenizer_encode_batch(inner, inputsHeap, inputs.size, addSpecialTokens).useContents {
                error_msg?.use { error(it.toKString()) }
                value.ptr?.let {
                    return (0 until value.len.toLong()).map { idx ->
                        Encoding.fromC(
                            it[idx] ?: throw NullPointerException("Encoding index = $idx")
                        )
                    }
                }
            }
            error(ERROR_EMPTY_RESULT)
        }

    actual companion object {
        actual fun fromPretrained(identifier: String): Tokenizer {
            new_tokenizer_from_pretrained(identifier).useContents {
                value?.let { return Tokenizer(it) }
                error_msg?.use { error(it.toKString()) }
            }

            error(ERROR_EMPTY_RESULT)
        }

        actual fun fromFile(filename: String): Tokenizer {
            new_tokenizer_from_file(filename).useContents {
                value?.let { return Tokenizer(it) }
                error_msg?.use { error(it.toKString()) }
            }

            error(ERROR_EMPTY_RESULT)
        }
    }
}