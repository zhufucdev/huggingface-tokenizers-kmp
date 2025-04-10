import io.ktor.client.HttpClient
import io.ktor.client.engine.darwin.Darwin

actual fun newHttpClient(): HttpClient = HttpClient(Darwin)