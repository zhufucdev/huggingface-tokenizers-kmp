@file:OptIn(ExperimentalForeignApi::class)

import kotlinx.cinterop.*
import lib.new_tokenizer_from_pretrained
import lib.tokenizer_encode

actual class Tokenizer private constructor(private val inner: CPointer<out CPointed>) {
    actual fun encode(content: String, addSpecialTokens: Boolean): Encoding {
        tokenizer_encode(inner, content, addSpecialTokens).useContents {
            value?.let { return Encoding.fromC(it) }
            error_msg?.use { error(it.toKString()) }
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
    }
}