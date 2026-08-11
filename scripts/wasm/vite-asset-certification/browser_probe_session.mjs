import path from "node:path";
import { pathToFileURL } from "node:url";

import {
  attachNetworkResponseOracle,
  summarizeAssetObservations,
} from "./network_response_oracle.mjs";

export async function withPlaywrightPage(worldRoot, run) {
  const playwright = await importPlaywright(worldRoot);
  const browser = await playwright.chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    return await run(page);
  } finally {
    await browser.close();
  }
}

async function importPlaywright(worldRoot) {
  const candidates = [
    path.join(worldRoot, "node_modules", "playwright", "index.mjs"),
    path.join(worldRoot, "node_modules", "playwright", "index.js"),
    path.join(worldRoot, "node_modules", "playwright", "package.json"),
  ];
  for (const candidate of candidates) {
    try {
      if (candidate.endsWith("package.json")) {
        return await import(pathToFileURL(path.dirname(candidate)).href);
      }
      return await import(pathToFileURL(candidate).href);
    } catch {
      // try next resolution candidate
    }
  }
  throw new Error(
    `Unable to import playwright from consumer world at ${worldRoot}`,
  );
}

export async function probeDeploymentCell(options) {
  const {
    page,
    baseUrl,
    deployment,
    cellId,
    timeoutMs = 120_000,
  } = options;

  const network = attachNetworkResponseOracle(page);
  const targetUrl =
    `${baseUrl}/?deployment=${encodeURIComponent(deployment)}&cell=${encodeURIComponent(cellId)}`;

  const consoleMessages = [];
  const pageErrors = [];
  page.on("console", (message) => {
    consoleMessages.push({
      type: message.type(),
      text: message.text(),
    });
  });
  page.on("pageerror", (error) => {
    pageErrors.push(error instanceof Error ? error.message : String(error));
  });

  let waitError = null;
  try {
    await page.goto(targetUrl, { waitUntil: "domcontentloaded", timeout: timeoutMs });
    await page.waitForFunction(
      () => {
        const result = window.__WORTH_GATE0__;
        return (
          result &&
          (result.phase === "succeeded" || result.phase === "failed")
        );
      },
      null,
      { timeout: timeoutMs },
    );
  } catch (error) {
    waitError = error instanceof Error ? error.message : String(error);
  }

  const construction = await page.evaluate(() => window.__WORTH_GATE0__ ?? null);
  const pageStatusText = await page.evaluate(() => {
    const node = document.querySelector("#status");
    return node ? node.textContent : null;
  });
  const observations = network.snapshot();
  const failedResponses = network.failedSnapshot();
  const wasm = summarizeAssetObservations(observations, "wasm");
  const worker = summarizeAssetObservations(observations, "worker");
  const normalizedConstruction = construction ?? {
    phase: "failed",
    deployment,
    errorName: "Gate0ProbeTimeout",
    errorMessage: waitError ?? "probe did not publish a terminal phase",
    artifactFamily: null,
    moduleUrl: null,
  };

  return {
    cellId,
    deployment,
    targetUrl,
    construction: normalizedConstruction,
    pageStatusText,
    waitError,
    wasm,
    worker,
    failedResponses,
    prebundleEvidence: inspectPrebundleEvidence(
      normalizedConstruction,
      observations,
    ),
    consoleMessages,
    pageErrors,
    allAssetObservations: observations,
  };
}

function inspectPrebundleEvidence(construction, observations) {
  const moduleUrl =
    construction && typeof construction.moduleUrl === "string"
      ? construction.moduleUrl
      : "";
  const fromViteDeps = moduleUrl.includes("/.vite/deps/");
  const wasmFromViteDeps = observations.some(
    (entry) => entry.kind === "wasm" && entry.url.includes("/.vite/deps/"),
  );
  const wasmFromNodeModules = observations.some(
    (entry) =>
      entry.kind === "wasm" &&
      (entry.url.includes("/node_modules/worth-signals-wasm/") ||
        entry.url.includes("/node_modules/worth-signals-wasm%2F") ||
        entry.url.includes("worth_signal_wasm_bg.wasm")),
  );
  return {
    probeModuleUrl: moduleUrl || null,
    probeAppearsFromViteDeps: fromViteDeps,
    wasmRequestedFromViteDeps: wasmFromViteDeps,
    wasmRequestedWithPackagePath: wasmFromNodeModules,
  };
}
