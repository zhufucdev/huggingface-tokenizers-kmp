@file:OptIn(ExperimentalForeignApi::class)

package tokenizers

import kotlinx.cinterop.*
import lib.Result_List
import lib.release_list

internal inline fun <reified T : CVariable, R> Result_List.readListResult(map: (T) -> R): List<R> =
    try {
        error_msg?.use { throw NullPointerException(it.toKString()) }
        if (value.len <= 0u) {
            emptyList()
        } else value.ptr?.reinterpret<T>()?.let {
            (0 until value.len.toLong()).map { idx -> map(it[idx]) }
        } ?: throw NullPointerException(ERROR_EMPTY_RESULT)
    } finally {
        release_list(value.readValue(), sizeOf<T>().convert(), alignOf<T>().convert())
    }

