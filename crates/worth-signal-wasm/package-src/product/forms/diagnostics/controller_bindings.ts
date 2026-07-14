import { readFormDiagnostics } from "../form_diagnostics.js";
import { buildFormVerificationPackage } from "../verification.js";
import { stableValueDigest } from "../values/value_paths.js";
import { digestFormDiagnosticsProof, readFormDiagnosticsStateDigestInput } from "./digests.js";
import { readFormDiagnosticsState } from "./state.js";
import { readFormDiagnosticsSummary } from "./summary.js";

export function createDiagnosticsControllerBindings(options) {
  return Object.freeze({
    diagnosticsSummary() {
      const diagnosticsSnapshot = synchronizeDiagnosticsSnapshot(options);
      return diagnosticsSnapshot.summary;
    },
    diagnosticsHistory() {
      const diagnosticsSnapshot = synchronizeDiagnosticsSnapshot(options);
      return diagnosticsSnapshot.history;
    },
    verification() {
      options.currentMeasurementSemanticContext();
      const diagnosticsSnapshot = synchronizeDiagnosticsSnapshot(options);
      return buildFormVerificationPackage(options.formRef(), diagnosticsSnapshot);
    },
    diagnostics() {
      const diagnosticsSnapshot = synchronizeDiagnosticsSnapshot(options);
      const verification = buildFormVerificationPackage(options.formRef(), diagnosticsSnapshot);
      const diagnosticsDigest = digestFormDiagnosticsProof(
        diagnosticsSnapshot.summary,
        diagnosticsSnapshot.diagnosticsStateDigest,
        diagnosticsSnapshot.history,
      );
      return readFormDiagnostics(
        diagnosticsSnapshot.state,
        diagnosticsSnapshot.summary,
        diagnosticsSnapshot.history,
        verification,
        diagnosticsDigest,
      );
    },
  });
}

function synchronizeDiagnosticsSnapshot(options) {
  const state = readFormDiagnosticsState(options.formRef(), options.fieldDeclarations);
  const summary = readFormDiagnosticsSummary(state);
  const diagnosticsStateDigest = stableValueDigest({
    summaryDigest: summary.digest,
    diagnostics: readFormDiagnosticsStateDigestInput(state),
  });
  options.diagnosticsHistory.reconcile(state, summary, diagnosticsStateDigest);
  return Object.freeze({
    state,
    summary,
    diagnosticsStateDigest,
    history: options.diagnosticsHistory.history(),
  });
}
