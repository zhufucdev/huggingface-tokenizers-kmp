package tokenizers

import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Test

class TokenizerTest {
    @Test
    fun initialize() {
        Tokenizer.fromPretrained("bert-base-cased")
    }
}