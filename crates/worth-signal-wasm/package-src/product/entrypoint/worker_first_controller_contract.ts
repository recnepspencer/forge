import { freezeObject } from "../graph_support.js";
import { CONTROLLER_CONTRACT } from "../symbols.js";
import {
  isWorkerFirstPublicGraphInputEntry,
  requireWorkerFirstInputHandle,
  requireWorkerFirstSignalHandle,
} from "./worker_first_public_input_support.js";

export function buildControllerContract(
  rootSession,
  path,
  definitionOrBuilder,
  createScopedNamespace,
) {
  if (typeof definitionOrBuilder === "function") {
    const authoringSurface = createScopedNamespace(rootSession, path);
    return buildControllerContract(
      rootSession,
      path,
      definitionOrBuilder(authoringSurface),
      createScopedNamespace,
    );
  }
  return createControllerContract(rootSession, definitionOrBuilder);
}

function createControllerContract(rootSession, definition) {
  if (!isPlainObject(definition)) {
    throw new TypeError("signals.controller requires a controller definition object");
  }
  return freezeObject({
    inputs: requireControllerInputRecord(rootSession, requireRecord(definition.inputs, "inputs")),
    outputs: requireControllerOutputRecord(rootSession, requireRecord(definition.outputs, "outputs")),
    internal: requireControllerInternalRecord(requireRecord(definition.internal, "internal")),
    [CONTROLLER_CONTRACT]: true,
  });
}

function requireControllerInputRecord(rootSession, record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      requireWorkerFirstInputHandle(rootSession, value.handle, `controller.inputs.\`${name}\``);
      clone[name] = value;
      continue;
    }
    clone[name] = requireWorkerFirstInputHandle(
      rootSession,
      value,
      `controller.inputs.\`${name}\``,
    );
  }
  return freezeObject(clone);
}

function requireControllerOutputRecord(rootSession, record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.outputs.\`${name}\` cannot use signals.publicInput(...); public input authority belongs only in controller.inputs`,
      );
    }
    clone[name] = requireWorkerFirstSignalHandle(
      rootSession,
      value,
      `controller.outputs.\`${name}\` must be a worker-first signal handle from the active imported graph`,
    );
  }
  return freezeObject(clone);
}

function requireControllerInternalRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isWorkerFirstPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.internal.\`${name}\` cannot use signals.publicInput(...); public authority wrappers belong only in controller.inputs`,
      );
    }
    clone[name] = value;
  }
  return freezeObject(clone);
}

function requireRecord(candidate, fieldName) {
  if (candidate === undefined) {
    return freezeObject(nullPrototypeRecord());
  }
  if (!isPlainObject(candidate)) {
    throw new TypeError(`controller.${fieldName} must be an object when provided`);
  }
  const clone = nullPrototypeRecord();
  for (const [key, value] of Object.entries(candidate)) {
    clone[key] = value;
  }
  return freezeObject(clone);
}

function nullPrototypeRecord() {
  return Object.create(null);
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
