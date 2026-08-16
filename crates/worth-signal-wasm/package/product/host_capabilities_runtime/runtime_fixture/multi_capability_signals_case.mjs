import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { buildHostRawSignals } from "./host_raw_signals.mjs";

export async function createMultiCapabilitySignalsCase() {
  const loaded = await loadSignalsModule();
  const {
    wrapSignals,
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
    cleanup,
  } = loaded;

  const calls = [];
  const runtimeState = { values: new Map() };
  const rawSignals = buildHostRawSignals(runtimeState, calls);
  rawSignals.diagnostics = () => ({
    why() {
      return null;
    },
    health() {
      return null;
    },
    summaryNow() {
      return { profile: "Development" };
    },
    historyNow() {
      return { history: {}, callbackNodes: [] };
    },
    latestObservation() {
      return null;
    },
    latestFlow() {
      return null;
    },
    performanceSummary() {
      return { activeHandleCount: 0 };
    },
    latestFailure() {
      return null;
    },
    latestRollback() {
      return null;
    },
    latestInvalidationPlanningEstimate() {
      return null;
    },
    latestInvalidationTraceRecords() {
      return [];
    },
    recentHistory() {
      return [];
    },
    subscribe() {
      return { free() {} };
    },
    free() {},
  });

  const state = {
    visibility: true,
    viewport: { width: 1280, height: 720 },
    online: true,
    clockTick: 0,
    persistedDraft: { mode: "draft", revision: 1 },
  };

  const signals = wrapSignals(rawSignals, {
    hostCapabilities: hostCapabilityPlan({
      visibility: visibilityCapability({
        source: {
          current() {
            return state.visibility;
          },
          subscribe() {
            return () => {};
          },
        },
      }),
      viewport: viewportCapability({
        source: {
          current() {
            return state.viewport;
          },
          subscribe() {
            return () => {};
          },
        },
      }),
      online: onlineCapability({
        source: {
          current() {
            return state.online;
          },
          subscribe() {
            return () => {};
          },
        },
      }),
      clock: clockCapability({
        source: {
          current() {
            return state.clockTick;
          },
        },
        pollMs: 5,
      }),
      persistence: persistenceCapability({
        source: {
          current() {
            return state.persistedDraft;
          },
        },
      }),
    }),
  });

  return {
    cleanup,
    rawSignals,
    calls,
    signals,
    state,
    wrapSignals,
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
  };
}
