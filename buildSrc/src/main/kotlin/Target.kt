import gradle.kotlin.dsl.accessors._611a75feaf31951ac969a948928e6651.android
import gradle.kotlin.dsl.accessors._611a75feaf31951ac969a948928e6651.androidComponents
import gradle.kotlin.dsl.accessors._611a75feaf31951ac969a948928e6651.sourceSets
import org.gradle.api.Project
import org.gradle.api.Task
import org.gradle.api.tasks.Copy
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.register
import org.gradle.kotlin.dsl.support.uppercaseFirstChar
import org.gradle.kotlin.dsl.withType
import org.gradle.language.jvm.tasks.ProcessResources
import org.jetbrains.kotlin.gradle.dsl.KotlinMultiplatformExtension
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
    platform: String = rustupTarget[konanTarget]!!,
    configure: CargoCompile.() -> Unit = {}
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
            konanTarget(konanTarget)
            configure(this)
            if (!cross) {
                dependsOn(prepareToolchain)
            }
        }

    val test =
        project.tasks.registerSafe("testRust${taskName}") {
            doLast {
                project.cargoRun("test", "--target=$platform", "--target-dir", targetArchDir)
            }
        }

    return RustSetupResult(build, test)
}

fun KotlinNativeTarget.crabNative(configure: CargoCompile.() -> Unit): RustSetupResult {
    val targetTuple = rustupTarget[konanTarget] ?: error("Target ${konanTarget.name} is not supported.")
    val setupResult = setupRustTask(
        konanTarget = konanTarget,
        project = project,
        platform = targetTuple,
        configure = configure
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
        .dependsOn(project.tasks.generateHeaders)
    project.tasks.named("compileKotlin${konanTarget.taskName}").get()
        .dependsOn(setupResult.build)

    return setupResult
}

fun Project.crabAndroid(configure: CargoCompile.() -> Unit) {
    data class AndroidSetup(
        val abi: String,
        val konanTarget: KonanTarget,
        val crab: RustSetupResult = setupRustTask(konanTarget, project, configure = configure),
    )

    val androidSetups = listOf(
        AndroidSetup("x86_64", KonanTarget.ANDROID_X64),
        AndroidSetup("arm64-v8a", KonanTarget.ANDROID_ARM64)
    )
    val buildJniLibsDir = project.layout.buildDirectory.dir("jniLibs")
    val copyAbiJniLibs = androidSetups.map { setup ->
        val copy = tasks.register("copy${setup.abi.uppercaseFirstChar()}JniLibs", Copy::class) {
            dependsOn(setup.crab.build)
            from(setup.crab.build.binaryFile)
            into(buildJniLibsDir.map { it.file(setup.abi) })
        }
        setup.crab.build.finalizedBy(copy)
        copy
    }

    androidComponents {
        onVariants { variant ->
            afterEvaluate {
                tasks.withType<com.android.build.gradle.internal.tasks.BaseTask>().configureEach {
                    dependsOn(*copyAbiJniLibs.toTypedArray())
                }
            }
        }
    }

    afterEvaluate {
        android {
            sourceSets.configureEach {
                jniLibs.srcDir(buildJniLibsDir)
            }
        }
    }
}

fun Project.crabJvm(configure: CargoCompile.() -> Unit) {
    val jvmSetups =
        listOf(
            KonanTarget.MACOS_ARM64,
            KonanTarget.MACOS_X64,
            KonanTarget.LINUX_ARM64,
            KonanTarget.LINUX_X64,
            KonanTarget.MINGW_X64
        ).map { setupRustTask(it, project, configure = configure) }
    val jniResourceDir = layout.buildDirectory.dir("jniResources")

    val copyJniResources = jvmSetups.map { setup ->
        val platform = setup.build.konanTarget.taskName
        tasks.register<Copy>("copy${platform}JniResources") {
            dependsOn(setup.build)
            from(setup.build.binaryFile)
            val family = setup.build.konanTarget.family
            rename {
                "${family.dynamicPrefix}${setup.build.libName}${platform}.${family.dynamicSuffix}"
            }
            into(jniResourceDir)
        }
    }

    tasks.withType<ProcessResources>().configureEach {
        dependsOn(*copyJniResources.toTypedArray())
    }

    afterEvaluate {
        extensions.configure(KotlinMultiplatformExtension::class) {
            sourceSets {
                jvmMain.configure {
                    resources.srcDir(jniResourceDir)
                }
            }
        }
    }
}