### Compiling for flutter
_Todo: write a script to handle this automatically._ 

I wrote out these steps so I can remember them, but here's how I build (on windows) for flutter, based mostly on [this](https://medium.com/flutter-community/finally-running-rust-natively-on-a-flutter-plugin-here-is-how-6f2826eb1735).

### Android
Install the Android NDK in Android Studio's SDK Manager (under SDK Tools tab).

Set the environment variable `ANDROID_NDK_HOME` to the NDK root folder, which should contain `ndk-build.cmd` and `ndk-gdb.cmd`. For me this was located in `Android/Sdk/ndk/21.3.6528147/`.

Install cbindgen
```
cargo install cbindgen
```

In the rust directory, add the following rust targets for android:
```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

Next, we need to run cargo build for each target, pointing to the correct Android NDK linkers. On Windows, I had to use Git Bash or some equivalent to execute the following bash statements. There's probably a simpler way without needing bash, but this is how I got it to work. Make sure to test the following paths, ensuring they point to the these files. 
```
AARCH64_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android26-clang.cmd

ARMV7_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/windows-x86_64/bin/armv7a-linux-androideabi26-clang.cmd

I686_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/windows-x86_64/bin/i686-linux-android26-clang.cmd
```

Then build the targets with these `cargo build` commands.
```
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$AARCH64_LINKER cargo build --target aarch64-linux-android --release

CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ARMV7_LINKER cargo build --target armv7-linux-androideabi --release

CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$I686_LINKER cargo build --target i686-linux-android --release
```

This compiles the following shared files for each target.
```
target/aarch64-linux-android/release/libcalc_2.so
target/armv7-linux-androideabi/release/libcalc_2.so
target/i686-linux-android/release/libcalc_2.so
```

Finally in the **flutter_calc** project directory, copy into each of the following 3 directories the specified `libcalc_2.so` file from the rust project's target folder.
```
android/app/src/main
└── jniLibs
    ├── arm64-v8a
    │   └── libcalc_2.so -> rust/target/aarch64-linux-android/release/libcalc_2.so
    ├── armeabi-v7a
    │   └── libcalc_2.so -> rust/target/armv7-linux-androideabi/release/libcalc_2.so
    └── x86
        └── libcalc_2.so -> rust/target/i686-linux-android/release/libcalc_2.so
```
To avoid copying these files after each rebuild, create symbolic links at the 3 locations so that the flutter files automatically point to the current rust builds.

The exported Rust functions should now be able to be called within Flutter as a dynamically linked library using [Flutter's FFI](https://flutter.dev/docs/development/platform-integration/c-interop).

### iOS

Todo