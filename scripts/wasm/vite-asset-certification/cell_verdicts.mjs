/**
 * Gate 0 verdicts are measurements, not product pass/fail gates for CI greenness.
 * Each cell records whether the measured world satisfied the asset-correctness claim.
 */

export function verdictForDefaultAssetCell(probe) {
  const constructionSucceeded = probe.construction?.phase === "succeeded";
  const wasmClasses = new Set(probe.wasm.prefixClasses);
  const sawWasmMagic = wasmClasses.has("wasmMagic");
  const sawHtml = wasmClasses.has("htmlDoctype") || wasmClasses.has("htmlLike");
  const workerNeeded = probe.deployment === "workerFirst";
  const assetsInjection = probe.world?.assetsInjection === true;
  const workerJsLooksHealthy = !workerNeeded
    ? true
    : probe.worker.entries.some(
      (entry) =>
        entry.status >= 200 &&
        entry.status < 400 &&
        entry.prefixClass !== "htmlDoctype" &&
        entry.prefixClass !== "htmlLike",
    );

  let claim = assetsInjection
    ? "createSignalsAssetsInjectionLoadsUnderVitePrebundle"
    : "defaultRelativeAssetsLoadUnderVitePrebundle";
  let status = "failed";
  let reason = null;

  const failedResponses = probe.failedResponses ?? [];
  const sawForbidden = failedResponses.some((entry) => entry.status === 403);

  if (probe.wasm.count === 0 && sawForbidden) {
    status = "failed";
    reason = "devServerReturnedHttp403BeforeWasmFetch";
  } else if (probe.wasm.count === 0) {
    status = "inconclusive";
    reason = "noWasmNetworkResponsesObserved";
  } else if (sawHtml && !sawWasmMagic) {
    status = "failed";
    reason = "wasmResponseWasHtml";
  } else if (sawWasmMagic && constructionSucceeded && workerJsLooksHealthy) {
    status = "passed";
    reason = "wasmMagicAndConstructionSucceeded";
  } else if (sawWasmMagic && !constructionSucceeded) {
    status = "failed";
    reason = "wasmMagicButConstructionFailed";
  } else if (!sawWasmMagic && constructionSucceeded) {
    status = "inconclusive";
    reason = "constructionSucceededWithoutObservedWasmMagic";
  } else {
    status = "failed";
    reason = "constructionFailedWithoutClearWasmMagic";
  }

  if (workerNeeded && probe.worker.count === 0 && status === "passed") {
    status = "inconclusive";
    reason = "workerFirstPassedButNoWorkerNetworkObservation";
  }

  if (
    workerNeeded &&
    probe.worker.entries.some((entry) => entry.status === 404) &&
    status !== "passed"
  ) {
    status = "failed";
    reason = "workerScriptMissingBesidePrebundle";
  }

  const prebundleCache = probe.prebundleCache;
  const prebundleConfirmed = Boolean(
    prebundleCache?.present &&
      (prebundleCache.metadataMentionsPackage ||
        prebundleCache.packageRelatedEntries?.length > 0),
  );
  if (status === "passed" && probe.mode === "vite-dev" && !prebundleConfirmed) {
    status = "inconclusive";
    reason = "passedButVitePrebundleCacheNotConfirmed";
  }

  return {
    claim,
    status,
    reason,
    constructionPhase: probe.construction?.phase ?? null,
    constructionError: probe.construction?.errorMessage ?? null,
    wasmPrefixClasses: [...wasmClasses],
    workerPrefixClasses: [...new Set(probe.worker.prefixClasses)],
    prebundleEvidence: probe.prebundleEvidence,
    prebundleCache: prebundleCache ?? null,
    prebundleConfirmed,
  };
}

export function verdictForSpaFallbackCell(probe) {
  const wasmClasses = new Set(probe.wasm.prefixClasses);
  const directClass = probe.directWasmProbe?.prefixClass ?? null;
  const sawHtml =
    wasmClasses.has("htmlDoctype") ||
    wasmClasses.has("htmlLike") ||
    directClass === "htmlDoctype" ||
    directClass === "htmlLike";
  const constructionFailed = probe.construction?.phase === "failed";

  let status = "failed";
  let reason = null;
  const packageExplainsFailure =
    typeof probe.construction?.errorMessage === "string" &&
    /worth-signals-wasm: expected WASM bytes/u.test(
      probe.construction.errorMessage,
    ) &&
    /received HTML/u.test(probe.construction.errorMessage);

  if (sawHtml && constructionFailed && packageExplainsFailure) {
    status = "passed";
    reason = "harnessDetectedHtmlWasmAndPackageDiagnostic";
  } else if (sawHtml && constructionFailed && !packageExplainsFailure) {
    status = "failed";
    reason = "htmlWasmFailedWithoutPackageDiagnostic";
  } else if (sawHtml && !constructionFailed) {
    status = "failed";
    reason = "htmlWasmObservedButConstructionDidNotFail";
  } else if (!sawHtml) {
    status = "failed";
    reason = "spaFallbackDidNotServeHtmlForWasm";
  }

  return {
    claim: "spaHtmlFallbackIsObservableAsNonWasmMagic",
    status,
    reason,
    constructionPhase: probe.construction?.phase ?? null,
    constructionError: probe.construction?.errorMessage ?? null,
    wasmPrefixClasses: [...wasmClasses],
    directWasmProbe: probe.directWasmProbe ?? null,
    packageExplainsFailure,
  };
}

export function summarizeGateDecision(cellResults) {
  const vite8Dev = cellResults.filter((cell) =>
    cell.cellId.startsWith("vite8-dev-") &&
    !cell.cellId.includes("harness-failure")
  );
  const vite7Dev = cellResults.filter((cell) =>
    cell.cellId.startsWith("vite7-dev-") &&
    !cell.cellId.includes("harness-failure") &&
    !cell.cellId.startsWith("vite7-assets-")
  );
  const vite7Assets = cellResults.filter((cell) =>
    cell.cellId.startsWith("vite7-assets-") &&
    !cell.cellId.includes("harness-failure")
  );
  const preview = cellResults.filter((cell) =>
    cell.cellId.startsWith("vite8-preview-")
  );
  const spa = cellResults.filter((cell) =>
    cell.cellId.startsWith("spa-fallback-")
  );

  const defaultPass = vite8Dev.length > 0 &&
    vite8Dev.every((cell) => cell.verdict.status === "passed");
  const defaultFail = vite8Dev.some((cell) => cell.verdict.status === "failed");
  const vite7Broken = vite7Dev.some((cell) =>
    cell.verdict.reason === "wasmResponseWasHtml" ||
    cell.verdict.reason === "workerScriptMissingBesidePrebundle"
  );
  const vite7AssetsPass = vite7Assets.length > 0 &&
    vite7Assets.every((cell) => cell.verdict.status === "passed");
  const vite7AssetsFail = vite7Assets.some((cell) =>
    cell.verdict.status === "failed"
  );

  let recommendation = "insufficientEvidence";
  if (defaultPass) {
    if (vite7Broken && vite7AssetsPass) {
      recommendation =
        "vite8DefaultRelativeAssetsAppearViable_withWorkerFormatEsAndFsAllow; vite7DefaultStillBroken; createSignalsAssetsInjectionProvenOnVite7";
    } else if (vite7Broken && vite7AssetsFail) {
      recommendation =
        "vite8DefaultRelativeAssetsAppearViable; vite7DefaultBrokenAndAssetsInjectionAlsoFailed";
    } else if (vite7Broken) {
      recommendation =
        "vite8DefaultRelativeAssetsAppearViable_withWorkerFormatEsAndFsAllow; vite7StillBroken_soAssetsApiRemainsRequiredForOlderVite; keepAssetsApiAsPortableAdvancedPath";
    } else {
      recommendation =
        "vite8DefaultRelativeAssetsAppearViable; keep assets API as portableAdvancedPath";
    }
  } else if (defaultFail) {
    recommendation =
      "vite8DefaultRelativeAssetsStillBroken; require assets injection or package vite plugin for Vite consumers";
  }

  return {
    recommendation,
    vite8DevDefaultCells: summarizeCells(vite8Dev),
    vite8PreviewCells: summarizeCells(preview),
    vite7DevCells: summarizeCells(vite7Dev),
    vite7AssetsCells: summarizeCells(vite7Assets),
    spaFallbackCells: summarizeCells(spa),
    notes: [
      "Vite 8 optimizer rebases wasm URLs out of .vite/deps toward the package files.",
      "Consumer vite configs need worker.format='es' because the worker uses top-level await.",
      "Vite 7 default relative URLs still request wasm beside .vite/deps and receive SPA HTML (3c 21 64 6f).",
      "Vite 7 + createSignals({ assets }) with worth-signals-wasm/wasm?url and worker?worker&url is the portable repair path.",
    ],
  };
}

function summarizeCells(cells) {
  return cells.map((cell) => ({
    cellId: cell.cellId,
    status: cell.verdict.status,
    reason: cell.verdict.reason,
  }));
}
