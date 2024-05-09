$JNI_LIBS="./target/jni-libs"
$LIB_NAME="libmain.so"
$SDL2_LIBS="SDL2_Source/SDL-release-2.28.0/libs"
$BUILD_MODE="release"

# copy sdl2 libs into rusts build dir
New-Item -Force -ItemType directory -Path ./target/aarch64-linux-android/$BUILD_MODE/deps/
New-Item -Force -ItemType directory -Path ./target/armv7-linux-androideabi/$BUILD_MODE/deps/
New-Item -Force -ItemType directory -Path ./target/i686-linux-android/$BUILD_MODE/deps/
Copy-item -Force -Recurse $SDL2_LIBS/arm64-v8a/. -Destination target/aarch64-linux-android/$BUILD_MODE/deps/
Copy-item -Force -Recurse $SDL2_LIBS/armeabi-v7a/. -Destination target/armv7-linux-androideabi/$BUILD_MODE/deps/
Copy-item -Force -Recurse $SDL2_LIBS/x86/. -Destination ./target/i686-linux-android/$BUILD_MODE/deps/

rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android

#build the libraries
cargo +nightly-x86_64-pc-windows-gnu build -Zbuild-std --target aarch64-linux-android-custom.json --$BUILD_MODE
pause
cargo +nightly build --target armv7-linux-androideabi-custom.json --$BUILD_MODE
cargo +nightly build --target i686-linux-android-custom.json --$BUILD_MODE

#prepare folders...
Remove-Item -Force -Recurse -Path $JNI_LIBS
New-Item -Force -ItemType directory -Path $JNI_LIBS
New-Item -Force -ItemType directory -Path $JNI_LIBS/arm64-v8a
New-Item -Force -ItemType directory -Path $JNI_LIBS/armeabi-v7a
New-Item -Force -ItemType directory -Path $JNI_LIBS/x86

#..and copy the rust library into the android studio project, ready for beeing included into the APK
Copy-item -Force -Recurse target/aarch64-linux-android/$BUILD_MODE/$LIB_NAME -Destination $JNI_LIBS/arm64-v8a/libmain.so
Copy-item -Force -Recurse target/armv7-linux-androideabi/$BUILD_MODE/$LIB_NAME -Destination $JNI_LIBS/armeabi-v7a/libmain.so
Copy-item -Force -Recurse target/i686-linux-android/$BUILD_MODE/$LIB_NAME -Destination $JNI_LIBS/x86/libmain.so