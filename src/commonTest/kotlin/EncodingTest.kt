import tokenizers.Tokenizer
import kotlin.test.*

class EncodingTest {
    val tokenizer: Tokenizer
        get() = Tokenizer.fromPretrained(MODEL_ID)

    @Test
    fun encode_one_without_special_tokens() {
        val en = tokenizer.encode("Hey there!", withSpecialTokens = false)
        assertTrue("Getting empty tokens") { en.tokens.isNotEmpty() }
        assertTrue("Getting empty ids") { en.ids.isNotEmpty() }
    }

    @Test
    fun encode_one() {
        val en = tokenizer.encode("Hey there!")
        assertEquals(5, en.size, "Mismatching encoding length")
        assertContentEquals(listOf(101u, 4403u, 1175u, 106u, 102u), en.ids, "Mismatching tokens")
        assertContentEquals(listOf(null, 0u, 0u, 0u, null), en.sequenceIds, "Mismatching sequence ids")
        assertContentEquals(listOf(1u, 1u, 1u, 1u, 1u), en.attentionMask, "Mismatching attention mask.")
    }

    @Test
    fun encode_many() {
        val encodings = tokenizer.encode(listOf("Salut!", "Hey there!", "¡hola!"))
        assertEquals(3, encodings.size)
        encodings.forEachIndexed { index, encoding ->
            assertTrue("Empty result at index $index") { encoding.size > 0 }
        }
    }

    @Test
    fun equality_test() {
        val text = "What's good."
        val (a, b) = tokenizer.encode(text) to tokenizer.encode(text)
        assertEquals(a, b)

        val c = tokenizer.encode(text.replace(".", "?"))
        assertNotEquals(a, c, "Inequality failed")
    }
}