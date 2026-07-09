import { readResourceLineHandle } from "../sources/form_sources.js";
import { stableValueDigest } from "../values/value_paths.js";
import { readFormStateSnapshot } from "../recovery/form_state_snapshot.js";

export function createFormReplayRestoreStore() {
  let nextReplayRestoreId = 1;
  const history = [];
  return Object.freeze({
    replayExactResourceSource(context, options = {}) {
      return executeReplayRestore(
        "resourceReplayExact",
        context,
        options,
        "replayExact",
        (historyRead) => historyRead.replayExact(),
        (result) => result.kind === "unavailable"
          ? unavailableResult(result)
          : Object.freeze({
            kind: "replayed",
            mode: result.mode,
            signalId: result.signalId,
            basisCurrentId: result.basisCurrentId,
            basisAdvanceCount: result.basisAdvanceCount,
            reloadStatus: result.reloadStatus,
            digest: stableValueDigest(result),
          }),
        (resourceResult) => resourceResult.kind === "unavailable"
          ? options.reason ?? resourceResult.detail
          : options.reason ?? "resource line source replayed exact runtime history",
      );
    },
    restoreExactResourceSource(context, options = {}) {
      return executeReplayRestore(
        "resourceRestoreExact",
        context,
        options,
        "restoreExact",
        (historyRead) => historyRead.restoreExact(),
        (result) => result.kind === "unavailable"
          ? unavailableResult(result)
          : Object.freeze({
            kind: "restored",
            mode: result.mode,
            branchId: result.branchId,
            snapshotId: result.snapshotId,
            basisCurrentId: result.basisCurrentId,
            basisAdvanceCount: result.basisAdvanceCount,
            reloadStatus: result.reloadStatus,
            digest: stableValueDigest(result),
          }),
        (resourceResult) => resourceResult.kind === "unavailable"
          ? options.reason ?? resourceResult.detail
          : options.reason ?? "resource line source restored exact branch history",
      );
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function executeReplayRestore(mode, context, options, requiredMethodName, execute, normalize, reasonFor) {
    const before = readFormStateSnapshot(context.form);
    const line = readResourceLineHandle(context.source);
    if (line === null || typeof line.history !== "function") {
      return recordArtifact({
        mode,
        resultKind: "unavailable",
        reason: options.reason ?? "resource replay/restore is unavailable because the form source is not a resource line",
        before,
        after: before,
        resourceReplayRestore: unavailableResult({
          reason: "resourceSourceUnavailable",
          detail: "form source is not a resource line",
          basisCurrentId: null,
          basisAdvanceCount: 0,
        }),
      });
    }
    const historyRead = line.history();
    if (historyRead === null || typeof historyRead !== "object") {
      return recordArtifact({
        mode,
        resultKind: "unavailable",
        reason: options.reason ?? "resource replay/restore is unavailable because the resource line does not expose exact replay/restore history",
        before,
        after: before,
        resourceReplayRestore: unavailableResult({
          reason: "exactHistoryUnavailable",
          detail: "resource line history does not expose exact replay/restore execution",
          basisCurrentId: null,
          basisAdvanceCount: 0,
        }),
      });
    }
    if (typeof historyRead[requiredMethodName] !== "function") {
      return recordArtifact({
        mode,
        resultKind: "unavailable",
        reason: options.reason ?? "resource replay/restore is unavailable because the resource line does not expose exact replay/restore execution",
        before,
        after: before,
        resourceReplayRestore: unavailableResult({
          reason: "exactHistoryUnavailable",
          detail: `resource line history does not expose ${requiredMethodName}()`,
          basisCurrentId: null,
          basisAdvanceCount: 0,
        }),
      });
    }
    const rawResult = execute(historyRead);
    if (isPromiseLike(rawResult)) {
      return rawResult.then((settledResult) => recordReplayRestoreArtifact(
        mode,
        before,
        normalize(settledResult),
        settledResult.kind === "unavailable"
          ? before
          : readFormStateSnapshot(context.form),
        reasonFor,
      ));
    }
    return recordReplayRestoreArtifact(
      mode,
      before,
      normalize(rawResult),
      rawResult.kind === "unavailable"
        ? before
        : readFormStateSnapshot(context.form),
      reasonFor,
    );
  }

  function recordReplayRestoreArtifact(mode, before, resourceReplayRestore, after, reasonFor) {
    return recordArtifact({
      mode,
      resultKind: resourceReplayRestore.kind === "unavailable"
        ? "unavailable"
        : resourceReplayRestore.kind,
      reason: reasonFor(resourceReplayRestore),
      before,
      after,
      resourceReplayRestore,
    });
  }

  function recordArtifact(options) {
    const artifact = {
      kind: "formReplayRestore",
      replayRestoreId: nextReplayRestoreId++,
      observedAtMs: Date.now(),
      mode: options.mode,
      resultKind: options.resultKind,
      reason: options.reason,
      previousSourceDigest: options.before.sourceDigest,
      previousDraftDigest: options.before.draftDigest,
      previousEffectiveDigest: options.before.effectiveDigest,
      nextSourceDigest: options.after.sourceDigest,
      nextDraftDigest: options.after.draftDigest,
      nextEffectiveDigest: options.after.effectiveDigest,
      resourceReplayRestore: options.resourceReplayRestore,
    };
    const frozen = Object.freeze({
      ...artifact,
      replayRestoreDigest: stableValueDigest(artifact),
    });
    history.push(frozen);
    return frozen;
  }
}

function isPromiseLike(value) {
  return value !== null && typeof value === "object" && typeof value.then === "function";
}

function unavailableResult(result) {
  return Object.freeze({
    kind: "unavailable",
    reason: result.reason,
    detail: result.detail,
    basisCurrentId: result.basisCurrentId,
    basisAdvanceCount: result.basisAdvanceCount,
    digest: stableValueDigest(result),
  });
}
