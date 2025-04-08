import org.gradle.api.Project
import org.gradle.api.Task
import org.gradle.kotlin.dsl.support.uppercaseFirstChar
import org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeTarget
import org.jetbrains.kotlin.konan.target.KonanTarget

internal val rustupTarget = mapOf(
    KonanTarget.ANDROID_ARM64 to "aarch64-linux-android",
    KonanTarget.ANDROID_ARM32 to "arm-linux-androideabi",
    KonanTarget.ANDROID_X64 to "x86_64-linux-android",
    KonanTarget.ANDROID_X86 to "i686-linux-android",
    KonanTarget.MACOS_X64 to "x86_64-apple-darwin",
    KonanTarget.MACOS_ARM64 to "aarch64-apple-darwin",
    KonanTarget.LINUX_X64 to "x86_64-unknown-linux-gnu",
    KonanTarget.LINUX_ARM64 to "aarch64-unknown-linux-gnu",
    KonanTarget.IOS_X64 to "x86_64-apple-ios",
    KonanTarget.IOS_ARM64 to "aarch64-apple-ios",
    KonanTarget.IOS_SIMULATOR_ARM64 to "aarch64-apple-ios-sim",
    KonanTarget.MINGW_X64 to "x86_64-pc-windows-gnu"
)

internal val KonanTarget.taskName get() = name.split('_').joinToString("") { it.uppercaseFirstChar() }

data class RustSetupResult(val build: CargoCompile, val test: Task)

internal fun setupRustTask(
    konanTarget: KonanTarget,
    project: Project,
    libName: String? = null,
    platform: String = rustupTarget[konanTarget]!!,
): RustSetupResult {
    val taskName = konanTarget.taskName
    val targetArchDir = "target/${platform}"

    val prepareToolchain =
        project.tasks.registerSafe("prepareRustToolchain${taskName}") {
            doLast {
                project.rustupAddTarget(platform)
            }
        }

    val build =
        project.tasks.registerSafe("buildRust${taskName}", CargoCompile::class.java) {
            finalizedBy(project.tasks.generateHeaders)
            dependsOn(prepareToolchain)

            konanTarget(konanTarget)
            libName?.let { libName(it) }
        }

    val test =
        project.tasks.registerSafe("testRust${taskName}") {
            doLast {
                project.cargoRun("test", "--target=$platform", "--target-dir", targetArchDir)
            }
        }

    return RustSetupResult(build, test)
}

fun KotlinNativeTarget.setupRustCompilationTask(): RustSetupResult {
    val targetTuple = rustupTarget[konanTarget] ?: error("Target ${konanTarget.name} is not supported.")
    val setupResult = setupRustTask(
        konanTarget = konanTarget,
        project = project,
        libName = Library.name,
        platform = targetTuple
    )

    binaries {
        all {
            linkerOpts.add(setupResult.build.binaryFile.get().path)
        }
        executable {
            entryPoint = "main"
        }
    }

    compilations.getByName("main") {
        cinterops {
            create("lib") {
                packageName("lib")
                headers(project.tasks.generateHeaders.headerFile)
            }
        }
    }

    project.tasks.named("cinteropLib${konanTarget.taskName}").get()
        .dependsOn(setupResult.build, project.tasks.generateHeaders)

    return setupResult
}