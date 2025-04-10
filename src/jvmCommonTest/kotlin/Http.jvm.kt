import io.ktor.client.*
import io.ktor.client.engine.cio.*

actual fun newHttpClient(): HttpClient = HttpClient(CIO)