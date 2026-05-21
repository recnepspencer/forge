import { createSignals } from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const api = signals.api({});

const finalizeReceipt = api.url("/receipts/:receiptId/finalize")
  .verb("POST")
  .body<{ amount: number }>()
  .detail({
    load: ({ receiptId, body }) => ({
      id: String(receiptId),
      submittedAmount: body.amount,
    }),
  });

const deleteReceipt = api.url("/receipts/:receiptId")
  .verb("DELETE")
  .detail({
    load: ({ receiptId }) => ({
      removedId: String(receiptId),
    }),
  });

const reportHeaders = api.url("/reports/:reportId")
  .headers(({ reportId }) => ({
    "x-report-id": String(reportId),
  }))
  .detail({
    load: ({ reportId }) => ({ id: String(reportId) }),
  });

const searchTasks = api.url("/workspaces/:workspaceId/tasks/search")
  .items((item: { id: string }) => item.id)
  .verb("POST")
  .body<{ query: string }>()
  .paged({
    accumulatePage: (existing: readonly { id: string }[], next: readonly { id: string }[]) => [...existing, ...next],
    load: ({ workspaceId, body }) => [{ id: `${workspaceId}:${body.query}` }],
  });

const taskHeaders = api.url("/workspaces/:workspaceId/tasks")
  .items((item: { id: string }) => item.id)
  .headers(({ workspaceId }) => ({
    "x-workspace-id": String(workspaceId),
  }))
  .list({
    load: ({ workspaceId }) => [{ id: `${workspaceId}:t1` }],
  });

const fluentCatalog = api.url("/workspaces/:workspaceId/catalog")
  .items((item: { id: string }) => item.id)
  .reconcile(
    (value: { items: { id: string }[] }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .headers(({ workspaceId }) => ({
    "x-workspace-id": String(workspaceId),
  }))
  .list({
    load: ({ workspaceId }) => ({ items: [{ id: `${workspaceId}:c1` }] }),
  });

const searchCatalog = api.url("/workspaces/:workspaceId/catalog/search")
  .items((item: { id: string }) => item.id)
  .reconcile(
    (value: { items: { id: string }[] }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .verb("POST")
  .body<{ query: string }>()
  .paged({
    accumulatePage: (
      existing: { items: readonly { id: string }[] },
      next: { items: readonly { id: string }[] },
    ) => ({
      ...next,
      items: [...existing.items, ...next.items],
    }),
    load: ({ workspaceId, body }) => ({
      items: [{ id: `${workspaceId}:${body.query}` }],
    }),
  });

void finalizeReceipt.line({
  receiptId: "r1",
  body: { amount: 42 },
}).request();
void deleteReceipt.line({ receiptId: "r2" }).request();
void reportHeaders.line({ reportId: "r3" }).request();
void searchTasks.line({ workspaceId: "demo", body: { query: "open" } }).request();
void taskHeaders.line({ workspaceId: "demo" }).request();
void fluentCatalog.line({ workspaceId: "demo" }).request();
void searchCatalog.line({ workspaceId: "demo", body: { query: "open" } }).request();
