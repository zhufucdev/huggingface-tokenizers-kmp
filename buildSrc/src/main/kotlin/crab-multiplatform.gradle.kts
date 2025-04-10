plugins {
    `kotlin-multiplatform`
    `android-library`
}

tasks.register("generateHeaders", CbindGenerate::class)
