# Hugging Face Tokenizers KMP

Kotlin Multiplatform binding of 🤗 Tokenizers.

![android](https://camo.githubusercontent.com/6d3fa15a839b018358f823f7439bedd68ac5d97120a9f581e746d8a81e43ba86/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d616e64726f69642d3645444238442e7376673f7374796c653d666c6174)
![ios](https://camo.githubusercontent.com/84f060f08f4b60116d3f2740fa7fc7127fa990a9dc4c200aea08a5963d97b94f/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d696f732d4344434443442e7376673f7374796c653d666c6174)
![jvm](https://camo.githubusercontent.com/48140edbc0bf4c032d4776ec8ea77d25b7970d3611a36a9926488205eeffda45/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d6a766d2d4442343133442e7376673f7374796c653d666c6174)
![linux](https://camo.githubusercontent.com/c2b504b566c86ee9076173e463766aac0fb1bab07a0c7f1aad6f2edc57027fdc/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d6c696e75782d3244334636432e7376673f7374796c653d666c6174)
![macos](https://camo.githubusercontent.com/9fe6c59ec72739ee7041ec5808fe8a48b538cc18c3fa7906c8b54205c966f3ea/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d6d61636f732d3131313131312e7376673f7374796c653d666c6174)
![windows](https://camo.githubusercontent.com/7981e11c5f07de2906c827d4d659894344ca6d3e0fd911c958cd607c40c02772/687474703a2f2f696d672e736869656c64732e696f2f62616467652f2d77696e646f77732d3444373643442e7376673f7374796c653d666c6174)

> [!IMPORTANT]
> This library is currently under development.
> Partiality of the official API is prioritized for personal reasons.
> Contribution is welcome if you feel like something important is missing.

## Usage

Whenever maven central is available, following Gradle configuration
can be utilized for importing this library.

**build.gradle.kts**
```kotlin
kotlin {
    sourceSets {
        commonMain.dependencies {
            implementation(libs.hgtk.core)
        }
    }
}
```

**libs.versions.toml**
```toml
[versions]
hgtk = "0.0.4"

[libraries]
hgtk-core = { module = "com.zhufucdev.hgtk:core", version.ref = "hgtk" }
```
