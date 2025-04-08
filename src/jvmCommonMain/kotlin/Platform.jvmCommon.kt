expect fun platformLoadLib()

actual object Platform {
    const val a = 3
    const val b = 5

    external fun plus(x: Int, y: Int): Int

    init {
        platformLoadLib()
    }

    actual val name = "JVM. Also, $a + $b equals to ${plus(a, b)} in Rust"
}
