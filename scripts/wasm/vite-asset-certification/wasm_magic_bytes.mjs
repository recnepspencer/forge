/** Independent oracles for WASM / HTML response bodies. */

export const WASM_MAGIC = Object.freeze([0x00, 0x61, 0x73, 0x6d]); // \0asm
export const HTML_DOCTYPE_PREFIX = Object.freeze([0x3c, 0x21, 0x64, 0x6f]); // <!do

export function bytesToHex(bytes) {
  return [...bytes]
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join(" ");
}

export function classifyResponsePrefix(prefixBytes) {
  if (prefixMatches(prefixBytes, WASM_MAGIC)) {
    return "wasmMagic";
  }
  if (prefixMatches(prefixBytes, HTML_DOCTYPE_PREFIX)) {
    return "htmlDoctype";
  }
  if (prefixBytes.length >= 1 && prefixBytes[0] === 0x3c) {
    return "htmlLike";
  }
  if (prefixBytes.length === 0) {
    return "empty";
  }
  return "other";
}

export function prefixMatches(actual, expected) {
  if (actual.length < expected.length) {
    return false;
  }
  return expected.every((byte, index) => actual[index] === byte);
}

export async function readResponsePrefix(response, length = 4) {
  const buffer = await response.body();
  if (!buffer) {
    return new Uint8Array();
  }
  return new Uint8Array(buffer.slice(0, length));
}
