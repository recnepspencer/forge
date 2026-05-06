import {
  createSignals,
  resourceCollectionShape,
  resourceProcessingJob,
  resourceParams,
  resourceUploadTransport,
  resourceValueSummaries,
} from "../index.js";

const signals = createSignals();

const detail = signals.api({}).url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId }),
});
const createUser = signals.api({}).url("/users").create({
  load: ({ body }: { body: { name: string } }) => ({ name: body.name }),
});
const updateUser = signals.api({}).url("/users/:userId").update({
  load: ({ userId, body }: { userId: string; body: { name: string } }) => ({
    id: userId,
    name: body.name,
  }),
});
const removeUser = signals.api({}).url("/users/:userId").remove({
  load: ({ userId }) => ({ removed: userId }),
});
const home = signals.api({}).url("/").detail({
  load: () => ({ ok: true }),
});
const search = signals.api({}).url("/users").params<{
  search?: string;
  page?: number;
}>().list({
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ params }) => [{ id: `${params.search ?? "all"}:${params.page ?? 1}` }],
});
const taskList = signals.api({}).url("/tasks").list({
  itemIdentity: (item: { id: string }) => item.id,
  load: () => [{ id: "t1" }],
});
const directTaskList = signals.api({}).url("/direct-tasks")
  .items((item: { id: string }) => item.id)
  .list({
    load: () => [{ id: "t1" }],
  });
const directTaskCatalog = signals.api({}).url("/direct-task-catalog")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  )
  .summary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  .list({
    load: () => [{ id: "t1", title: "Task" }],
  });
const directTaskLineSummaryPages = signals.api({}).url("/direct-task-line-pages")
  .items((item: { id: string; title: string }) => item.id)
  .summary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  .paged({
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: () => [{ id: "t1", title: "Task" }],
  });
const fluentTaskCatalog = signals.api({}).url("/fluent-task-catalog")
  .items((item: { id: string; title: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string; title: string }>; total: number }) =>
      value.items,
    (
      value: { items: Array<{ id: string; title: string }>; total: number },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  )
  .aspect(
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  )
  .summary(
    "total",
    (value: { items: Array<{ id: string; title: string }>; total: number }) =>
      value.total,
    (
      value: { items: Array<{ id: string; title: string }>; total: number },
      total: number,
    ) => ({ ...value, total }),
  )
  .list({
    load: () => ({ items: [{ id: "t1", title: "Task" }], total: 1 }),
  });
const pagedTaskFeed = signals.api({}).url("/tasks/feed").paged({
  itemIdentity: (item: { id: string }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string }>; total: number },
    { id: string },
    {},
    {
      total: {
        read(value: { items: Array<{ id: string }>; total: number }): number;
        write(
          value: { items: Array<{ id: string }>; total: number },
          total: number,
        ): { items: Array<{ id: string }>; total: number };
      };
    }
  >({
    items: (value: { items: Array<{ id: string }>; total: number }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string }>; total: number },
      nextItems: readonly { id: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    summaries: resourceValueSummaries({
      total: {
        read: (value: { items: Array<{ id: string }>; total: number }) =>
          value.total,
        write: (
          value: { items: Array<{ id: string }>; total: number },
          total: number,
        ) => ({ ...value, total }),
      },
    }),
  }),
  accumulatePage: (
    existing: { items: Array<{ id: string }>; total: number },
    next: { items: Array<{ id: string }>; total: number },
  ) => ({
    items: [...existing.items, ...next.items],
    total: next.total,
  }),
  load: () => ({ items: [{ id: "t1" }], total: 1 }),
});
const exportReport = signals.api({}).url("/reports/export:csv").detail({
  load: () => ({ ok: true }),
});

detail.line({ userId: "u1" });
createUser.line({ body: { name: "Ada" } });
updateUser.line({ userId: "u1", body: { name: "Grace" } });
removeUser.line({ userId: "u1" });
home.line({});
search.line({ params: {} });
exportReport.line({});

// @ts-expect-error required route param must remain mandatory at line(...)
detail.line({});

// @ts-expect-error path params must not admit booleans
detail.line({ userId: true });

// @ts-expect-error create(...) lines must require a body
createUser.line({});

// @ts-expect-error update(...) lines must require a body in addition to path params
updateUser.line({ userId: "u1" });

// @ts-expect-error remove(...) lines must not admit a body
removeUser.line({ userId: "u1", body: { hard: true } });

// @ts-expect-error undeclared request params must stay denied before params(...) exists
detail.line({ userId: "u1", search: "ada" });

const extraRouteParams = { userId: "u1", search: "ada" };

// @ts-expect-error undeclared route params must be denied through variables too
detail.line(extraRouteParams);

// @ts-expect-error invalidate(...) must reject undeclared params through variables too
detail.invalidate(extraRouteParams);

// @ts-expect-error literal colon text must not create a phantom route param
exportReport.line({ csv: "x" });

// @ts-expect-error root routes must not admit phantom path params
home.line({ anything: "nope" });

// @ts-expect-error declared request-param routes require an explicit params object
search.line({});

// @ts-expect-error declared request-param routes keep path and request params separate
search.line({ search: "ada" });

// @ts-expect-error nested request params must stay exact through object literals
search.line({ params: { search: "ada", nope: true } });

const extraSearchParams = {
  params: {
    search: "ada",
    nope: true,
  },
};

// @ts-expect-error nested request params must stay exact through variables too
search.line(extraSearchParams);

// @ts-expect-error invalidate(...) must keep nested request params exact too
search.invalidate(extraSearchParams);

// @ts-expect-error unreconciled list families must not overclaim narrow patch helpers
taskList.patch.item({
  itemId: "t1",
  nextItem: { id: "t1" },
});

// @ts-expect-error unreconciled list families must not overclaim narrow delivery helpers
taskList.delivery.item({
  packetId: "pkt-task",
  itemId: "t1",
  nextItem: { id: "t1" },
});

// @ts-expect-error items(...) array lane must not overclaim aspect patch helpers without explicit reconcile DSL
directTaskList.patch.itemAspect({
  itemId: "t1",
  aspect: "title",
  value: "wrong",
});

// @ts-expect-error items(...) array lane must not overclaim summary delivery helpers without explicit reconcile DSL
directTaskList.delivery.summary({
  packetId: "pkt-direct-summary",
  summary: "total",
  value: 1,
});

directTaskCatalog.patch.itemAspect({
  itemId: "t1",
  // @ts-expect-error direct-array aspect lanes must reject unknown aspect names
  aspect: "missing",
  value: "wrong",
});

// @ts-expect-error paged direct-array line summaries must not overclaim summary delivery helpers
directTaskLineSummaryPages.delivery.summary({
  packetId: "pkt-direct-line-summary",
  summary: "count",
  value: 1,
});

fluentTaskCatalog.patch.summary({
  // @ts-expect-error fluent reconcile lanes must reject unknown summary names
  summary: "count",
  value: 1,
});

// @ts-expect-error paged families with line-scoped summaries must not overclaim summary delivery helpers
pagedTaskFeed.delivery.summary({
  packetId: "pkt-summary",
  summary: "total",
  value: 2,
});

// @ts-expect-error routes must start with /
signals.api({}).url("users/:userId");

// @ts-expect-error routes must not contain trailing slash segments
signals.api({}).url("/users/");

// @ts-expect-error routes must not contain empty segments
signals.api({}).url("/users//roles");

// @ts-expect-error param placeholders must start with a letter or underscore
signals.api({}).url("/users/:1bad");

// @ts-expect-error param placeholders must use only letters, digits, and underscores
signals.api({}).url("/users/:user-id");

// @ts-expect-error param placeholders must stay unique
signals.api({}).url("/users/:userId/:userId");

signals.api({
  baseUrl: "/api",
});

signals.api({
  // @ts-expect-error scoped baseUrl must stay string-or-function typed
  baseUrl: 7,
});

// @ts-expect-error request params lane must not collide with a :params path placeholder
signals.api({}).url("/reports/:params").params<{ search?: string }>();

signals.api({
  // @ts-expect-error scoped defaults must not admit unknown fields
  nope: true,
});

signals.api({}).url("/users/:userId").detail({
  // @ts-expect-error route-first finalizers own params(...) in the common lane
  params: resourceParams<{ userId: string }>(),
  load: ({ userId }) => ({ id: userId }),
});

// @ts-expect-error items(...) is only the list-shaped lane and must not expose detail finalizers
signals.api({}).url("/tasks").items((item: { id: string }) => item.id).detail({
  load: () => ({ id: "t1" }),
});

signals.api({}).url("/tasks").items((item: { id: string }) => item.id).list({
  // @ts-expect-error items(...) owns itemIdentity(...) in the direct-array lane
  itemIdentity: (item: { id: string }) => item.id,
  load: () => [{ id: "t1" }],
});

signals.api({}).url("/tasks").items((item: { id: string }) => item.id).list({
  // @ts-expect-error items(...) owns reconcile(...) in the direct-array lane
  reconcile: resourceCollectionShape({
    items: (value: Array<{ id: string }>) => value,
    replaceItems: (
      _value: Array<{ id: string }>,
      nextItems: readonly { id: string }[],
    ) => [...nextItems],
  }),
  load: () => [{ id: "t1" }],
});

signals.api({}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  )
  .aspect(
    // @ts-expect-error duplicate direct-array aspect names must stay uncallable
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  );

signals.api({}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .summary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  // @ts-expect-error direct-array summary scopes must not mix in one fluent lane
  .pageWindowSummary(
    "windowCount",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  );

signals.api({}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .summary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  // @ts-expect-error reconcile(...) must be declared before summary ownership changes the outer value shape
  .reconcile(
    (value: { items: Array<{ id: string; title: string }> }) => value.items,
    (
      value: { items: Array<{ id: string; title: string }> },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  );

signals.api({}).url("/users").create({
  // @ts-expect-error route-first write finalizers own method selection in the common lane
  method: "POST",
  load: ({ body }: { body: { name: string } }) => ({ name: body.name }),
});

signals.api({}).url("/users").create({
  // @ts-expect-error route-first write finalizers own requestBody(...) in the common lane
  requestBody: ({ body }: { body: { name: string } }) => body,
  load: ({ body }: { body: { name: string } }) => ({ name: body.name }),
});

// @ts-expect-error list finalizer must still require itemIdentity
signals.api({}).url("/tasks").list({
  load: () => [{ id: "t1" }],
});

// @ts-expect-error paged finalizer must still require accumulatePage
signals.api({}).url("/tasks").paged({
  itemIdentity: (item: { id: string }) => item.id,
  load: () => [{ id: "t1" }],
});
