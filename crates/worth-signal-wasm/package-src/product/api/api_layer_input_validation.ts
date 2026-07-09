import { isPlainObject } from "../authoring_option_validation.js";

function resolveHeaderObject(value) {
  if (!isPlainObject(value)) {
    throw new TypeError(
      "signals.api(...) headers must resolve to a plain object of string values",
    );
  }
  const snapshot = {};
  for (const [name, headerValue] of Object.entries(value)) {
    if (typeof headerValue !== "string") {
      throw new TypeError(
        "signals.api(...) headers must resolve to a plain object of string values",
      );
    }
    snapshot[name] = headerValue;
  }
  return snapshot;
}

function validateHeadersInput(input) {
  if (input === undefined || typeof input === "function") {
    return;
  }
  resolveHeaderObject(input);
}

function validateBaseUrlInput(input) {
  if (
    input !== undefined
    && typeof input !== "string"
    && typeof input !== "function"
  ) {
    throw new TypeError("signals.api(...) baseUrl must be a string or function");
  }
}

function validatePostureInput(name, input, validator) {
  if (input === undefined || typeof input === "function") {
    return;
  }
  validator(input, `signals.api ${name}`);
}

export {
  resolveHeaderObject,
  validateBaseUrlInput,
  validateHeadersInput,
  validatePostureInput,
};
