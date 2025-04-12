import tokenizers.Tokenizer
import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class EncodingTest {
    val tokenizer: Tokenizer by lazy {
        Tokenizer.fromPretrained(MODEL_ID)
    }

    @Test
    fun encode_one_without_special_tokens() {
        val encoding = tokenizer.encode("Hey there!", withSpecialTokens = false)
        assertTrue("Getting empty tokens") { encoding.tokens.isNotEmpty() }
        assertTrue("Getting empty ids") { encoding.ids.isNotEmpty() }
    }

    @Test
    fun encode_one() {
        val encoding = tokenizer.encode("Hey there!")
        assertEquals(5, encoding.size, "Mismatching encoding length")
        assertContentEquals(listOf(101u, 4403u, 1175u, 106u, 102u), encoding.ids, "Mismatching tokens")
    }

    @Test
    fun encode_many() {
        val encodings = tokenizer.encode(listOf("Salut!", "Hey there!", "¡hola!"))
        assertEquals(3, encodings.size)
        encodings.forEachIndexed { index, encoding ->
            assertTrue("Empty result at index $index") { encoding.size > 0 }
        }
    }
}