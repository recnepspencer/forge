import {
  createSignals,
  resourceCollectionShape,
  resourceEffects,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourceResponse,
  resourceValueSummaries,
  type ResourceEffectBranchLifecycle,
  type ResourceEffectCompactInverseDescriptor,
  type ResourceEffectEnvelope,
  type ResourceEffectLocusProof,
  type ResourceEffectRollback,
  type ResourceResponseLensDenialProof,
  type ResourceResponseLensProof,
  type ResourceLineEffectRollbackResult,
  type ResourceLineVisibleSelection,
} from "../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

type Task = { id: string; title: string };
type TaskResponse = { items: Task[] };

const tasks = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  effects: resourceEffects.branchNative(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (task: Task) => task.id,
  reconcile: resourceCollectionShape<
    TaskResponse,
    Task,
    {
      title: {
        read(task: Task): string;
        write(task: Task, title: string): Task;
      };
    },
    {}
  >({
    items: (value: TaskResponse) => value.items,
    replaceItems: (
      value: TaskResponse,
      nextItems: readonly Task[],
    ) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (task: Task) => task.title,
        write: (task: Task, title: string) => ({ ...task, title }),
      },
    }),
  }),
  load: () => ({ items: [{ id: "task:1", title: "Loaded" }] }),
});

const taskLine = tasks.line({ workspaceId: "demo" });

taskLine.patch(
  resourcePatch.itemAspect({
    itemId: "task:1",
    aspect: "title",
    value: "Patched",
  }),
);

const latestEffect: ResourceEffectEnvelope | null =
  taskLine.diagnostics().lastEffect;
const latestBranchLifecycle: ResourceEffectBranchLifecycle | undefined =
  latestEffect?.branchLifecycle;
const latestRollback: ResourceEffectRollback | undefined =
  latestEffect?.optimistic.rollback;
const latestInverse: ResourceEffectCompactInverseDescriptor | null =
  latestRollback?.kind === "compactInverseAvailable"
    ? latestRollback.inverse
    : null;
const latestLocusProof: ResourceEffectLocusProof | null | undefined =
  latestEffect?.locusProof;
const rollbackResult: ResourceLineEffectRollbackResult =
  taskLine.history().rollbackLastEffect();
const visibleSelection: ResourceLineVisibleSelection =
  taskLine.diagnostics().visibleSelection;
const response = resourceResponse.array<Task>({
  itemId: (task) => task.id,
  summaries: resourceValueSummaries({
    count: {
      read: (value: readonly Task[]) => value.length,
      write: (value: readonly Task[]) => value,
    },
  }),
});
const responseLensProof: ResourceResponseLensProof = response.lensProof;
const jsonResponse = resourceResponse.array<Task>({
  itemId: (task) => task.id,
  aspects: resourceResponse.jsonObjectAspects<Task>()({
    title: "title",
  }),
});
const jsonAspectLocus: ResourceEffectEnvelope["locus"] = {
  kind: "jsonItemAspect",
  itemId: "task:1",
  aspect: "title",
};
const detailResponse = resourceResponse.detail<Task>();
const detailResponseLocus: ResourceEffectEnvelope["locus"] = {
  kind: "detailResponse",
};
const summaryResponse = resourceResponse.summary<{ total: number }>();
const summaryResponseLocus: ResourceEffectEnvelope["locus"] = {
  kind: "summaryResponse",
};
const entityStoreLocus: ResourceEffectEnvelope["locus"] = {
  kind: "entityStore",
  itemId: "task:1",
};
const mapCollectionLocus: ResourceEffectEnvelope["locus"] = {
  kind: "mapCollection",
  itemId: "task:1",
};
const groupedCollectionLocus: ResourceEffectEnvelope["locus"] = {
  kind: "groupedCollection",
  itemId: "task:1",
};
const discriminatedTupleLocus: ResourceEffectEnvelope["locus"] = {
  kind: "discriminatedTuple",
  itemId: "task:1",
};
const sparsePageLocus: ResourceEffectEnvelope["locus"] = {
  kind: "sparsePage",
  itemId: "task:1",
};
const namedCollectionLocus: ResourceEffectEnvelope["locus"] = {
  kind: "namedCollection",
  itemId: "task:1",
};
const recursiveTreeLocus: ResourceEffectEnvelope["locus"] = {
  kind: "recursiveTree",
  itemId: "task:1",
};

void latestEffect?.version;
void latestEffect?.plan.admissionKind;
void latestEffect?.plan.branch.kind;
void latestEffect?.branchLifecycle.kind;
void latestEffect?.branchLifecycle.creation;
void latestEffect?.branchLifecycle.disposal.kind;
void latestEffect?.branchLifecycle.leakDenial.kind;
void latestEffect?.optimistic.kind;
void latestRollback?.kind;
void latestInverse?.cost.retainedResponsePreimage;
void rollbackResult.kind;
void rollbackResult.rollback?.kind;
void visibleSelection.kind;
void taskLine.diagnosticsSummary().current.visibleSelection.kind;
void latestEffect?.counters.planningBreadth;
void latestEffect?.counters.branchProofBreadth;
void latestEffect?.counters.branchLifecycleBreadth;
void latestEffect?.counters.optimisticLifecycleBreadth;
void latestEffect?.counters.serverConfirmationBreadth;
void latestEffect?.counters.rollbackReadinessBreadth;
void latestEffect?.counters.responseLensBreadth;
void latestEffect?.counters.effectLocusBreadth;
void latestEffect?.counters.detailRegionTraversalBreadth;
void latestEffect?.counters.detailRegionReconstructionBreadth;
void latestEffect?.counters.jsonPathTraversalBreadth;
void latestEffect?.counters.jsonPathReconstructionBreadth;
void latestEffect?.patch.field;
void latestEffect?.patch.regionName;
void latestEffect?.patch.path;
void latestEffect?.patch.region?.regionName;
void latestEffect?.patch.region?.identityBoundary;
void latestEffect?.patch.region?.mergeGranularity;
void latestEffect?.patch.region?.cost.cloneBreadth;
void latestEffect?.patch.jsonPath?.cost.cloneBreadth;
void latestEffect?.patch.jsonPath?.policy.containerWrite;
void latestEffect?.patch.jsonPath?.policy.objectPrototype;
void latestEffect?.patch.jsonPath?.policy.extensibility;
void latestEffect?.patch.jsonPath?.policy.prototypeReconstruction;
void latestEffect?.patch.jsonPath?.policy.absence;
void latestLocusProof?.topology;
const responseBroadLocus: ResourceEffectEnvelope["locus"] = {
  kind: "broadResponse",
};
void responseBroadLocus.kind;
const responseMembershipLocus: ResourceEffectEnvelope["locus"] = {
  kind: "membership",
  itemId: "task:1",
};
void responseMembershipLocus.itemId;
void responseLensProof.capabilityRows;
void responseLensProof.declarationDigest;
void responseLensProof.capabilityDigest;
void responseLensProof.compiledLensDigest;
void responseLensProof.parityDigest;
void responseLensProof.compileBoundaryDigest;
const denialProof: ResourceResponseLensDenialProof = {
  version: "resource-response-lens-denial-proof-v1",
  lensVersion: "resource-response-lens-proof-v1",
  lensSource: responseLensProof.source,
  declarationDigest: responseLensProof.declarationDigest,
  capabilityDigest: responseLensProof.capabilityDigest,
  compiledLensDigest: responseLensProof.compiledLensDigest,
  parityDigest: responseLensProof.parityDigest,
  compileBoundaryDigest: responseLensProof.compileBoundaryDigest,
  requestedLocus: "summary",
  requestedPatchScope: "summary",
  field: null,
  aspect: null,
  summary: "count",
  reason: "listSummaryScopeMismatch",
  denialDigest: "response-lens-denial",
};
void denialProof.denialDigest;
void jsonResponse.lensProof.jsonAspectNames;
void detailResponse().lensProof.topology;
void summaryResponse.lensProof.topology;
void detailResponseLocus.kind;
void summaryResponseLocus.kind;
void entityStoreLocus.itemId;
void mapCollectionLocus.itemId;
void groupedCollectionLocus.itemId;
void discriminatedTupleLocus.itemId;
void namedCollectionLocus.itemId;
void recursiveTreeLocus.itemId;
void sparsePageLocus.itemId;
void jsonAspectLocus.aspect;
void responseLensProof.summaryNames;
void latestLocusProof?.declarationDigest;
void latestLocusProof?.capabilityDigest;
void latestLocusProof?.cost.lookupBreadth;
void latestLocusProof?.compiledLensDigest;
void latestLocusProof?.parityDigest;
void latestLocusProof?.compileBoundaryDigest;
void latestLocusProof?.capabilityRowDigest;
void latestLocusProof?.effectLocusDigest;
void taskLine.diagnosticsSummary().latest.effect?.locus.kind;
void taskLine.history().verificationPackage().lifecycle.lastEffect?.effectId;
void taskLine.history().verificationPackage()
  .deliveryProvenance.lastEffect?.provenance;
void latestBranchLifecycle?.kind;
