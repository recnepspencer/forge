import { stableValueDigest } from "../values/value_paths.js";

const EMPTY_FIELD_STATE = Object.freeze({
  touched: false,
  visited: false,
  focusIntent: false,
  blurred: false,
  lastInputSource: null,
  composing: false,
  compositionDigest: null,
});

const EMPTY_SUBMIT_INTENT = Object.freeze({
  active: false,
  source: null,
  count: 0,
});

export function createInteractionStore() {
  let nextArtifactId = 1;
  const history = [];
  const states = new Map();
  let submitIntent = EMPTY_SUBMIT_INTENT;

  return Object.freeze({
    touch(fieldId, source = "imperative") {
      return recordFieldArtifact(fieldId, "touched", source, null, { touched: true });
    },
    visit(fieldId, source = "imperative") {
      return recordFieldArtifact(fieldId, "visited", source, null, { visited: true });
    },
    input(fieldId, source = "typing", rawValue = null) {
      return recordFieldArtifact(fieldId, "input", source, rawValue, {
        touched: true,
        lastInputSource: source,
      });
    },
    compose(fieldId, rawValue) {
      return recordFieldArtifact(fieldId, "compositionStarted", "composition", rawValue, {
        touched: true,
        composing: true,
        compositionDigest: stableValueDigest(rawValue),
      });
    },
    finishComposition(fieldId, resultKind = "compositionCommitted") {
      if (!(states.get(fieldId) ?? EMPTY_FIELD_STATE).composing) {
        return null;
      }
      return recordFieldArtifact(fieldId, resultKind, "composition", null, {
        composing: false,
        compositionDigest: null,
      });
    },
    focus(fieldId, source = "imperative") {
      for (const [currentFieldId, currentState] of states) {
        if (!currentState.focusIntent) {
          continue;
        }
        states.set(currentFieldId, Object.freeze({
          ...currentState,
          focusIntent: false,
          blurred: true,
        }));
      }
      return recordFieldArtifact(fieldId, "focused", source, null, {
        focusIntent: true,
        blurred: false,
      });
    },
    blur(fieldId, source = "imperative") {
      return recordFieldArtifact(fieldId, "blurred", source, null, {
        visited: true,
        focusIntent: false,
        blurred: true,
      });
    },
    reportSubmitIntent(source = "programmatic") {
      submitIntent = Object.freeze({
        active: true,
        source,
        count: submitIntent.count + 1,
      });
      return recordSubmitIntentArtifact("reported", source, null);
    },
    clearSubmitIntent(reason = null) {
      const previousSource = submitIntent.source;
      submitIntent = Object.freeze({
        ...submitIntent,
        active: false,
        source: null,
      });
      return recordSubmitIntentArtifact("cleared", previousSource, reason);
    },
    report(fieldDeclarations, hostReport) {
      const focusIntentField = fieldDeclarations.find((field) => (
        (states.get(field.id) ?? EMPTY_FIELD_STATE).focusIntent
      ))?.id ?? null;
      const focusedField = hostReport.facts.focus.posture === "supported"
        ? hostReport.facts.focus.focusedField ?? null
        : focusIntentField;
      const fields = fieldDeclarations.map((field) => {
        const state = states.get(field.id) ?? EMPTY_FIELD_STATE;
        const rawInputPosture = field.inputAdapter.capabilities.reportsRawInput
          ? Object.freeze({ posture: "supported", reason: null })
          : Object.freeze({
              posture: "unavailable",
              reason: `${field.id} adapter does not report raw input`,
            });
        const compositionPosture = field.inputAdapter.capabilities.reportsComposition
          ? Object.freeze({ posture: "supported", reason: null })
          : Object.freeze({
              posture: "unavailable",
              reason: `${field.id} adapter does not report composition`,
            });
        return Object.freeze({
          field: field.id,
          path: field.path,
          touched: state.touched,
          visited: state.visited,
          focused: focusedField === field.id,
          focusIntent: state.focusIntent,
          blurred: state.blurred,
          lastInputSource: state.lastInputSource,
          composing: state.composing,
          compositionDigest: state.compositionDigest,
          rawInputPosture,
          compositionPosture,
          focusPosture: hostReport.facts.focus.posture,
          focusReason: hostReport.facts.focus.reason,
          interactionDigest: stableValueDigest({
            field: field.id,
            touched: state.touched,
            visited: state.visited,
            focused: focusedField === field.id,
            focusIntent: state.focusIntent,
            blurred: state.blurred,
            lastInputSource: state.lastInputSource,
            composing: state.composing,
            rawInputPosture: rawInputPosture.posture,
            compositionPosture: compositionPosture.posture,
            focusPosture: hostReport.facts.focus.posture,
          }),
        });
      });
      const summary = Object.freeze({
        fields: fields.length,
        touchedFields: fields.filter((field) => field.touched).length,
        visitedFields: fields.filter((field) => field.visited).length,
        focusedField,
        focusIntentField,
        focusPosture: hostReport.facts.focus.posture,
        composingFields: fields.filter((field) => field.composing).length,
        rawInputUnavailableFields: fields.filter((field) => field.rawInputPosture.posture === "unavailable").length,
        compositionUnavailableFields: fields.filter((field) => field.compositionPosture.posture === "unavailable").length,
        inputSources: Object.freeze({
          typing: fields.filter((field) => field.lastInputSource === "typing").length,
          paste: fields.filter((field) => field.lastInputSource === "paste").length,
          drop: fields.filter((field) => field.lastInputSource === "drop").length,
          autofill: fields.filter((field) => field.lastInputSource === "autofill").length,
        }),
        submitIntent,
      });
      const counters = Object.freeze({
        costBasis: "interactionArtifactAndHostFocusScan",
        incrementalStatus: "notIncremental",
        fields: fields.length,
        touchedFields: summary.touchedFields,
        visitedFields: summary.visitedFields,
        focusedFields: fields.filter((field) => field.focused).length,
        composingFields: summary.composingFields,
        rawInputUnavailableFields: summary.rawInputUnavailableFields,
        compositionUnavailableFields: summary.compositionUnavailableFields,
        inputArtifacts: history.filter((entry) => entry.kind === "fieldInteraction" && entry.interaction === "input").length,
        compositionArtifacts: history.filter((entry) => (
          entry.kind === "fieldInteraction" &&
          (entry.interaction === "compositionStarted" ||
            entry.interaction === "compositionCommitted" ||
            entry.interaction === "compositionCancelled")
        )).length,
        focusArtifacts: history.filter((entry) => (
          entry.kind === "fieldInteraction" &&
          (entry.interaction === "focused" || entry.interaction === "blurred")
        )).length,
        submitIntentArtifacts: history.filter((entry) => entry.kind === "submitIntent").length,
        interactionArtifacts: history.length,
      });
      const report = {
        fields: Object.freeze(fields),
        summary,
        counters,
        history: Object.freeze([...history]),
      };
      return Object.freeze({
        ...report,
        digest: stableValueDigest(report),
      });
    },
  });

  function recordFieldArtifact(fieldId, interaction, source, rawValue, patch) {
    const previous = states.get(fieldId) ?? EMPTY_FIELD_STATE;
    states.set(fieldId, Object.freeze({
      touched: patch.touched ?? previous.touched,
      visited: patch.visited ?? previous.visited,
      focusIntent: patch.focusIntent ?? previous.focusIntent,
      blurred: patch.blurred ?? previous.blurred,
      lastInputSource: patch.lastInputSource ?? previous.lastInputSource,
      composing: patch.composing ?? previous.composing,
      compositionDigest: patch.compositionDigest ?? previous.compositionDigest,
    }));
    const artifact = Object.freeze({
      kind: "fieldInteraction",
      artifactId: nextArtifactId++,
      field: fieldId,
      interaction,
      source,
      rawDigest: rawValue === null ? null : stableValueDigest(rawValue),
      interactionDigest: stableValueDigest({
        field: fieldId,
        interaction,
        source,
        rawValue,
      }),
    });
    history.push(artifact);
    return artifact;
  }

  function recordSubmitIntentArtifact(resultKind, source, reason) {
    const artifact = Object.freeze({
      kind: "submitIntent",
      artifactId: nextArtifactId++,
      source,
      resultKind,
      reason,
      intentDigest: stableValueDigest({
        source,
        resultKind,
        reason,
        count: submitIntent.count,
      }),
    });
    history.push(artifact);
    return artifact;
  }
}
