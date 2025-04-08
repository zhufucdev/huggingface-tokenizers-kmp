import junit.framework.TestCase.assertEquals
import org.junit.Test

class JniLoadTest {
    @Test
    fun shouldOutputPlatformName() {
        assertEquals(
            "JVM. Also, ${Platform.a} + ${Platform.b} equals to ${Platform.a + Platform.b} in Rust",
            Platform.name
        )
    }
}