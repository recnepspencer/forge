import {
  bytesToHex,
  classifyResponsePrefix,
  readResponsePrefix,
} from "./wasm_magic_bytes.mjs";

/**
 * Attach Playwright response listeners that classify wasm and worker asset
 * responses independently of package error strings.
 */
export function attachNetworkResponseOracle(page) {
  const observations = [];
  const failedResponses = [];

  const onResponse = async (response) => {
    const url = response.url();
    const status = response.status();
    if (status >= 400) {
      failedResponses.push({
        url,
        status,
        contentType: response.headers()["content-type"] ?? null,
      });
    }
    const kind = classifyAssetUrl(url);
    if (kind === null) {
      return;
    }
    let prefixBytes = new Uint8Array();
    let prefixError = null;
    try {
      prefixBytes = await readResponsePrefix(response);
    } catch (error) {
      prefixError = error instanceof Error ? error.message : String(error);
    }
    observations.push({
      kind,
      url,
      status,
      contentType: response.headers()["content-type"] ?? null,
      prefixHex: bytesToHex(prefixBytes),
      prefixClass: classifyResponsePrefix(prefixBytes),
      prefixError,
      fromServiceWorker: response.fromServiceWorker(),
    });
  };

  page.on("response", (response) => {
    void onResponse(response);
  });

  return {
    snapshot() {
      return observations.map((entry) => ({ ...entry }));
    },
    failedSnapshot() {
      return failedResponses.map((entry) => ({ ...entry }));
    },
    reset() {
      observations.length = 0;
      failedResponses.length = 0;
    },
  };
}

export function classifyAssetUrl(url) {
  const normalized = url.toLowerCase();
  if (normalized.includes(".wasm")) {
    return "wasm";
  }
  if (normalized.includes("worker_runtime_bridge_worker")) {
    return "worker";
  }
  // Vite may emit hashed worker chunks that no longer contain the source name.
  if (
    normalized.includes("worker") &&
    (normalized.includes(".js") || normalized.includes(".mjs"))
  ) {
    return "workerCandidate";
  }
  return null;
}

export function summarizeAssetObservations(observations, kind) {
  const matched = observations.filter((entry) =>
    kind === "worker"
      ? entry.kind === "worker" || entry.kind === "workerCandidate"
      : entry.kind === kind
  );
  return {
    count: matched.length,
    urls: matched.map((entry) => entry.url),
    prefixClasses: matched.map((entry) => entry.prefixClass),
    statuses: matched.map((entry) => entry.status),
    contentTypes: matched.map((entry) => entry.contentType),
    entries: matched,
  };
}
