import {
  createSignals,
  resourceEffects,
  resourceParamIdentity,
  resourceParams,
  type ResourceEffectProfile,
} from "../../../index.js";

const signals = createSignals();

const scopedApi = signals.api({
  effects: resourceEffects.branchNative(),
}).scope({
  effects: ({ workspaceId }: { workspaceId: string }) =>
    workspaceId === "sensitive"
      ? resourceEffects.sensitive()
      : resourceEffects.serverCanonical(),
});

const directResourceDetail = signals.resource.detail({
  params: resourceParams<{ id: string }>(),
  effects: resourceEffects.serverCanonical(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  load: ({ id }) => ({ id }),
});

const taskDetail = scopedApi.url("/workspaces/:workspaceId/tasks/:taskId")
  .effects(resourceEffects.pessimistic())
  .detail({
    load: ({ workspaceId, taskId }) => ({
      id: taskId,
      workspaceId,
    }),
  });

const taskLine = taskDetail.line({
  workspaceId: "demo",
  taskId: "t1",
});

const lineEffects: ResourceEffectProfile | null = taskLine.request().effects;
const diagnosticsEffects: ResourceEffectProfile | null =
  taskLine.diagnostics().request.effects;

void lineEffects;
void diagnosticsEffects;
void signals.resource.effects.deliveryAuthoritative();
void signals.resource.branch.planMerge({
  source_branch_id: 0,
  target_branch_id: 0,
});
void directResourceDetail.line({ id: "direct" }).request().effects;
