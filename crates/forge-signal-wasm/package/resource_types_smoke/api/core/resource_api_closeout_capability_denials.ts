import { createSignals } from "../../../index.js";

const signals = createSignals();

const detailLine = signals.api({}).url("/reports/:reportId").detail({
  load: ({ reportId }) => ({ id: reportId }),
}).line({ reportId: "r1" });

// @ts-expect-error detail DX lines must not overclaim patch(...)
detailLine.patch;
// @ts-expect-error detail DX lines must not overclaim deliver(...)
detailLine.deliver;
// @ts-expect-error detail DX lines must not overclaim reconciliation()
detailLine.reconciliation;

const plainTaskFamily = signals.api({}).url("/tasks").list({
  itemIdentity: (item: { id: string }) => item.id,
  load: () => [{ id: "t1" }],
});

// @ts-expect-error plain list families must not overclaim narrow family patch helpers
plainTaskFamily.patch.item({
  itemId: "t1",
  nextItem: { id: "t1" },
});
// @ts-expect-error plain list families must not overclaim narrow family delivery helpers
plainTaskFamily.delivery.item({
  packetId: "pkt-1",
  itemId: "t1",
  nextItem: { id: "t1" },
});

const arrayTaskFamily = signals.api({}).url("/tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: () => [{ id: "t1", title: "First" }],
  });

// @ts-expect-error direct-array lanes without declared aspects must not overclaim aspect patch helpers
arrayTaskFamily.patch.itemAspect({
  itemId: "t1",
  aspect: "title",
  value: "Updated",
});

const pagedTaskFamily = signals.api({}).url("/tasks/feed")
  .items((item: { id: string }) => item.id)
  .paged({
    accumulatePage: (existing, next) => [...existing, ...next],
    load: () => [{ id: "t1" }],
  });

// @ts-expect-error paged families without page-window summaries must not overclaim summary patch helpers
pagedTaskFamily.patch.summary({
  summary: "visibleCount",
  value: 1,
});
// @ts-expect-error paged families without page-window summaries must not overclaim summary delivery helpers
pagedTaskFamily.delivery.summary({
  packetId: "pkt-1",
  basisId: null,
  summary: "visibleCount",
  value: 1,
});

const advancedRoute = signals.api({}).url("/reports/:reportId")
  .verb("POST")
  .body<{ publish: boolean }>();

// @ts-expect-error explicit advanced shaping must keep standard write finalizers unavailable
advancedRoute.create({
  load: ({ reportId, body }: { reportId: string; body: { publish: boolean } }) => ({
    reportId,
    body,
  }),
});

const stackedTransfer = signals.api({}).url("/uploads")
  .signedUpload();

// @ts-expect-error upload builders must not stack in one route lane during closeout too
stackedTransfer.multipartUpload();
