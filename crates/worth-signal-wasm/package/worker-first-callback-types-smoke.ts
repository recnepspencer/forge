import {
  createSignals,
  type Signal,
} from "./index.js";

const signals = await createSignals();
const count = signals.input(1);
const rootComputed: Signal<number> = signals.computedCallback(
  "rootComputed",
  () => count() * 2,
);
const scopedComputed: Signal<number> = signals.scope("wizard").computedCallback(
  "scopedComputed",
  () => rootComputed() + 1,
);
const scopedDescriptor = signals.scope("wizard").descriptor();
const scopedCanonicalId: string = signals.scope("wizard").canonicalId("scopedComputed");
const scopedIdentity = signals.scope("wizard").signalIdentity("scopedComputed");
const panel: Signal<{ value: number }> = signals.outputCallback(
  "panel",
  () => ({ value: scopedComputed() }),
);

rootComputed();
scopedComputed();
panel();
scopedDescriptor.identity.scopeId;
scopedCanonicalId.toUpperCase();
scopedIdentity.canonicalId.toUpperCase();
