import org.gradle.api.DefaultTask
import org.gradle.api.provider.Property
import org.gradle.api.tasks.*
import java.io.File

abstract class CbindGenerate : DefaultTask() {
    @get:OutputFile
    abstract val headerFileProperty: Property<File>

    @get:Optional
    @get:InputFiles
    protected abstract val sourceDirProperty: Property<File>

    init {
        val defaultFile = project.layout.buildDirectory.file("target/lib${Library.name}.h").get().asFile
        headerFileProperty.set(defaultFile)
        sourceDirProperty.set(project.file("src/rustMain"))
    }

    @get:Internal
    var headerFile: File
        get() = headerFileProperty.get()
        set(value) {
            headerFileProperty.set(value)
        }

    @get:Internal
    var sourceDir: File
        get() = sourceDirProperty.get()
        set(value) {
            sourceDirProperty.set(value)
        }

    @TaskAction
    fun generate() {
        project.rustupRun(
            "--output",
            headerFile.path,
            program = "cbindgen"
        )
    }
}