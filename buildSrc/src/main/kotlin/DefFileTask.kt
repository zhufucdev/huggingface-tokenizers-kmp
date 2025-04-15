import org.gradle.api.DefaultTask
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.TaskAction
import org.jetbrains.kotlin.konan.target.Family
import java.io.File

abstract class DefFileTask : DefaultTask() {
    @get:OutputFile
    protected abstract val outputFileProperty: Property<File>

    @get:Internal
    var outputFile: File
        get() = outputFileProperty.get()
        set(value) {
            outputFileProperty.set(value)
        }

    private val Family.properName get() = when(this) {
        Family.OSX ->  "macos"
        Family.MINGW -> "windows"
        else -> name.lowercase()
    }

    @TaskAction
    fun write() {
        outputFile.writeText(
            project.tasks.filterIsInstance<CargoCompile>().joinToString("\n") { curr ->
                val targetFilter = curr.konanTarget.name
                "staticLibraries.$targetFilter = ${curr.staticLinkBinary.get().name}\n" +
                        "libraryPaths.$targetFilter = ${curr.libPath.get().path}"
            }
        )
    }
}