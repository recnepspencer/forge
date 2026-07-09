import {
  createRealResourceNamespace,
  createRealResourceRuntime,
} from "./real_resource_signals.mjs";

export async function createRealResourceTestRuntime(overrides = null) {
  const runtime = await createRealResourceRuntime();
  return {
    ...runtime,
    mod: runtime.resourceMod,
    resource: createRealResourceNamespace(
      runtime.resourceMod,
      runtime.signals,
      overrides,
    ),
    async cleanup() {
      await runtime.cleanup();
    },
  };
}
