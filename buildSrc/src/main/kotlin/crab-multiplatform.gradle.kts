import org.gradle.kotlin.dsl.support.uppercaseFirstChar
import org.jetbrains.kotlin.gradle.dsl.KotlinMultiplatformExtension
import org.jetbrains.kotlin.konan.target.KonanTarget

plugins {
    `kotlin-multiplatform`
    `android-library`
}

tasks.register("generateHeaders", CbindGenerate::class)

data class AndroidSetup(
    val abi: String,
    val konanTarget: KonanTarget,
    val crab: RustSetupResult = setupRustTask(konanTarget, project, Library.name),
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

val jvmSetups =
    listOf(
        KonanTarget.MACOS_ARM64,
        KonanTarget.MACOS_X64,
        KonanTarget.LINUX_ARM64,
        KonanTarget.LINUX_X64,
        KonanTarget.MINGW_X64
    ).map { setupRustTask(it, project, Library.name) }
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
    android {
        sourceSets.configureEach {
            jniLibs.srcDir(buildJniLibsDir)
        }
    }
}

