# Termux Api

Rust library for termux apis (+ exposed WASI api).

## Usage

```rs
use termux_api_rs::*;

fn main() {
    dbg!(battery_status().unwrap());
}
```

To compile for android, the easiest way is to use
https://github.com/bbqsrc/cargo-ndk

## Wasi

Wasi bindings are provided, deno wrapper that consumes those bindings is also
provided.

To update the deno bindings run `deno run -A deno_api/build.ts`

## Drive Test

Run `deno run -A run_android.ts` to test the example termux-all on andorid

It requires an android phone/emulator setup with termux/termux-api installed +
cargo-ndk

This script does the following:

- compile termux-all example, it defaults to x86_64 arch, this can be changed
  with ARCH env variable, values are the same as cargo-ndk values
- push it to android
- activate adb root (needed for the next steps)
- run termux inside adb shell
- run termux-all and show the output (ctrlc to end the program)

## TODO

Implement missing apis, references: https://wiki.termux.com/wiki/Termux:API
https://github.com/termux/termux-api/tree/master/app/src/main/java/com/termux/api/apis
