import org.gradle.api.Project
import java.io.File
import java.io.IOException

private const val CARGO = "cargo"
private const val RUSTUP = "rustup"

/**
 * Run a program hosted in CARGO_HOME
 * @param program Name of program binary to run
 * @param installIfAbsent Install the program via [cargoInstall] if the program has been missing
 */
fun Project.rustupRun(vararg args: String, program: String = RUSTUP, installIfAbsent: Boolean = true) {
    val execution = try {
        providers.exec {
            val cargoBin = environment["CARGO_HOME"]?.let { "$it/bin/" }?.takeIf { File(it, program).exists() } ?: ""
            isIgnoreExitValue = true
            commandLine("${cargoBin}${program}", *args)
        }
    } catch (e: IOException) {
        if (installIfAbsent && program != CARGO && program != RUSTUP) {
            cargoInstall(program)
            rustupRun(args = args, program = program, installIfAbsent = false)
        } else {
            throw RuntimeException("Program `$program` is absent and will not be installed.", e)
        }
        return
    }

    if (execution.result.get().exitValue != 0) {
        println(execution.standardError.asText.get())
    }
}

/**
 * Run a cargo command
 * @param args Arguments to be passed to cargo
 */
fun Project.cargoRun(vararg args: String) = rustupRun(*args, program = CARGO)

/**
 * Compile and install a crate
 * @param program Name of the crate
 */
fun Project.cargoInstall(program: String) = cargoRun("install", program)

/**
 * Install a rustup toolchain
 * @param toolchain Names of the desired toolchains
 */
fun Project.rustupAddTarget(vararg toolchain: String) {
    if (toolchain.isEmpty()) {
        return
    }
    rustupRun("target", "add", *toolchain)
}
