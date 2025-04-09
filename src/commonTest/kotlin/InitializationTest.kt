import kotlin.test.Test
import kotlin.test.assertFailsWith

class InitializationTest {
    @Test
    fun from_pretrained() {
        Tokenizer.fromPretrained(MODEL_ID)
    }

    @Test
    fun from_pretrained_error() {
        assertFailsWith(IllegalStateException::class) { Tokenizer.fromPretrained("hello") }
    }
}