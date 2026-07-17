import { createLocalTruthAuthority } from "./authority/local_truth_authority.js";
import { createCompatibilitySignalProjectionDriver } from "./projection/compatibility_signal_driver.js";
import { createLocalTruthSignalProjection } from "./projection/signal_projection.js";
import { requireDeclaredSchema } from "./schema/schema_declaration.js";
import { canonicalDigest } from "./support/canonical.js";

export function createCompatibilityLocalTruthFactory(signals) {
  return function localTruth(options) {
    const schema = requireDeclaredSchema(options?.schema, "signals.localTruth");
    const bindings = normalizePublicBindings(options?.bindings);
    validateInitialBindings(options?.initialEntities, bindings);
    const projection = createLocalTruthSignalProjection({
      schema,
      bindings,
      driver: createCompatibilitySignalProjectionDriver(signals),
    });
    return createLocalTruthAuthority(
      { ...options, schema },
      {
        faultInjector: null,
        onInitialize: (branch, snapshot) => projection.initialize(branch, snapshot),
        onBranchFork: (branch, parent) => projection.fork(branch, parent),
        onCommitted: (commit, snapshot) => projection.project(commit, snapshot),
        projection,
      },
    );
  };
}

export function normalizePublicBindings(bindings) {
  if (!Array.isArray(bindings)) {
    throw new TypeError("signals.localTruth bindings must be an array");
  }
  return bindings.map((binding) => {
    const signalId = typeof binding?.input === "function"
      ? binding.input.id
      : binding?.signalId;
    if (typeof binding?.entityId !== "string" || typeof signalId !== "string") {
      throw new TypeError("signals.localTruth binding requires entityId and an input handle");
    }
    const initialValue = typeof binding.input?.value === "function"
      ? binding.input.value()
      : typeof binding.input === "function"
        ? binding.input()
        : binding.initialValue;
    return {
      entityId: binding.entityId,
      signalId,
      aspectMap: binding.aspectMap,
      initialValue,
    };
  });
}

export function validateInitialBindings(initialEntities, bindings) {
  for (const binding of bindings) {
    if (
      !initialEntities
      || !Object.hasOwn(initialEntities, binding.entityId)
      || canonicalDigest(initialEntities[binding.entityId]) !== canonicalDigest(binding.initialValue)
    ) {
      throw new TypeError(
        `signals.localTruth initial entity ${binding.entityId} must equal its bound Signal input value`,
      );
    }
  }
}
