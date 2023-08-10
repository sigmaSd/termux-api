import { base64 } from "../deps.ts";

await new Deno.Command("cargo", {
  args: ["build", "--release", "--target", "wasm32-wasi"],
}).spawn().status;

const cargoTargetDir = Deno.env.get("CARGO_TARGET_DIR") || "target";
const data = Deno.readFileSync(
  `${cargoTargetDir}/wasm32-wasi/release/termux_api_rs.wasm`,
);

Deno.writeTextFileSync(
  "./deno_api/termux_wasm.js",
  `export const wasmData = "${base64.encode(data)}";`,
);

await new Deno.Command("deno", { args: ["fmt", "./deno_api/termux_wasm.js"] })
  .spawn().status;
