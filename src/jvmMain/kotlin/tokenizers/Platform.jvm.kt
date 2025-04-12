package tokenizers

import java.io.File

data class Jni(val prefix: String = "", val postfix: String, val platformTuple: String) {
    val filename: String by lazy { "${prefix}${LIB_NAME}${platformTuple}.${postfix}" }
}

private fun getCopy(): File {
    val jniOf = mapOf(
        ("windows" to "x86_64") to Jni(postfix = "dll", platformTuple = "MingwX64"),
        ("linux" to "x86_64") to Jni(prefix = "lib", postfix = "so", platformTuple = "LinuxX64"),
        ("linux" to "aarch64") to Jni(prefix = "lib", postfix = "so", platformTuple = "LinuxArm64"),
        ("mac" to "aarch64") to Jni(prefix = "lib", postfix = "dylib", platformTuple = "MacosArm64"),
        ("mac" to "x86_64") to Jni(prefix = "lib", postfix = "dylib", platformTuple = "MacosX64")
    )
    val osName = System.getProperty("os.name").lowercase()
    val osArch = System.getProperty("os.arch")
    val jni = jniOf.entries.firstOrNull { osName.contains(it.key.first) && osArch == it.key.second }
        ?.value
        ?: error("Platform (os = $osName, arch = $osArch) is not supported.")

    val ips = NativeBridge.javaClass.getResourceAsStream(jni.filename)
        ?: error("Should never happen. tokenizers.Jni filename = ${jni.filename}")
    val libFile = File.createTempFile(LIB_NAME, jni.platformTuple)
    ips.use {
        libFile.outputStream().use { ops ->
            it.copyTo(ops)
        }
    }

    libFile.deleteOnExit()
    return libFile
}

@Suppress("UnsafeDynamicallyLoadedCode")
actual fun platformLoadLib() {
    try {
        System.loadLibrary("vectoria")
    } catch (_: UnsatisfiedLinkError) {
        val copy = getCopy()
        try {
            System.load(copy.absolutePath)
        } catch (_: UnsatisfiedLinkError) {
            error("Error loading JNI lib (from ${copy.path}).")
        }
    }
}
