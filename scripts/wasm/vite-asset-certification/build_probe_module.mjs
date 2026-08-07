/** Probe module source for Gate 0 consumer worlds. */

export function buildProbeModule(options = {}) {
  const assetsInjection = options.assetsInjection === true;
  if (assetsInjection) {
    return buildAssetsInjectionProbeModule();
  }
  return buildDefaultRelativeAssetProbeModule();
}

function buildDefaultRelativeAssetProbeModule() {
  return `${probePreamble()}
async function run() {
  const startedAt = performance.now();
  try {
    const options = deployment === "mainThreadCompatibility"
      ? { deployment: "mainThreadCompatibility" }
      : undefined;
    publish({
      phase: "createSignals-started",
      deployment,
      assetsInjection: false,
    });
    // Default Gate 0 cells deliberately omit assets injection.
    const signals = await createSignals(options);
    await finishSucceeded(signals, startedAt, { assetsInjection: false });
  } catch (error) {
    publishFailed(error, startedAt, { assetsInjection: false });
  }
}

void run();
`;
}

function buildAssetsInjectionProbeModule() {
  return `import { createSignals } from "worth-signals-wasm";
import { createReactSignalsStore } from "worth-signals-wasm/react";
import wasmUrl from "worth-signals-wasm/wasm?url";
import workerUrl from "worth-signals-wasm/worker?worker&url";

const statusNode = document.querySelector("#status");
const params = new URLSearchParams(location.search);
const deployment = params.get("deployment") === "mainThreadCompatibility"
  ? "mainThreadCompatibility"
  : "workerFirst";

window.__WORTH_GATE0__ = Object.freeze({
  phase: "module-loaded",
  deployment,
  assetsInjection: true,
});
if (statusNode) {
  statusNode.textContent = "module-loaded";
}

function publish(result) {
  window.__WORTH_GATE0__ = Object.freeze(result);
  if (statusNode) {
    statusNode.textContent = JSON.stringify(result, null, 2);
  }
}

async function finishSucceeded(signals, startedAt, extras) {
  const contract = typeof signals.contract === "function"
    ? signals.contract()
    : null;
  const store = createReactSignalsStore(signals);
  const input = signals.input(2, { debugName: "gate0.input" });
  const doubled = signals.computed(() => input() * 2, {
    debugName: "gate0.doubled",
  });
  const smoke = {
    input: input(),
    doubled: doubled(),
    storeSnapshot: store.getSignalSnapshot(doubled),
  };
  if (smoke.doubled !== 4 || smoke.storeSnapshot !== 4) {
    throw new Error(
      \`gate0 smoke expected doubled===4 and React store snapshot===4, got \${JSON.stringify(smoke)}\`,
    );
  }
  store.dispose();
  if (typeof signals.free === "function") {
    signals.free();
  }
  publish({
    phase: "succeeded",
    deployment,
    elapsedMs: performance.now() - startedAt,
    contract,
    smoke,
    reactAttached: true,
    moduleUrl: import.meta.url,
    assetUrls: {
      wasmUrl: String(wasmUrl),
      workerUrl: String(workerUrl),
    },
    ...extras,
  });
}

function publishFailed(error, startedAt, extras) {
  publish({
    phase: "failed",
    deployment,
    elapsedMs: performance.now() - startedAt,
    errorName: error && typeof error === "object" ? error.name ?? null : null,
    errorMessage: error instanceof Error ? error.message : String(error),
    artifactFamily:
      error && typeof error === "object" && "artifactFamily" in error
        ? error.artifactFamily
        : null,
    moduleUrl: import.meta.url,
    assetUrls: {
      wasmUrl: String(wasmUrl),
      workerUrl: String(workerUrl),
    },
    ...extras,
  });
}

async function run() {
  const startedAt = performance.now();
  try {
    const options = deployment === "mainThreadCompatibility"
      ? {
        deployment: "mainThreadCompatibility",
        assets: { wasmUrl },
      }
      : {
        assets: { wasmUrl, workerUrl },
      };
    publish({
      phase: "createSignals-started",
      deployment,
      assetsInjection: true,
      assetUrls: {
        wasmUrl: String(wasmUrl),
        workerUrl: String(workerUrl),
      },
    });
    const signals = await createSignals(options);
    await finishSucceeded(signals, startedAt, { assetsInjection: true });
  } catch (error) {
    publishFailed(error, startedAt, { assetsInjection: true });
  }
}

void run();
`;
}

function probePreamble() {
  return `import { createSignals } from "worth-signals-wasm";

const statusNode = document.querySelector("#status");
const params = new URLSearchParams(location.search);
const deployment = params.get("deployment") === "mainThreadCompatibility"
  ? "mainThreadCompatibility"
  : "workerFirst";

window.__WORTH_GATE0__ = Object.freeze({
  phase: "module-loaded",
  deployment,
});
if (statusNode) {
  statusNode.textContent = "module-loaded";
}

function publish(result) {
  window.__WORTH_GATE0__ = Object.freeze(result);
  if (statusNode) {
    statusNode.textContent = JSON.stringify(result, null, 2);
  }
}

async function finishSucceeded(signals, startedAt, extras) {
  const contract = typeof signals.contract === "function"
    ? signals.contract()
    : null;
  const input = signals.input(2, { debugName: "gate0.input" });
  const doubled = signals.computed(() => input() * 2, {
    debugName: "gate0.doubled",
  });
  const smoke = { input: input(), doubled: doubled() };
  if (smoke.doubled !== 4) {
    throw new Error(\`gate0 smoke expected doubled===4, got \${smoke.doubled}\`);
  }
  if (typeof signals.free === "function") {
    signals.free();
  }
  publish({
    phase: "succeeded",
    deployment,
    elapsedMs: performance.now() - startedAt,
    contract,
    smoke,
    moduleUrl: import.meta.url,
    ...extras,
  });
}

function publishFailed(error, startedAt, extras) {
  publish({
    phase: "failed",
    deployment,
    elapsedMs: performance.now() - startedAt,
    errorName: error && typeof error === "object" ? error.name ?? null : null,
    errorMessage: error instanceof Error ? error.message : String(error),
    artifactFamily:
      error && typeof error === "object" && "artifactFamily" in error
        ? error.artifactFamily
        : null,
    moduleUrl: import.meta.url,
    ...extras,
  });
}
`;
}
