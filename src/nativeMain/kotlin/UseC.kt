@file:OptIn(ExperimentalForeignApi::class)

import kotlinx.cinterop.ByteVarOf
import kotlinx.cinterop.CPointer
import kotlinx.cinterop.ExperimentalForeignApi
import lib.release_cstring_ptr

fun <T> CPointer<ByteVarOf<Byte>>.use(block: (CPointer<ByteVarOf<Byte>>) -> T): T {
    val result = block(this)
    release_cstring_ptr(this)
    return result
}