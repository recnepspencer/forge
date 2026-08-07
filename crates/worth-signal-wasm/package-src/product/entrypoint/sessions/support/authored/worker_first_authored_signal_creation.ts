import {
  createAuthoredInputPublication,
  createWorkerFirstAuthoredInputState,
} from "./worker_first_authored_input_state.js";
import {
  createAuthoredReadablePublication,
  createWorkerFirstAuthoredReadableState,
  updateWorkerFirstAuthoredReadables,
} from "./worker_first_authored_readable_state.js";
import {
  createWorkerFirstAuthoredCallbackState,
  nextWorkerFirstCallbackBackingInputId,
} from "./worker_first_authored_callback_authoring.js";
import { outputProjectionSpec } from "../../../../output_projection_ids.js";

export async function createStandaloneAuthoredInput(deps, id, initial, options = {}) {
  deps.requireActive("inputAsync");
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError("worker-first inputAsync(...) requires a non-empty authored input id");
  }
  deps.assertUnusedId(id, "inputAsync");
  await deps.publishAuthoredInput(id, initial, options);
}

export function createEagerStandaloneAuthoredInput(deps, id, initial, options = {}) {
  deps.requireActive("input");
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError("worker-first input(...) requires a non-empty authored input id");
  }
  deps.assertUnusedId(id, "input");
  deps.authoredInputs.set(id, createWorkerFirstAuthoredInputState(initial, "pending", options));
  deps.trackEagerPublication(
    [id],
    deps.bridge.publishPortableGraph(createAuthoredInputPublication(id, initial, options)),
    "worker-first input(...) background publication failed",
  );
}

export async function createStandaloneAuthoredReadable(deps, id, family, spec) {
  deps.requireActive(`${family}Async`);
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError(`worker-first ${family}Async(...) requires a non-empty authored ${family} id`);
  }
  deps.assertUnusedId(id, `${family}Async`);
  deps.assertSupportedReadableSpec(family, spec);
  const publishEpoch = deps.currentTipEpoch();
  await deps.bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec));
  const signalPacket = await deps.bridge.readSignals({ signalIds: [id] });
  const signal = signalPacket.signals[0];
  if (!signal || signal.id !== id) {
    throw new TypeError(
      `worker-first ${family}Async(...) could not read committed worker truth for \`${id}\` after authoring`,
    );
  }
  const state = createWorkerFirstAuthoredReadableState(
    family,
    signal.value,
    spec.reads ?? [],
    [],
    [],
    "ready",
    spec,
  );
  deps.stampAdmittedIfEpoch(state, publishEpoch);
  deps.authoredReadables.set(id, state);
}

export function createEagerStandaloneAuthoredReadable(
  deps,
  id,
  family,
  spec,
  initialValue,
  dependencyIds,
) {
  deps.requireActive(`${family}`);
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError(`worker-first ${family}(...) requires a non-empty authored ${family} id`);
  }
  deps.assertUnusedId(id, `${family}`);
  deps.assertSupportedReadableSpec(family, spec);
  deps.authoredReadables.set(
    id,
    createWorkerFirstAuthoredReadableState(
      family,
      initialValue,
      dependencyIds,
      [],
      [],
      "pending",
      spec,
    ),
  );
  const initializePublishedReadable = spec?.when === undefined || spec?.when === null
    ? deps.bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec))
    : deps.bridge.publishPortableGraph(createAuthoredReadablePublication(id, family, spec))
      .then(() => deps.bridge.readSignals({ signalIds: [id] }))
      .then((signalPacket) => {
        updateWorkerFirstAuthoredReadables(deps.authoredReadables, signalPacket.signals);
      });
  deps.trackEagerPublication(
    [id],
    initializePublishedReadable,
    `worker-first ${family}(...) background publication failed`,
  );
}

export async function createStandaloneAuthoredCallbackReadable(deps, id, family, callback) {
  deps.requireActive(`${family}Async`);
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError(`worker-first ${family}Async(...) requires a non-empty authored ${family} id`);
  }
  if (typeof callback !== "function") {
    throw new TypeError(`worker-first ${family}Async(...) callback form requires a function`);
  }
  deps.assertUnusedId(id, `${family}Async`);
  const capture = deps.captureCallback(callback, family);
  const hiddenInputId = nextWorkerFirstCallbackBackingInputId(
    deps.generatedStandaloneSignalCounters,
    family,
    id,
  );
  const readableSpec = outputProjectionSpec(hiddenInputId);
  const publishEpoch = deps.currentTipEpoch();
  await deps.publishAuthoredInput(hiddenInputId, capture.value, {});
  await deps.bridge.publishPortableGraph(
    createAuthoredReadablePublication(id, family, readableSpec),
  );
  const signalPacket = await deps.bridge.readSignals({ signalIds: [id] });
  const signal = signalPacket.signals[0];
  if (!signal || signal.id !== id) {
    throw new TypeError(
      `worker-first ${family}Async(...) could not read committed worker truth for \`${id}\` after callback authoring`,
    );
  }
  const state = createWorkerFirstAuthoredReadableState(
    family,
    signal.value,
    capture.reads,
    capture.hostDependencyIds,
    capture.hostDependencies,
    "ready",
    readableSpec,
  );
  deps.stampAdmittedIfEpoch(state, publishEpoch);
  deps.authoredReadables.set(id, state);
  deps.authoredCallbacks.set(
    id,
    createWorkerFirstAuthoredCallbackState(family, callback, hiddenInputId, capture),
  );
}

export function createEagerStandaloneAuthoredCallbackReadable(deps, id, family, callback) {
  deps.requireActive(`${family}`);
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError(`worker-first ${family}(...) requires a non-empty authored ${family} id`);
  }
  if (typeof callback !== "function") {
    throw new TypeError(`worker-first ${family}(...) callback form requires a function`);
  }
  deps.assertUnusedId(id, `${family}`);
  const capture = deps.captureCallback(callback, family);
  const hiddenInputId = nextWorkerFirstCallbackBackingInputId(
    deps.generatedStandaloneSignalCounters,
    family,
    id,
  );
  const readableSpec = outputProjectionSpec(hiddenInputId);
  deps.authoredInputs.set(
    hiddenInputId,
    createWorkerFirstAuthoredInputState(capture.value, "pending"),
  );
  deps.authoredReadables.set(
    id,
    createWorkerFirstAuthoredReadableState(
      family,
      capture.value,
      capture.reads,
      capture.hostDependencyIds,
      capture.hostDependencies,
      "pending",
      readableSpec,
    ),
  );
  deps.authoredCallbacks.set(
    id,
    createWorkerFirstAuthoredCallbackState(family, callback, hiddenInputId, capture),
  );
  deps.trackEagerPublication(
    [hiddenInputId, id],
    deps.publishCallbackReadableGraph(id, family, hiddenInputId, capture.value),
    `worker-first ${family}(...) background publication failed`,
  );
}
