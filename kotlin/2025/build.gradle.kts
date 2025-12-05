plugins {
    kotlin("jvm") version "2.2.21"
    id("java-library")
}

group = "com.jcjolley"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

val quarkusBomVersion = "3.29.4"
val quarkusAwsBomVersion = "3.12.1"
val mockkVersion = "1.14.2"
val striktVersion = "0.35.1"
val junit5Version = "6.0.1"
val t0KotlinUtilsVersion = "0.8.0"

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.10.2")
    testImplementation("io.mockk:mockk:$mockkVersion")
    testImplementation("io.strikt:strikt-core:$striktVersion")
    testImplementation("io.strikt:strikt-mockk:$striktVersion")
    testImplementation("org.awaitility:awaitility-kotlin:4.3.0")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.10.2")
    testImplementation("org.junit.jupiter:junit-jupiter-api:$junit5Version")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher:$junit5Version")
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        showStandardStreams = true
    }
}
