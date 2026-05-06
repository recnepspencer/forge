function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

function projectLineDiagnosticsHistoryDigest(line) {
  const summary = line.summary();
  const history = line.history();
  return {
    summary: {
      current: normalizeForProof(summary.current),
      request: {
        familyKind: summary.request.family.kind,
        method: summary.request.method,
        authKind: summary.request.auth.kind,
        baseUrl: summary.request.baseUrl,
        context: normalizeForProof(summary.request.context),
        continuation: normalizeForProof(summary.request.continuation),
        processingJob: normalizeForProof(summary.request.processingJob),
        uploadTransport: normalizeForProof(summary.request.uploadTransport),
      },
      processing: normalizeForProof(summary.processing),
      upload: normalizeForProof(summary.upload),
      download: normalizeForProof(summary.download),
      diagnostics: {
        current: normalizeForProof(summary.diagnostics.current),
        activity: normalizeForProof(summary.diagnostics.activity),
        counts: normalizeForProof(summary.diagnostics.counts),
        latest: normalizeForProof(summary.diagnostics.latest),
        request: {
          baseUrl: summary.diagnostics.request.baseUrl,
          method: summary.diagnostics.request.method,
          bodyPresent: summary.diagnostics.request.bodyPresent,
          authKind: summary.diagnostics.request.auth.kind,
          context: normalizeForProof(summary.diagnostics.request.context),
          continuation: normalizeForProof(summary.diagnostics.request.continuation),
          processingJob: normalizeForProof(summary.diagnostics.request.processingJob),
          uploadTransport: normalizeForProof(summary.diagnostics.request.uploadTransport),
        },
        processing: normalizeForProof(summary.diagnostics.processing),
        upload: normalizeForProof(summary.diagnostics.upload),
        download: normalizeForProof(summary.diagnostics.download),
        explainability: normalizeForProof(summary.diagnostics.explainability),
      },
      explainability: normalizeForProof(summary.explainability),
    },
    history: {
      availability: normalizeForProof(history.availability),
      basis: normalizeForProof(history.basis),
      lifecycleLength: history.lifecycle.length,
      lastLifecycleEvent: history.lifecycle.at(-1)?.event ?? null,
      lifecycleTail: normalizeForProof(history.lifecycle.slice(-3)),
    },
  };
}

export { projectLineDiagnosticsHistoryDigest };
