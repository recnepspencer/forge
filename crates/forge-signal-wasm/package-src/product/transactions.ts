import {
  activeComputedCallbackFrame,
  denySignalMutationDuringCallbackAuthoring,
} from "./callback_frames.js";
import { buildHostCapabilityTransportReport } from "./host_capability_reports.js";
import { unwrapInputSignalTarget } from "./handles.js";
import { RAW_SIGNAL_HANDLE } from "./symbols.js";

function attachSerializedField(value, field, serialized) {
  if (!value || typeof value !== "object" || typeof serialized !== "string") {
    return value;
  }
  Object.defineProperty(value, field, {
    value: serialized,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return value;
}

function attachLiteralField(value, field, literal) {
  if (!value || typeof value !== "object") {
    return value;
  }
  Object.defineProperty(value, field, {
    value: literal,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return value;
}

function throwPortableImportUnavailableCallbacks(envelope, hostCapabilities) {
  const unavailableCallbacks = envelope?.definitions?.unavailableCallbacks;
  if (!Array.isArray(unavailableCallbacks) || unavailableCallbacks.length === 0) {
    return;
  }
  hostCapabilities.recordPortableImportDenial(unavailableCallbacks);
  const ids = unavailableCallbacks
    .map((artifact) => artifact?.id)
    .filter((id) => typeof id === "string" && id.length > 0)
    .join(", ");
  throw {
    code: "computeCallbackUnavailableForRuntimeEnvelopeImport",
    message: `runtime envelope import cannot restore callback-backed nodes without live callback registrations: ${ids}`,
  };
}

export function wrapTransaction(rawTx, rawSignals) {
  return Object.freeze({
    set(input, value) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.set(unwrapInputSignalTarget(input, rawSignals, "transaction.set"), value);
    },
    setWithAspects(input, value, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithAspects(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithAspects"),
        value,
        aspects,
      );
    },
    setWithRegions(input, value, changedRegions) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegions(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithRegions"),
        value,
        changedRegions,
      );
    },
    setWithRegionsAndAspects(input, value, changedRegions, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegionsAndAspects(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithRegionsAndAspects"),
        value,
        changedRegions,
        aspects,
      );
    },
    free() {
      rawTx.free();
    },
    [Symbol.dispose]() {
      if (typeof rawTx[Symbol.dispose] === "function") {
        rawTx[Symbol.dispose]();
        return;
      }
      rawTx.free();
    },
  });
}

function requirePublishedGraphInput(inputOrName, publishedInputs, publishedInputHandleSet, rawSignals, graphId, operation) {
  if (typeof inputOrName === "string") {
    const publishedInput = publishedInputs[inputOrName];
    if (!publishedInput) {
      throw new TypeError(
        `${operation} only accepts published graph input names from graph \`${graphId}\`; \`${inputOrName}\` is not part of the public input contract`,
      );
    }
    return publishedInput[RAW_SIGNAL_HANDLE];
  }

  const rawInput = unwrapInputSignalTarget(inputOrName, rawSignals, operation);
  const signalId = typeof inputOrName?.id === "string" ? inputOrName.id : "<unknown>";
  if (!publishedInputHandleSet.has(inputOrName)) {
    throw new TypeError(
      `${operation} only accepts published graph inputs from graph \`${graphId}\`; \`${signalId}\` is outside the graph contract`,
    );
  }
  return rawInput;
}

function requireGraphInputWritable(authorities, graphId, inputName, operation) {
  const authority = authorities[inputName];
  if (!authority?.supportsWrite) {
    throw new TypeError(
      `${operation} cannot write public input \`${inputName}\` from graph \`${graphId}\` because its authority is \`${authority?.authority ?? "unknown"}\``,
    );
  }
}

export function wrapGraphTransaction(rawTx, rawSignals, graphId, publishedInputs, authorities) {
  const publishedInputHandleSet = new Set(Object.values(publishedInputs));

  function resolve(inputOrName, operation) {
    const rawInput = requirePublishedGraphInput(
      inputOrName,
      publishedInputs,
      publishedInputHandleSet,
      rawSignals,
      graphId,
      operation,
    );
    const inputName = typeof inputOrName === "string"
      ? inputOrName
      : Object.entries(publishedInputs).find(([, handle]) => handle === inputOrName)?.[0];
    if (typeof inputName === "string") {
      requireGraphInputWritable(authorities, graphId, inputName, operation);
    }
    return rawInput;
  }

  return Object.freeze({
    set(inputOrName, value) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.set(resolve(inputOrName, "graph.transaction.set"), value);
    },
    setWithAspects(inputOrName, value, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithAspects(
        resolve(inputOrName, "graph.transaction.setWithAspects"),
        value,
        aspects,
      );
    },
    setWithRegions(inputOrName, value, changedRegions) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegions(
        resolve(inputOrName, "graph.transaction.setWithRegions"),
        value,
        changedRegions,
      );
    },
    setWithRegionsAndAspects(inputOrName, value, changedRegions, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegionsAndAspects(
        resolve(inputOrName, "graph.transaction.setWithRegionsAndAspects"),
        value,
        changedRegions,
        aspects,
      );
    },
    free() {
      rawTx.free();
    },
    [Symbol.dispose]() {
      if (typeof rawTx[Symbol.dispose] === "function") {
        rawTx[Symbol.dispose]();
        return;
      }
      rawTx.free();
    },
  });
}

export function wrapAdapters(rawAdapters, hostCapabilities) {
  return Object.freeze({
    exportDefinitions() {
      return rawAdapters.export_definitions();
    },
    exportRuntimeEnvelope() {
      const envelope = attachSerializedField(
        rawAdapters.export_runtime_envelope(),
        "runtimeEnvelopeRestoreToken",
        rawAdapters.export_runtime_envelope_wire(),
      );
      hostCapabilities.recordExportedUnavailableCallbacks(
        envelope?.definitions?.unavailableCallbacks,
      );
      attachSerializedField(
        envelope,
        "runtimeEnvelopePortableWire",
        rawAdapters.export_runtime_envelope_portable_wire(),
      );
      return attachLiteralField(envelope, "runtimeEnvelopeRestoreMode", "SameRuntimeExact");
    },
    replaceRuntimeEnvelope(envelope) {
      throwPortableImportUnavailableCallbacks(envelope, hostCapabilities);
      if (typeof envelope?.runtimeEnvelopePortableWire === "string") {
        return rawAdapters.replace_runtime_envelope_portable_wire(envelope.runtimeEnvelopePortableWire);
      }
      return rawAdapters.replace_runtime_envelope(envelope);
    },
    restoreExactRuntimeEnvelope(envelope) {
      if (typeof envelope?.runtimeEnvelopeRestoreToken !== "string") {
        throw new TypeError(
          "adapters.restoreExactRuntimeEnvelope expects an artifact returned by adapters.exportRuntimeEnvelope()",
        );
      }
      return rawAdapters.replace_runtime_envelope_wire(envelope.runtimeEnvelopeRestoreToken);
    },
    runtimeProofReport() {
      return rawAdapters.runtime_proof_report();
    },
    hostCapabilityTransportReport(envelope) {
      const targetEnvelope = envelope ?? this.exportRuntimeEnvelope();
      return buildHostCapabilityTransportReport(
        targetEnvelope?.definitions?.unavailableCallbacks,
      );
    },
    free() {
      rawAdapters.free();
    },
    [Symbol.dispose]() {
      if (typeof rawAdapters[Symbol.dispose] === "function") {
        rawAdapters[Symbol.dispose]();
        return;
      }
      rawAdapters.free();
    },
  });
}
