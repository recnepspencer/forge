export const DEMO_ONE_CODE = `import { createSignals } from "worth-signals-wasm";
import wasmUrl from "worth-signals-wasm/wasm?url";

const signals = await createSignals({
  deployment: "mainThreadCompatibility",
  assets: { wasmUrl },
});

const amount = signals.input(8_000);
const fee = signals.computed(() => amount() * 0.004);
const reviewLane = signals.computed(() =>
  amount() >= 10_000 ? "Manual review" : "Automatic"
);

// commit one decision, not three unrelated setters
await signals.transaction((tx) => tx.set(amount, 12_000));

// ask the runtime that made the decision to explain it
await signals.diagnostics().why(reviewLane.id);`;

export const DEMO_TWO_CODE = `const api = signals.api({ baseUrl: "/api/storefront" });

const product = api.url("/products/:productId").detail({
  reconcile: signals.resource.detailFields({
    price: { read: (v) => v.price, write: (v, price) => ({ ...v, price }) },
  }),
  load: ({ productId }) => fetchProduct(productId),
});

const line = product.line({ productId: "p-204" });

line.deliver(product.delivery.field({
  packetId: "pkt-08", basisId: "srv-v1", nextBasisId: "srv-v2",
  field: "price", value: 188,
}));

line.diagnostics().lastEffect.provenance;
line.history().lifecycle;`;

export const DEMO_THREE_CODE = `const form = signals.form({
  source: payoutPolicy,
  collaboration: {
    mode: "fieldLease",
    actorId: session.userId,
    supportsPresence: true,
    supportsComments: true,
  },
  fields: ({ field }) => ({
    limit: field("limit"),
    justification: field("justification"),
  }),
});

channel.on("collaboration", (event) => {
  form.reportCollaboration({
    posture: event.posture,
    leasedFields: event.leases,
    presence: event.presence,
  });
});

form.fieldWritePosture("limit");
form.readiness();`;

export const DEMO_FOUR_CODE = `const stepExecution = signals.router.prerequisite(
  "stepExecution",
  async ({ facts, allow, forbidden }) => {
    if (facts.trainedRev !== facts.effectiveRev) {
      return forbidden({
        reason: "trainingSupersededByRevision",
        detail: \`Trained on rev \${facts.trainedRev}; effective is \${facts.effectiveRev}.\`,
      });
    }
    return allow({ reason: "trainingCurrent" });
  },
);

const routes = signals.router.define({
  stepExecute: signals.router.route("/batches/:batchId/steps/:stepId", {
    admission: [stepExecution],
    resources: {
      page: signals.router.resourceLine(stepFamily, { prefetch: "intent" }),
    },
  }),
});

const ingress = signals.router.browserHistory.push(href);
const report = await routes.admitBrowserHistoryIngress(ingress, session.facts);
story.record(report);`;

export const DEMO_FIVE_CODE = `export async function savePoLine(nextItem, parentEffectIds = []) {
  const insert = poLines.patch.insert({
    itemId: nextItem.id,
    placement: "append",
    nextItem,
  });
  const admission = await line.patch(
    resourcePatch.dependsOn(insert, parentEffectIds),
  );
  if (!("effectId" in admission)) return admission;

  try {
    const saved = await client.saveLine(nextItem);
    return line.effects().confirm(admission.effectId, {
      responseId: saved.requestId,
      serverPatch: poLines.patch.insert({
        itemId: saved.line.id,
        placement: "append",
        nextItem: saved.line,
      }),
    });
  } catch (failure) {
    return line.effects().reject(admission.effectId, {
      responseId: failure.responseId,
    });
  }
}`;
