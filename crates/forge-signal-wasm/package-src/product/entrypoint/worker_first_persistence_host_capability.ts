import {
  HOST_PERSISTENCE_HANDLE_BRAND,
} from "../host_capability_declarations.js";
import { freezeObject } from "../graph_support.js";
import { createAuthoredInputPublication } from "./sessions/support/worker_first_authored_input_state.js";
import { materializeWorkerCachedValue } from "./sessions/support/worker_cached_value.js";
import {
  recordFlushedEvent,
  recordIgnoredStaleEvent,
  recordNoOpEvent,
} from "./worker_first_host_capability_events.js";

export function createWorkerFirstPersistenceCapability(
  registration,
  rootSession,
  performanceSummary,
  diagnosticsRecorder,
) {
  const descriptor = freezeObject({
    family: "persistence",
    compatibility: registration.compatibility,
    registrationId: "persistence",
  });
  const hiddenSignalId = rootSession.nextGeneratedStandaloneSignalId("hostPersistence");
  let committedState = materializeWorkerCachedValue(registration.source.current());
  let disposed = false;

  return {
    handle: freezeObject({
      value() {
        performanceSummary.hostCapabilityReadCount += 1;
        return committedState;
      },
      commit() {
        return commitPersistence();
      },
      descriptor() {
        return descriptor;
      },
      [HOST_PERSISTENCE_HANDLE_BRAND]: true,
    }),
    async bootstrap() {
      await publishCurrentState();
    },
    async replayCurrentIngress() {
      if (!disposed) {
        await publishCurrentState();
      }
    },
    dispose() {
      if (!disposed) {
        disposed = true;
        performanceSummary.hostCapabilityDisposalCount += 1;
      }
    },
  };

  async function publishCurrentState() {
    await rootSession.bridge().publishPortableGraph(
      createAuthoredInputPublication(hiddenSignalId, committedState, {}),
    );
  }

  async function commitPersistence() {
    if (disposed) {
      recordIgnoredStaleEvent(
        performanceSummary,
        diagnosticsRecorder,
        descriptor,
        "manually-committed",
        committedState,
      );
      return { touchedNodes: 0, nodesRecomputed: 0 };
    }
    performanceSummary.hostCapabilityManualCommitCount += 1;
    const nextState = materializeWorkerCachedValue(registration.source.current());
    if (Object.is(JSON.stringify(nextState), JSON.stringify(committedState))) {
      performanceSummary.hostCapabilityNoOpManualCommitCount += 1;
      recordNoOpEvent(
        performanceSummary,
        diagnosticsRecorder,
        descriptor,
        "manually-committed",
        committedState,
        nextState,
      );
      return { touchedNodes: 0, nodesRecomputed: 0 };
    }
    performanceSummary.hostCapabilityInvalidationCount += 1;
    const previousState = committedState;
    const result = await rootSession.bridge().applyTransaction([{
      kind: "set",
      id: hiddenSignalId,
      value: nextState,
    }]);
    committedState = nextState;
    const touchedNodes = typeof result?.runSummary?.touchedNodes === "number"
      ? Math.max(0, result.runSummary.touchedNodes)
      : 0;
    const reevaluatedNodes = typeof result?.runSummary?.nodesRecomputed === "number"
      ? Math.max(0, result.runSummary.nodesRecomputed)
      : touchedNodes;
    performanceSummary.hostCapabilityReevaluationCount += reevaluatedNodes;
    performanceSummary.hostCapabilityInvalidationTouchedNodeCount += touchedNodes;
    recordFlushedEvent(
      performanceSummary,
      diagnosticsRecorder,
      descriptor,
      "manually-committed",
      previousState,
      nextState,
      touchedNodes,
      reevaluatedNodes,
    );
    return result.runSummary ?? { touchedNodes, nodesRecomputed: reevaluatedNodes };
  }
}
