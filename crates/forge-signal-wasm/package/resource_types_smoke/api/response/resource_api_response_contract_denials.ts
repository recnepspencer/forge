import { createSignals, resourceCollectionShape } from "../../../index.js";

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

type TaskEnvelope = {
  tasks: readonly Task[];
  nextCursor: string | null;
};

signals.resource.response.objectAspects<Task>()({
  title: "title",
  // @ts-expect-error object response aspects must name real item fields
  missing: "missing",
});

signals.resource.response.jsonPathAspects<Task>()({
  missingField: {
    // @ts-expect-error JSON path aspects must name real item fields
    field: "missing",
    path: ["nested", "rank"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  invalidPath: {
    field: "title",
    // @ts-expect-error JSON path arrays accept only string object keys and numeric array indexes
    path: [false],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  scalarRootPath: {
    field: "title",
    // @ts-expect-error JSON path declarations cannot traverse scalar root fields
    path: ["length"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  missingNestedField: {
    field: "metadata",
    // @ts-expect-error JSON path declarations must name real nested object fields
    path: ["missing"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  stringArrayIndex: {
    field: "metadata",
    // @ts-expect-error JSON path array crossings require numeric indexes
    path: ["labels", "0"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  invalidPresence: {
    field: "metadata",
    path: ["priority"],
    // @ts-expect-error JSON path presence accepts only required or optional
    presence: "maybe",
  },
});

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

const taskEnvelopeResponse = signals.resource.response.objectItems<TaskEnvelope>()({
  field: "tasks",
  itemId: (item) => item.id,
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
  }),
});

const taskEnvelope = signals.api({}).url("/task-page")
  .response(taskEnvelopeResponse)
  .list({
    load: () => ({
      tasks: [{
        id: "t1",
        title: "Task",
        status: "open" as const,
        metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
      }],
      nextCursor: null,
    }),
  });

tasks.patch.itemAspect({
  itemId: "t1",
  // @ts-expect-error response contract patch helpers must reject unknown aspects
  aspect: "missing",
  value: "wrong",
});

tasks.patch.itemAspect({
  itemId: "t1",
  aspect: "status",
  // @ts-expect-error response contract aspects preserve field value types
  value: "blocked",
});

taskEnvelope.patch.itemAspect({
  itemId: "t1",
  // @ts-expect-error typed envelope response contracts reject unknown aspects
  aspect: "status",
  value: "done",
});

signals.resource.response.objectItems<TaskEnvelope>()({
  // @ts-expect-error objectItems response contracts only accept array-valued fields
  field: "nextCursor",
  itemId: (item) => item.id,
});

signals.resource.response.collection<TaskEnvelope, Task>({
  itemId: (item) => item.id,
  items: (value) => value.tasks,
  // @ts-expect-error generic response contracts require replacement to preserve response shape
  replaceItems: (_value, nextItems) => nextItems,
});

signals.resource.response.collection<TaskEnvelope>()({
  itemId: (item: Task) => item.id,
  items: (value) => value.tasks,
  // @ts-expect-error curried generic response contracts require replacement to preserve response shape
  replaceItems: (_value, nextItems) => nextItems,
});

signals.api({}).url("/tasks").response(taskResponse).list({
  // @ts-expect-error response(...) owns itemIdentity(...) in the route lane
  itemIdentity: (item: Task) => item.id,
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

signals.api({}).url("/tasks").response(taskResponse).list({
  // @ts-expect-error response(...) owns reconcile(...) in the route lane
  reconcile: resourceCollectionShape({
    items: (value: readonly Task[]) => value,
    replaceItems: (_value: readonly Task[], nextItems: readonly Task[]) => [
      ...nextItems,
    ],
  }),
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

// @ts-expect-error response(...) owns aspect definitions through the contract
signals.api({}).url("/tasks").response(taskResponse).aspect(
  "title",
  (item: Task) => item.title,
  (item: Task, title: string) => ({ ...item, title }),
);

// @ts-expect-error response(...) does not declare summary contracts in this lane
signals.api({}).url("/tasks").response(taskResponse).summary(
  "total",
  (value: readonly Task[]) => value.length,
  (value: readonly Task[]) => value,
);

// @ts-expect-error response(...) does not declare page-window summary contracts in this lane
signals.api({}).url("/tasks").response(taskResponse).pageWindowSummary(
  "visible",
  (value: readonly Task[]) => value.length,
  (value: readonly Task[]) => value,
);

// @ts-expect-error response(...) is a collection lane and must not expose detail finalizers
signals.api({}).url("/tasks").response(taskResponse).detail({
  load: () => ({
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }),
});
