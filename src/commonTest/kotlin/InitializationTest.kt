import io.ktor.client.request.*
import io.ktor.client.statement.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import kotlinx.coroutines.runBlocking
import kotlinx.io.buffered
import kotlinx.io.files.Path
import kotlinx.io.files.SystemFileSystem
import kotlinx.io.files.SystemTemporaryDirectory
import tokenizers.Tokenizer
import kotlin.test.Test
import kotlin.test.assertFailsWith

private const val tokenizerUrl =
    "https://huggingface.co/google-bert/bert-base-cased/resolve/main/tokenizer.json?download=true"

class InitializationTest {
    val client = newHttpClient()

    @Test
    fun from_pretrained() {
        run {
            Tokenizer.fromPretrained(MODEL_ID)
        }
    }

    @Test
    fun from_file() {
        val content = runBlocking(Dispatchers.IO) {
            client.get(tokenizerUrl)
                .bodyAsBytes()
        }
        val file = Path(SystemTemporaryDirectory, "tokenizer.json")
        SystemFileSystem
            .sink(file)
            .buffered()
            .use {
                it.write(content)
            }

        Tokenizer.fromFile(file.toString())
    }

    @Test
    fun from_pretrained_error() {
        assertFailsWith(IllegalStateException::class) { Tokenizer.fromPretrained("hello") }
    }

    @Test
    fun from_bytes() {
        val bytes = runBlocking(Dispatchers.IO) {
            client.get(tokenizerUrl)
                .bodyAsBytes()
        }
        Tokenizer.fromBytes(bytes)
    }
}