@file:OptIn(NativeRuntimeApi::class)

import tokenizers.Tokenizer
import kotlin.native.runtime.GC
import kotlin.native.runtime.NativeRuntimeApi
import kotlin.test.Test

class LeakTest {
    @Test
    fun tokenizer() {
        fun createAndDrop() {
            Tokenizer.fromPretrained(MODEL_ID)
        }
        createAndDrop()
        GC.collect()
    }

    @Test
    fun encoding() {
        fun createAndDrop() {
            val tokenizer = Tokenizer.fromPretrained(MODEL_ID)
            tokenizer.encode("Hey there!")
        }
        createAndDrop()
        GC.collect()
    }
}