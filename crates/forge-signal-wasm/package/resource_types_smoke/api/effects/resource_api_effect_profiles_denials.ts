import {
  createSignals,
  resourceEffects,
  resourceParamIdentity,
  resourceParams,
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
