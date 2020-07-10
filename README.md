# Example project to run rust game loop on android
This is an attempt to run a game loop on android, implemented in rust:

* Android
* Rust
* OpenGL
* Event handling, but running game loop

This project is an extension of given repos:
* https://github.com/katyo/glutin-simple - Katyo's example fork of android example project
* https://github.com/rust-windowing/winit - Winit master with some changes
* https://github.com/katyo/glutin/tree/android-support - Katyo's glutin branch 'android-support', commit 'Fixed compiling for Android'

## How to compile?
The project compiles for all 4 android targets:
* armv7-linux-androideabi
* aarch64-linux-android
* i686-linux-android
* x86_64-linux-android

Install rust and its android toolchains, explained here:
https://github.com/rust-windowing/android-rs-glue

__DO NOT:__
`cargo install cargo-apk`

__INSTEAD:__
`cargo install --git https://github.com/rust-windowing/android-ndk-rs cargo-apk`

## Why is this working?
The basic issues are https://github.com/rust-windowing/glutin/issues/1307 and https://github.com/rust-windowing/glutin/pull/1313

@dvc94ch mentioned that creating windows with glutin on android is only available between resume and suspend. @katyo did the first big step by implementing needed methods to glutin. I just did a hacky add of winit events to "resume" and "suspend". By now, CreateWindows and Resume both create the "resumed" events. Therefore this app is checking for native window being available before trying to create the context. Not pretty nice, but currently working.

Additionally, there is some bug with sRGB pixel format on android. This means we need to adjust the glutin's ContextBuilder:
```
let context = ContextBuilder::new()
  .with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)))
  .with_gl_debug_flag(false)
  .with_srgb(false)  // <--- Settings to true makes the app crash with 'NoAvailablePixelFormat'
  .with_vsync(true)  // <--- Settings to false makes the app crash with 'NoAvailablePixelFormat'
  .build_windowed(wb, &el)
  .unwrap();
```

## What is better than other projects?
This project compiles, starts without crashing and generates an OpenGL context für up-to-date glutin and winit crates. To be honest, i was not able to find any other running project. In terms of GameDev, the winit events `resumed` and `suspended` are analog to `OnCreateDevice` and `OnDestroyDevice`. 
Sure this means that all graphics resources must be destroyed when the app is send to background, and recreated if resumed, but this should be the way to do it (this is the way we were told at university). 
Many apps try to workaround this to improve user experience (faster resume), but it's more a question of design. There shouldn't be any need to hold complete graphics resources in memory if the app is not drawing while in background.

You can just clone this project and run it. No internal crate dependencies or other stuff that makes someone go crazy.
