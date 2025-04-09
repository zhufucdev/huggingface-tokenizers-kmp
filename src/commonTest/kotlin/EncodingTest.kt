import kotlin.test.Test
import kotlin.test.assertTrue

class EncodingTest {
    val tokenizer: Tokenizer by lazy {
        Tokenizer.fromPretrained(MODEL_ID)
    }

    @Test
    fun encode_one() {
        val encoding = tokenizer.encode("Hey there!")
        assertTrue("Getting empty tokens") { encoding.tokens.isNotEmpty() }
        assertTrue("Getting empty ids") { encoding.ids.isNotEmpty() }
    }
}