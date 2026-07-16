import { freezeObject } from "../graph_support.js";
import { createResourceNamespace } from "../resource/facade.js";
import { readRootHistoryFacade } from "./worker_first_root_history.js";

export function createWorkerFirstResourceNamespace(
  signalNamespace,
  rootSession,
) {
  const rawSignals = freezeObject({
    history() {
      return readRootHistoryFacade(rootSession);
    },
  });
  return createResourceNamespace(signalNamespace, rawSignals);
}
