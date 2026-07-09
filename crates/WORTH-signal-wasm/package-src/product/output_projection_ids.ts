const OUTPUT_CALLBACK_PROJECTION_COUNTERS = new WeakMap();

function nextOutputProjectionId(rawSignals, outputId) {
  const next = (OUTPUT_CALLBACK_PROJECTION_COUNTERS.get(rawSignals) ?? 0) + 1;
  OUTPUT_CALLBACK_PROJECTION_COUNTERS.set(rawSignals, next);
  return `__WORTHSignal.outputProjection.${outputId}.${next}`;
}

function outputProjectionSpec(hiddenComputedId) {
  return {
    reads: [hiddenComputedId],
    expr: {
      kind: "read",
      id: hiddenComputedId,
    },
  };
}

export { nextOutputProjectionId, outputProjectionSpec };
