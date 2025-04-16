import org.gradle.api.DefaultTask
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.TaskAction
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