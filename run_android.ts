import { $ } from "https://deno.land/x/dax@0.34.0/mod.ts";

const E = new TextEncoder();
const D = new TextDecoder();

if (import.meta.main) {
  await buildTermuxAll();
  await pushBinToAndroid();
  await adbRoot();

  const shell = adbShell();
  const stdin = shell.stdin.getWriter();
  const stdout = shell.stdout.getReader();

  await startTermux(stdin);

  write(stdin, "cp /sdcard/termux-all .");
  write(stdin, "chmod +x termux-all");
  write(stdin, "./termux-all");
  const output = await stdout.read().then((r) =>
    JSON.parse(D.decode(r.value!))
  );
  console.log(output);
}

async function startTermux(stdin: WritableStreamDefaultWriter<Uint8Array>) {
  await write(
    stdin,
    `export PREFIX='/data/data/com.termux/files/usr' HOME='/data/data/com.termux/files/home' LD_LIBRARY_PATH='/data/data/com.termux/files/usr/lib' PATH="/data/data/com.termux/files/usr/bin:/data/data/com.termux/files/usr/bin/applets:$PATH" LANG='en_US.UTF-8' SHELL='/data/data/com.termux/files/usr/bin/bash'`,
  );
  await write(stdin, `cd "$HOME" && exec "$SHELL" -l`);
}

function adbShell() {
  return new Deno.Command("adb", {
    args: ["shell"],
    stdin: "piped",
    stdout: "piped",
  }).spawn();
}

async function adbRoot() {
  await new Deno.Command("adb", {
    args: ["root"],
    stdout: "null",
    stderr: "null",
  }).spawn().status;
}

async function write(
  stdin: WritableStreamDefaultWriter<Uint8Array>,
  cmd: string,
) {
  await stdin.write(E.encode(cmd + "\n"));
}

async function buildTermuxAll() {
  if (await $.which("cargo-ndk") === undefined) {
    throw "test requires cargo-ndk to be installed";
  }
  await $`cargo ndk -t x86_64  build --release --example termux-all`;
}

async function pushBinToAndroid() {
  const cargoTargetDir = Deno.env.get("CARGO_TARGET_DIR") || "target";
  let archDir = "";
  switch (Deno.env.get("ARCH") ?? "x86_64") {
    case "x86_64":
      archDir = "x86_64-linux-android";
      break;
    case "armeabi-v7a":
      archDir = "armv7-linux-androideabi";
      break;
    case "arm64-v8a":
      archDir = "aarch64-linux-android";
      break;
  }
  await $`adb push ${cargoTargetDir}/${archDir}/release/examples/termux-all /sdcard/termux-all`;
}
