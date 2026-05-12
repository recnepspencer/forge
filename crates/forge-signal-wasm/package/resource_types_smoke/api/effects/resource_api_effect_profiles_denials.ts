import {
  createSignals,
  resourceEffects,
  resourceParamIdentity,
  resourceParams,
  type ResourceEffectMergeExecutionArtifact,
  type ResourceEffectRebaseArtifact,
} from "../../../index.js";

const signals = createSignals();

signals.api({
  // @ts-expect-error effects must be created by resourceEffects
  effects: { name: "fake" },
});

signals.api({}).url("/tasks")
  // @ts-expect-error route effects must be created by resourceEffects
  .effects({ name: "fake" })
  .detail({
    load: () => ({ id: "t1" }),
  });

signals.api({}).url("/tasks")
  .effects(resourceEffects.branchNative())
  // @ts-expect-error route effects can only be owned once
  .effects(resourceEffects.serverCanonical());

signals.api({}).url("/tasks")
  .effects(resourceEffects.branchNative())
  .detail({
    // @ts-expect-error builder-owned effects lane forbids raw effects restatement
    effects: resourceEffects.serverCanonical(),
    load: () => ({ id: "t1" }),
  });

signals.resource.detail({
  params: resourceParams<{ id: string }>(),
  // @ts-expect-error raw resource declaration effects must be created by resourceEffects
  effects: { name: "fake" },
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  load: ({ id }) => ({ id }),
});

resourceEffects.custom({
  name: "bad",
  // @ts-expect-error optimism must stay inside the declared effect vocabulary
  optimism: "maybe",
  confirmation: "serverCanonical",
  rollback: "branchRestore",
  rebase: "nativeMergePlan",
  preimage: "none",
});

const unavailableArtifactWithoutNativeEvidence: ResourceEffectRebaseArtifact = {
  kind: "mappingUnavailable",
  reason: "resourceTopologyMappingUnavailable",
  conflictCount: 1,
  conflicts: [],
  resource: {
    effectId: "effect",
    family: { kind: "collection", familyId: "family" },
    line: { runtimeLineId: "runtime-line", scopeId: "scope", canonicalKey: "line" },
    locus: { kind: "line" },
    topology: null,
    effectLocus: "broadResponse",
  },
  detail: "missing native evidence must stay unrepresentable",
  proof: {
    nativeMergePlanDigest: "plan",
    nativeMergeSemanticsDigest: "semantics",
    resourceLocusDigest: "locus",
    aspectPolicyDigest: "aspect",
    policyBindingDigest: "policy",
    conflictIsolationDigest: "isolation",
  },
  // @ts-expect-error mappingUnavailable artifacts must carry native branch evidence
  native: undefined,
};

void unavailableArtifactWithoutNativeEvidence;

const executionUnavailableArtifactWithoutNativeEvidence:
  ResourceEffectMergeExecutionArtifact = {
    kind: "mappingUnavailable",
    reason: "resourceTopologyMappingUnavailable",
    conflictCount: 1,
    conflicts: [],
    resource: {
      effectId: "effect",
      family: { kind: "collection", familyId: "family" },
      line: {
        runtimeLineId: "runtime-line",
        scopeId: "scope",
        canonicalKey: "line",
      },
      locus: { kind: "line" },
      topology: null,
      effectLocus: "broadResponse",
    },
    detail: "missing native evidence must stay unrepresentable",
    proof: {
      nativeMergeResultDigest: "result",
      nativeMergeSemanticsDigest: "semantics",
      nativeMergeLineageDigest: "lineage",
      resourceLocusDigest: "locus",
      aspectPolicyDigest: "aspect",
      policyBindingDigest: "policy",
      conflictIsolationDigest: "isolation",
    },
    // @ts-expect-error execution mappingUnavailable artifacts must carry native branch evidence
    native: undefined,
  };

void executionUnavailableArtifactWithoutNativeEvidence;

// @ts-expect-error closeout matrices require branded resource effect profiles
resourceEffects.closeoutMatrix({
  name: "fake",
  optimism: "branchSpeculative",
  confirmation: "serverCanonical",
  rollback: "branchRestoreOrInverse",
  rebase: "nativeMergePlan",
  preimage: "compactInverse",
});
