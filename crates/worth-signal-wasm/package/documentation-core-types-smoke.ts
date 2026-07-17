import { createSignals } from "./index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const quantity = signals.input(2, { debugName: "quantity" });
const unitPrice = signals.input(18, { debugName: "unitPrice" });
const total = signals.computed(() => quantity() * unitPrice(), {
  debugName: "total",
});

await signals.transaction((tx) => {
  tx.set(quantity, 4);
  tx.set(unitPrice, 20);
});

interface ShippingOption {
  id: string;
  label: string;
}

const shippingOptions = signals.input<ShippingOption[]>([
  { id: "ground", label: "Ground" },
  { id: "air", label: "Air" },
]);
const selectedShipping = signals.linked<ShippingOption[], ShippingOption | null>({
  source: () => shippingOptions(),
  computation: (options, previous) =>
    options.find((option) => option.id === previous?.value?.id) ?? options[0] ?? null,
});

selectedShipping.set({ id: "air", label: "Air" });
selectedShipping.relink();

const pricing = signals.graph("documentationPricing", (graph) => {
  const state = graph.scope("state");
  const graphQuantity = state.input(2);
  const graphUnitPrice = state.input(18);
  const graphTotal = state.computed(() => graphQuantity() * graphUnitPrice());

  return graph.expose({
    inputs: { quantity: graphQuantity, unitPrice: graphUnitPrice },
    outputs: { total: graphTotal },
  });
});

await pricing.writeInput("quantity", 4);
pricing.read().total satisfies number;
total() satisfies number;

const TransferAspect = {
  financialTerms: 0,
  operatorNote: 1,
} as const;

interface Transfer {
  amount: number;
  note: string;
}

const transfer = signals.spec.input<Transfer>(
  "documentationTransfer",
  { amount: 8_000, note: "Standard vendor invoice" },
  {
    producesAspects: [
      TransferAspect.financialTerms,
      TransferAspect.operatorNote,
    ],
  },
);

const reviewLane = signals.spec.computed<string>("documentationReviewLane", {
  reads: [{ id: transfer.id, aspect: TransferAspect.financialTerms }],
  expr: {
    kind: "if",
    condition: {
      kind: "gte",
      left: {
        kind: "get",
        target: { kind: "read", id: transfer.id },
        field: "amount",
      },
      right: { kind: "value", value: 10_000 },
    },
    thenExpr: { kind: "value", value: "Manual review" },
    elseExpr: { kind: "value", value: "Automatic" },
  },
  identity: { kind: "exact" },
});

reviewLane() satisfies string;

await signals.transaction((tx) => {
  tx.setWithAspects(
    transfer,
    { ...transfer(), note: "Urgent vendor invoice" },
    [TransferAspect.operatorNote],
  );
});

signals.diagnostics().latestFlow()?.flow.change.changed_aspects satisfies ReadonlyArray<number> | undefined;

signals.free();
