plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.antigravity.manager"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.antigravity.manager"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    // Zero dipendenze terze parti: usa solo le API native di Android SDK
    // per mantenere il progetto leggerissimo e semplice da compilare nel tempo.
}
