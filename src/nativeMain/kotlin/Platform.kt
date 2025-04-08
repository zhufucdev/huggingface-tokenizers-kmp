import kotlinx.cinterop.ExperimentalForeignApi
import lib.plus

actual object Platform {
    const val x = 2
    const val y = 2
    @OptIn(ExperimentalForeignApi::class)
    actual val name = "Native. Also $x + $y is equal to ${plus(x, y)} in Rust"
}
