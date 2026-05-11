import { createSignals } from "../../../index.js";

const signals = createSignals();

signals.api({}).url("/reports/:reportId")
  .verb("POST")
  .body<{ amount: number }>()
  .detail({
    // @ts-expect-error builder-owned body lane forbids raw requestBody restatement
    requestBody: (params) => params.body,
    load: ({ reportId, body }) => ({ reportId, body }),
  });

signals.api({}).url("/reports/:reportId")
  .headers({ "x-report-id": "r1" })
  .detail({
    // @ts-expect-error builder-owned headers lane forbids raw headers restatement
    headers: { "x-extra": "1" },
    load: ({ reportId }) => ({ reportId }),
  });

// @ts-expect-error standard create/update/remove finalizers are unavailable after explicit verb(...)
signals.api({}).url("/reports/:reportId").verb("POST").create({
  load: ({ reportId, body }: { reportId: string; body: { amount: number } }) => ({
    reportId,
    body,
  }),
});

signals.api({}).url("/tasks/search")
  .items((item: { id: string }) => item.id)
  .body<{ query: string }>()
  .list({
    // @ts-expect-error collection-owned body lane forbids raw requestBody restatement
    requestBody: (params) => params.body,
    load: ({ body }) => [{ id: body.query }],
  });

signals.api({}).url("/tasks")
  .items((item: { id: string }) => item.id)
  .headers({ "x-tasks": "1" })
  .list({
    // @ts-expect-error direct-array headers lane forbids raw headers restatement
    headers: { "x-extra": "1" },
    load: () => [{ id: "t1" }],
  });

signals.api({}).url("/catalog")
  .items((item: { id: string }) => item.id)
  .reconcile(
    (value: { items: { id: string }[] }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .headers({ "x-catalog": "1" })
  .list({
    // @ts-expect-error collection-owned headers lane forbids raw headers restatement
    headers: { "x-extra": "2" },
    load: () => ({ items: [{ id: "c1" }] }),
  });

signals.api({}).url("/catalog/search")
  .items((item: { id: string }) => item.id)
  .reconcile(
    (value: { items: { id: string }[] }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .body<{ query: string }>()
  .paged({
    // @ts-expect-error reconcile body lane forbids raw requestBody restatement
    requestBody: (params) => params.body,
    accumulatePage: (existing, next) => next,
    load: ({ body }) => ({ items: [{ id: body.query }] }),
  });

// @ts-expect-error standard create/update/remove finalizers are unavailable after explicit advanced shaping on collection lanes
signals.api({}).url("/tasks/search").items((item: { id: string }) => item.id).verb("POST").create({
  load: ({ body }: { body: { query: string } }) => ({ body }),
});
