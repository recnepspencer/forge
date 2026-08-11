import {
  applyCommittedWorkerFirstAuthoredInputs,
  buildAuthoredInputMutationOperation,
  isWorkerFirstAuthoredInputPublicationReady,
  writeWorkerFirstAuthoredInputBaseline,
} from "./worker_first_authored_input_state.js";
import { materializeWorkerCachedValue } from "../worker_cached_value.js";

/** Begin an authored input mutation with epoch-gated tip rollback. */
export function beginWorkerFirstAuthoredInputMutation(authoredInputs, id, mutation) {
  const authoredInput = authoredInputs.get(id);
  if (
    !authoredInput
    || authoredInput.invalidatedMessage !== null
    || authoredInput.publicationState === "failed"
  ) {
    throw new TypeError(
      `worker-first inputAsync(...) can mutate only currently available worker-first authored inputs; \`${id}\` is not currently available`,
    );
  }
  const previousValue = authoredInput.currentValue;
  const previousEpoch = authoredInput.hostTipEpoch ?? 0;
  const epochAtWrite = previousEpoch + 1;
  const transactionOp = {
    ...buildAuthoredInputMutationOperation(id, mutation, authoredInput),
    epochAtWrite,
  };
  authoredInput.hostTipEpoch = epochAtWrite;
  authoredInput.currentValue = materializeWorkerCachedValue(transactionOp.value);
  return Object.freeze({
    transactionOps: [transactionOp],
    epochAtWrite,
    rollback() {
      if ((authoredInput.hostTipEpoch ?? 0) !== epochAtWrite) {
        return false;
      }
      authoredInput.currentValue = previousValue;
      authoredInput.hostTipEpoch = previousEpoch;
      return true;
    },
  });
}

export function requireWorkerFirstAuthoredInputPublicationReady(authoredInputs, id) {
  if (!authoredInputs.has(id)) {
    return;
  }
  if (!isWorkerFirstAuthoredInputPublicationReady(authoredInputs, id)) {
    const authoredInput = authoredInputs.get(id);
    const detail = authoredInput?.invalidatedMessage
      ?? (authoredInput?.publicationState === "pending"
        ? "background publication has not completed"
        : "it is not currently available");
    throw new TypeError(
      `worker-first authored input \`${id}\` cannot be mutated on the worker because ${detail}`,
    );
  }
}

export function applyCommittedWorkerFirstAuthoredInputOps(authoredInputs, transactionOps) {
  applyCommittedWorkerFirstAuthoredInputs(authoredInputs, transactionOps);
}

export function writeWorkerFirstAuthoredInputBaselineValue(authoredInputs, id, value) {
  writeWorkerFirstAuthoredInputBaseline(authoredInputs, id, value);
}
