import { createSignals } from "../../../index.js";

const signals = createSignals();

type Task = {
  id: string;
  title: string;
  status: "open" | "done";
  metadata: {
    priority: number;
    labels: readonly string[];
    nested: { rank: number };
    note?: string | null;
  };
};

const taskResponse = signals.resource.response.array({
  itemId: (item: Task) => item.id,
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
    status: "status",
  }),
});

const tasks = signals.api({}).url("/tasks").response(taskResponse).list({
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

const taskDetailResponse = signals.resource.response.detail<Task>()({
  title: "title",
});

signals.api({}).url("/tasks").response(taskDetailResponse).create({
  reconciles: [{
    family: tasks,
    params: () => ({}),
    fallback: "refetchRequired",
    // @ts-expect-error create responses must not overclaim Phase 3 exact reconciliation targets
    collection: { kind: "item" },
  }],
  load: ({ body }) => body,
});

signals.api({}).url("/tasks/:taskId").params<{ taskId: string }>()
  .response(taskDetailResponse)
  .remove({
    // @ts-expect-error remove responses must not overclaim Phase 3 diagnostics
    diagnostics: [{ kind: "warnings", field: "title" }],
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open",
      metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
    }),
  });

signals.api({}).url("/tasks").response(taskDetailResponse).create({
  identity: {
    submitted: ({ body }: { body: Task }) => body.id,
    response: (value: Task) => value.id,
    canonical: (value: Task) => value.id,
    targets: [{
      family: tasks,
      params: (_params: { body: Task }) => ({}),
      // @ts-expect-error identity migration targets only admit the dedicated fallback posture set
      fallback: "unsupportedTarget",
    }],
  },
  load: ({ body }: { body: Task }) => body,
});

signals.api({}).url("/tasks/:taskId").params<{ taskId: string }>()
  .response(taskDetailResponse)
  .update({
    // @ts-expect-error ordinary mutation atomicity accepts only allOrNone or partialAllowed
    atomicity: "sometimes",
    reconciles: [{
      family: tasks,
      params: () => ({}),
      fallback: "partialReconciliation",
      detail: { kind: "replace" },
    }],
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open",
      metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
    }),
  });

signals.api({}).url("/tasks").response(taskDetailResponse).create({
  identity: {
    submitted: ({ body }: { body: Task }) => body.id,
    canonical: (value: Task) => value.id,
    // @ts-expect-error identity atomicity accepts only allOrNone or partialAllowed
    atomicity: "sometimes",
  },
  load: ({ body }: { body: Task }) => body,
});

const taskIdentityDetail = signals.api({}).url("/tasks/:taskId")
  .params<{ taskId: string }>()
  .response(taskDetailResponse)
  .detail({
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open" as const,
      metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
    }),
  });

const invalidIdentityTarget:
  import("../../../types/resource/resource_mutation_response.js").ResourceMutationResponseIdentityTargetDeclaration<
    { body: Task },
    Task,
    typeof taskIdentityDetail
  > = {
    family: taskIdentityDetail,
    params: ({ body }: { body: Task }) => ({
      taskId: body.id,
      params: { taskId: body.id },
    }),
    // @ts-expect-error identity canonicalParams must return the declared target family params
    canonicalParams: (_params: { body: Task }, _value: Task, canonicalIdentity: string) => ({
      taskKey: canonicalIdentity,
    }),
    fallback: "identityMigrationUnavailable",
  };

void invalidIdentityTarget;

signals.api({}).url("/tasks/:taskId").params<{ taskId: string }>()
  .response(taskDetailResponse)
  .remove({
    // @ts-expect-error remove responses do not admit identity migration declarations
    identity: {
      submitted: ({ taskId }: { taskId: string | number }) => String(taskId),
      canonical: (value: Task) => value.id,
    },
    load: ({ taskId }: { taskId: string | number }) => ({
      id: String(taskId),
      title: "Task",
      status: "open",
      metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
    }),
  });
