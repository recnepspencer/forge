export type * from "./types.d.ts";
export type * from "./builders.d.ts";
export type * from "./surface.d.ts";

import { initForgeSignal, createSignalApp, createSignalRuntime } from "./init.ts";
import { expr as exprValue } from "./builders/expr.js";
import * as defineModule from "./builders/define.js";
import { tx as txValue } from "./builders/tx.js";
import { policy as policyValue } from "./builders/policy.js";
import type * as BuilderSurface from "./builders.d.ts";

const {
  define: defineValue,
  keyed: keyedValue,
  partition: partitionValue,
  SourceBuilder,
  RecipeBuilder,
  SourceFamilyBuilder,
  RecipeFamilyBuilder,
} = defineModule as typeof import("./builders/define.js") & {
  partition: typeof BuilderSurface.partition;
};

export { initForgeSignal, createSignalApp, createSignalRuntime };

export const expr: typeof BuilderSurface.expr = exprValue as typeof BuilderSurface.expr;
export const define: typeof BuilderSurface.define = defineValue as typeof BuilderSurface.define;
export const keyed: typeof BuilderSurface.keyed = keyedValue as typeof BuilderSurface.keyed;
export const partition: typeof BuilderSurface.partition = partitionValue as typeof BuilderSurface.partition;
export const tx: typeof BuilderSurface.tx = txValue as typeof BuilderSurface.tx;
export const policy: typeof BuilderSurface.policy = policyValue as typeof BuilderSurface.policy;
export {
  SourceBuilder,
  RecipeBuilder,
  SourceFamilyBuilder,
  RecipeFamilyBuilder,
};

export {
  SignalHandle,
  SourceHandle,
  RecipeHandle,
  SourceFamilyHandle,
  RecipeFamilyHandle,
  KeyedSourceHandle,
  KeyedRecipeHandle,
} from "./surface/handles.ts";
export { SignalApp } from "./surface/app.ts";
export { SignalRuntime } from "./surface/runtime.ts";
export { SignalDiagnostics } from "./surface/diagnostics.ts";
export { SignalHistory } from "./surface/history.ts";
export { SignalSpecialist } from "./surface/specialist.ts";
export { SignalAdapters } from "./surface/adapters.ts";
