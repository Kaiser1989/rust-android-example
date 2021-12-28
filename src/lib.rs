#![cfg(target_os = "android")]

//////////////////////////////////////////////////
// Using

pub mod runner;

//////////////////////////////////////////////////
// Entry point for android

#[ndk_glue::main(backtrace = "on")]
fn main() {
    runner::start();
}
