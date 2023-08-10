import Context from "https://deno.land/std@0.190.0/wasi/snapshot_preview1.ts";
import { wasmData } from "./termux_wasm.js";
import { base64 } from "./deps.ts";

interface Exports {
  memory: WebAssembly.Memory;
  // deno-lint-ignore no-explicit-any
  battery_status: () => any;
}

const context = new Context({});

const { instance } = await WebAssembly.instantiate(
  base64.decode(wasmData),
  {
    "wasi_snapshot_preview1": context.exports,
  },
);

const exports = instance.exports as unknown as Exports;

const memory = new Uint8Array(exports.memory.buffer);

export function battery_status() {
  const ptr = exports.battery_status();
  if (ptr === 0) throw "battery_status failed";
  const nul = memory.slice(ptr).findIndex((b) => b === 0);
  return JSON.parse(
    new TextDecoder().decode(
      memory.slice(ptr, ptr + nul),
    ),
  );
}

if (import.meta.main) {
  console.log(battery_status());
}
