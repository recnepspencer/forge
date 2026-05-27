import { createSignals } from "./index.js";

const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});

const rootContract = signals.contract();
const rootSurfaceFamily:
  | "mainThreadCompatibilityCallable"
  | "mainThreadCompatibilityScoped"
  | "workerFirstCallable"
  | "workerFirstScoped" = rootContract.surfaceFamily;
const rootSurfaceVersion: "1" = rootContract.surfaceVersion;
const rootCallableCapability: boolean = rootContract.capabilities.callableSurface;

signals.assertCompatibility({
  requires: ["callableSurface", "scopedAuthoring", "specNamespace"],
});

const scopedContract = signals.scope("admin").contract();
const scopedScopeId: string | null = scopedContract.scopeId;
const scopedSpecNamespaceCapability: boolean =
  scopedContract.capabilities.specNamespace;

void rootSurfaceFamily;
void rootSurfaceVersion;
void rootCallableCapability;
void scopedScopeId;
void scopedSpecNamespaceCapability;

await signals.terminate();

