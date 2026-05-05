import { createSignals, resourceParams } from "../index.js";

const signals = createSignals();

const detail = signals.api({}).url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId }),
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
const exportReport = signals.api({}).url("/reports/export:csv").detail({
  load: () => ({ ok: true }),
});

detail.line({ userId: "u1" });
home.line({});
search.line({ params: {} });
exportReport.line({});

// @ts-expect-error required route param must remain mandatory at line(...)
detail.line({});

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

// @ts-expect-error list finalizer must still require itemIdentity
signals.api({}).url("/tasks").list({
  load: () => [{ id: "t1" }],
});

// @ts-expect-error paged finalizer must still require accumulatePage
signals.api({}).url("/tasks").paged({
  itemIdentity: (item: { id: string }) => item.id,
  load: () => [{ id: "t1" }],
});
