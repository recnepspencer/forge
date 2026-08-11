/**
 * Proves panic=abort does not turn Result-shaped wasm-bindgen denials into traps.
 * Hits the raw WASM boundary (createRawSignals().read), not only JS admission.
 */
export function buildAbortSmokeSource(packageName) {
  return `import init, { createSignals } from "${packageName}";
import { createRawSignals } from "${packageName}/raw";

await init();

let rawDenialKind = null;
let rawDenialName = null;
let rawDenialMessage = null;
try {
  const raw = createRawSignals();
  raw.read("__track6_missing_signal__");
  rawDenialKind = "missing-denial";
} catch (error) {
  rawDenialKind = error instanceof WebAssembly.RuntimeError ? "trap" : "js-error";
  rawDenialName = error?.name ?? null;
  rawDenialMessage = String(error?.message ?? error);
}

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
let productDenialKind = null;
try {
  signals.read("__track6_missing_signal__");
  productDenialKind = "missing-denial";
} catch (error) {
  productDenialKind =
    error instanceof WebAssembly.RuntimeError ? "trap" : "js-error";
}

console.log(JSON.stringify({
  rawDenialKind,
  rawDenialName,
  rawDenialLooksLikeBoundaryError:
    typeof rawDenialMessage === "string" &&
    rawDenialMessage.length > 0 &&
    !(errorIsHtml(rawDenialMessage)),
  productDenialKind,
  stillReadable: (() => {
    const input = signals.input(1, { debugName: "track6AbortProbe" });
    return input() === 1;
  })(),
}));

function errorIsHtml(message) {
  return /<!doctype|<html/i.test(message);
}
`;
}
