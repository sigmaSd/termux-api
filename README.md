# Termux Api

Rust library for termux apis (+ exposed WASI api).

## Drive Test

Run `deno run -A run_android.ts` to test the example termux-all on andorid, this script does the following:
- compile termux-all example
- push it to android
- activate adb root (needed for the next steps)
- run termux inside adb shell
- run termux-all and show the output (ctrlc to end the program)

It requires an android phone/emulator setup with termux/termux-api installed

## TODO

Implement missing apis, references: https://wiki.termux.com/wiki/Termux:API https://github.com/termux/termux-api/tree/master/app/src/main/java/com/termux/api/apis
