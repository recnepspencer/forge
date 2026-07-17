import {
  createCallableSignals,
  createSignals,
  explainCreateSignalsConstruction,
  planCreateSignalsDeployment,
} from "./index.js";
import type {
  CallableSignals,
  SignalsConstructionArtifact,
  SignalsRuntimeCapabilityName,
  SignalsRuntimeContract,
} from "./index.js";

const explanation = explainCreateSignalsConstruction();
explanation.requestedDeployment satisfies "workerFirst" | "mainThreadCompatibility";
explanation.selectedFamily satisfies
  | "workerFirst"
  | "mainThreadCompatibility"
  | "workerUnavailable"
  | "denied";
explanation.selectedDeployment satisfies
  | "workerFirst"
  | "mainThreadCompatibility"
  | null;
explanation.reason satisfies string;

const plan = planCreateSignalsDeployment({
  deployment: "mainThreadCompatibility",
});
plan.explanation.selectedDeployment satisfies
  | "workerFirst"
  | "mainThreadCompatibility"
  | null;

const requiredCapabilities: ReadonlyArray<SignalsRuntimeCapabilityName> = [
  "callableSurface",
  "scopedAuthoring",
  "workerRuntime",
];

const signals: CallableSignals = await createSignals();
const contract: SignalsRuntimeContract = signals.contract();
contract.surfaceVersion satisfies "1";
signals.assertCompatibility({ requires: requiredCapabilities });

const quantity = signals.input(2, { debugName: "quantity" });
const unitPrice = signals.input(18, { debugName: "unitPrice" });
const total = signals.computed(() => quantity() * unitPrice());

await signals.transaction((tx) => {
  tx.set(quantity, 4);
  tx.set(unitPrice, 20);
});
total() satisfies number;

const billing = signals.scope("billing");
const invoice = billing.graph("invoice", (graph) => {
  const state = graph.scope("state");
  const amount = state.input(0);
  const publishedTotal = state.output(() => amount());
  return graph.expose({
    inputs: { amount },
    outputs: { total: publishedTotal },
  });
});
invoice.read().total satisfies number;

const compatibility = await createCallableSignals({
  deployment: "workerFirst",
});
compatibility.contract().deployment satisfies "workerFirst" | "mainThreadCompatibility";

function isConstructionArtifact(
  value: unknown,
): value is Error & SignalsConstructionArtifact {
  return value instanceof Error && "artifactFamily" in value;
}

async function constructWithTypedFailure() {
  try {
    return await createSignals();
  } catch (error) {
    if (!isConstructionArtifact(error)) throw error;
    error.compatibilityRecovery.deployment satisfies "mainThreadCompatibility";
    return null;
  }
}

// @ts-expect-error unsupported deployment names must not type-check.
void createSignals({ deployment: "automaticFallback" });

void constructWithTypedFailure;
compatibility.free();
signals.free();
