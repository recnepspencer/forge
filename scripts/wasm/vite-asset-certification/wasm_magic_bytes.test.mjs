import assert from "node:assert/strict";
import test from "node:test";

import {
  HTML_DOCTYPE_PREFIX,
  WASM_MAGIC,
  bytesToHex,
  classifyResponsePrefix,
} from "./wasm_magic_bytes.mjs";

test("classifies WASM magic independently of package error strings", () => {
  assert.equal(classifyResponsePrefix(WASM_MAGIC), "wasmMagic");
  assert.equal(bytesToHex(WASM_MAGIC), "00 61 73 6d");
});

test("classifies the SPA HTML fingerprint that masquerades as WASM", () => {
  assert.equal(classifyResponsePrefix(HTML_DOCTYPE_PREFIX), "htmlDoctype");
  assert.equal(bytesToHex(HTML_DOCTYPE_PREFIX), "3c 21 64 6f");
  assert.equal(
    classifyResponsePrefix(Uint8Array.from([0x3c, 0x68, 0x74, 0x6d])),
    "htmlLike",
  );
});
