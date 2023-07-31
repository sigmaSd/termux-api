import Context from "https://deno.land/std@0.190.0/wasi/snapshot_preview1.ts";

interface Exports {
  memory: WebAssembly.Memory;
  // deno-lint-ignore no-explicit-any
  battery_status: () => any;
}

const context = new Context({});

// TODO: figure something nicer for the end user
//       maybe inlie the wasm file, or use the js bindings
//       so the user don't need to have another file
const cargoTargetDir = Deno.env.get("CARGO_TARGET_DIR") || "target";
const { instance } = await WebAssembly.instantiate(
  await Deno.readFile(
    cargoTargetDir + "/wasm32-wasi/release/termux_api.wasm",
  ),
  {
    "wasi_snapshot_preview1": context.exports,
  },
);

const exports = instance.exports as unknown as Exports;

const memory = new Uint8Array(exports.memory.buffer);

function battery_status() {
  const ptr = exports.battery_status();
  if (ptr === 0) throw "battery_status failed";
  const nul = memory.slice(ptr).findIndex((b) => b === 0);
  return JSON.parse(
    new TextDecoder().decode(
      memory.slice(ptr, ptr + nul),
    ),
  );
}

console.log(battery_status());
