@file:OptIn(ExperimentalKotlinGradlePluginApi::class)

import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.KotlinMultiplatform
import com.vanniktech.maven.publish.SonatypeHost
import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.konan.target.Family

plugins {
    `crab-multiplatform`
    `maven-publish`
    alias(libs.plugins.dokka)
    alias(libs.plugins.mavenPublisher)
}

repositories {
    google()
    mavenCentral()
}

crabAndroid {
    libName = Library.name
    cross = true
}

crabJvm {
    libName = Library.name
    cross = true
}

kotlin {
    applyDefaultHierarchyTemplate {
        common {
            group("jvmCommon") {
                withJvm()
                withAndroidTarget()
            }
        }
    }

    androidTarget {
        publishLibraryVariants("release")
        compilations.all {
            compileTaskProvider.configure {
                compilerOptions {
                    jvmTarget.set(JvmTarget.JVM_1_8)
                }
            }
        }
    }

    jvm {
        compilations.all {
            kotlinOptions.jvmTarget = "11"
        }
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
    }

    listOf(
        linuxX64(),
        linuxArm64(),
        mingwX64(),
        macosX64(),
        macosArm64(),
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        it.crabNative {
            libName = Library.name
            cross = it.konanTarget.family != Family.IOS && it.konanTarget.family != Family.OSX
        }
        it.binaries {
            sharedLib()
            staticLib()
        }
    }

    compilerOptions {
        freeCompilerArgs.add("-Xexpect-actual-classes")
        allWarningsAsErrors = true
    }

    sourceSets {
        val commonMain by getting { }
        val commonTest by getting {
            dependencies {
                implementation(libs.kotlin.test)
                implementation(libs.ktor.client.core)
                implementation(libs.kotlinx.io.core)
            }
        }
        val jvmMain by getting {

        }
        val jvmTest by getting {
            dependencies {
                implementation(libs.kotest.runner.junit5)
            }
        }
        val jvmCommonTest by getting {
            dependencies {
                implementation(libs.ktor.client.cio)
            }
        }
        val nativeMain by getting {
        }
        val appleTest by getting {
            dependencies {
                implementation(libs.ktor.client.darwin)
            }
        }

        val androidInstrumentedTest by getting {
            dependencies {
                implementation(libs.junit)
                implementation(libs.bundles.androidx.test)
            }
        }
    }
}

android {
    namespace = Library.namespace
    compileSdk = 34

    defaultConfig {
        minSdk = 27
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }
}

tasks.dokkaJavadoc {
    enabled = false
}

mavenPublishing {
    configure(
        KotlinMultiplatform(
            javadocJar = JavadocJar.Dokka("dokkaHtml"),
            sourcesJar = true,
            androidVariantsToPublish = listOf("release")
        )
    )

    publishToMavenCentral(SonatypeHost.CENTRAL_PORTAL)
    signAllPublications()

    coordinates(Library.namespace, "core", "0.1.0")
    pom {
        name = "Hugging Face Tokenizers KMP"
        description = "Kotlin binding to the Hugging Face tokenizers, as a Multiplatform library."
        url = "https://github.com/zhufucdev/huggingface-tokenizers-kmp"

        licenses {
            license {
                name = "The Apache License, Version 2.0"
                url = "http://www.apache.org/licenses/LICENSE-2.0.txt"
            }
        }
        developers {
            developer {
                name = "Steve Reed"
                email = "zhufuzhufu1@gmail.com"
                id = "zhufucdev"
            }
        }
        scm {
            url = "https://github.com/zhufucdev/huggingface-tokenizers-kmp"
            connection = "scm:git:git://github.com/zhufucdev/huggingface-tokenizers-kmp.git"
            developerConnection = "scm:git:ssh://github.com/zhufucdev/huggingface-tokenizers-kmp.git"
        }
    }
}
