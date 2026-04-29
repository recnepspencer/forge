import {
  createSignals,
  type ComputedSpec,
  type InputSignalHandle,
  type OutputSpec,
  type Signal,
} from "./index.js";

const signals = createSignals();

const count: InputSignalHandle<number> = signals.input("count", 1);
const next: number = count();
const alsoNext: number = count.get();
const commit = count.set(next + alsoNext);

const doubledSpec: ComputedSpec = {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
};

const doubled: Signal<number> = signals.computedSpec<number>("doubled", doubledSpec);
const doubledFromCallback: Signal<number> = signals.computed<number>(
  "doubledCallback",
  () => count() * 2,
);
const constantFromCallback: Signal<number> = signals.computed<number>(
  "constantCallback",
  () => 2,
);
const generatedFromCallback: Signal<number> = signals.computed<number>(() => 3, { id: "three" });

const panelSpec: OutputSpec = {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
};

const panel = signals.outputSpec<{ count: number; doubled: number }>("panel", panelSpec);
const snapshot = panel();
const deferredOutput: never = signals.output("panelDeferred", () => snapshot);
const explicitDeferredOutput: never = signals.outputCallback("panelDeferredToo", () => snapshot);
const adapters = signals.adapters();
const definitions = adapters.exportDefinitions();
const proof = adapters.runtimeProofReport();
const maybeUnavailable = definitions.unavailableCallbacks.map(
  (artifact) => artifact.signalKind,
);

signals.transaction((tx) => {
  tx.set(count, snapshot.count + commit.touchedNodes);
  // @ts-expect-error computed handles must stay read-only inside transactions
  tx.set(doubled, 4);
});

// @ts-expect-error branded callable handles must not accept structural forgeries
const forgedSignal: InputSignalHandle<number> = {
  id: "forged",
  get() {
    return 1;
  },
  set() {
    return commit;
  },
};

void constantFromCallback;
void doubledFromCallback;
void generatedFromCallback;
void deferredOutput;
void explicitDeferredOutput;
void definitions;
void maybeUnavailable;
void proof;
void forgedSignal;
