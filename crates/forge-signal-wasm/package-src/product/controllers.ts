import { isPublicGraphInputEntry } from "./public_inputs.js";
import { CONTROLLER_CONTRACT, PRODUCT_SIGNAL_KIND } from "./symbols.js";

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function nullPrototypeRecord() {
  return Object.create(null);
}

function freezeRecord(record) {
  return Object.freeze(record);
}

function cloneRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [key, value] of Object.entries(record)) {
    clone[key] = value;
  }
  return freezeRecord(clone);
}

function requireRecord(candidate, fieldName) {
  if (candidate === undefined) {
    return freezeRecord(nullPrototypeRecord());
  }
  if (!isPlainObject(candidate)) {
    throw new TypeError(`controller.${fieldName} must be an object when provided`);
  }
  return cloneRecord(candidate);
}

function isProductHandle(value) {
  return typeof value === "function" && typeof value.id === "string" && PRODUCT_SIGNAL_KIND in value;
}

function requireControllerInputRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isPublicGraphInputEntry(value)) {
      clone[name] = value;
      continue;
    }
    if (!isProductHandle(value) || value[PRODUCT_SIGNAL_KIND] !== "input") {
      throw new TypeError(
        `controller.inputs.\`${name}\` must be an input handle or signals.publicInput(...) entry`,
      );
    }
    clone[name] = value;
  }
  return freezeRecord(clone);
}

function requireControllerOutputRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.outputs.\`${name}\` cannot use signals.publicInput(...); public input authority belongs only in controller.inputs`,
      );
    }
    if (!isProductHandle(value)) {
      throw new TypeError(
        `controller.outputs.\`${name}\` must be a product signal handle created by this package`,
      );
    }
    clone[name] = value;
  }
  return freezeRecord(clone);
}

function requireControllerInternalRecord(record) {
  const clone = nullPrototypeRecord();
  for (const [name, value] of Object.entries(record)) {
    if (isPublicGraphInputEntry(value)) {
      throw new TypeError(
        `controller.internal.\`${name}\` cannot use signals.publicInput(...); public authority wrappers belong only in controller.inputs`,
      );
    }
    clone[name] = value;
  }
  return freezeRecord(clone);
}

export function isControllerContract(candidate) {
  return isPlainObject(candidate) && candidate[CONTROLLER_CONTRACT] === true;
}

export function createControllerContract(definition) {
  if (!isPlainObject(definition)) {
    throw new TypeError("signals.controller requires a controller definition object");
  }

  const inputs = requireControllerInputRecord(requireRecord(definition.inputs, "inputs"));
  const outputs = requireControllerOutputRecord(requireRecord(definition.outputs, "outputs"));
  const internal = requireControllerInternalRecord(requireRecord(definition.internal, "internal"));

  return Object.freeze({
    inputs,
    outputs,
    internal,
    [CONTROLLER_CONTRACT]: true,
  });
}
