import { freezeObject } from "../graph_support.js";
import { PUBLIC_GRAPH_INPUT } from "../symbols.js";

export function createWorkerFirstPublicInputEntry(rootSession, handle, options) {
  const normalizedHandle = requireWorkerFirstInputHandle(
    rootSession,
    handle,
    "signals.publicInput(...)",
  );
  const normalizedOptions = normalizePublicInputOptions(options);
  return freezeObject({
    handle: normalizedHandle,
    authority: normalizedOptions.authority,
    requiredness: normalizedOptions.requiredness,
    [PUBLIC_GRAPH_INPUT]: true,
  });
}

export function isWorkerFirstPublicGraphInputEntry(candidate) {
  return isPlainObject(candidate) && candidate[PUBLIC_GRAPH_INPUT] === true;
}

export function requireWorkerFirstInputHandle(rootSession, handle, operation) {
  const normalizedHandle = requireWorkerFirstSignalHandle(
    rootSession,
    handle,
    `${operation} expects a worker-first input handle`,
  );
  if (
    !rootSession.hasMutableInputId(normalizedHandle.id)
    || typeof normalizedHandle.set !== "function"
    || typeof normalizedHandle.reset !== "function"
    || typeof normalizedHandle.patch !== "function"
    || typeof normalizedHandle.assign !== "function"
  ) {
    throw new TypeError(`${operation} expects a worker-first input handle`);
  }
  return normalizedHandle;
}

export function requireWorkerFirstSignalHandle(rootSession, handle, message) {
  if (
    typeof handle !== "function"
    || typeof handle.id !== "string"
    || handle.id.length === 0
    || !rootSession.hasKnownSignalId(handle.id)
  ) {
    throw new TypeError(message);
  }
  return handle;
}

function normalizePublicInputOptions(options) {
  if (options === undefined) {
    return { authority: "writable", requiredness: "required" };
  }
  if (!isPlainObject(options)) {
    throw new TypeError("signals.publicInput(...) options must be an object when provided");
  }
  return {
    authority: requireAuthority(options.authority),
    requiredness: requireRequiredness(options.requiredness),
  };
}

function requireAuthority(authority) {
  if (authority === undefined) {
    return "writable";
  }
  if (authority !== "writable" && authority !== "readOnly" && authority !== "imported") {
    throw new TypeError(
      `signals.publicInput(...) authority must be "writable", "readOnly", or "imported" when provided`,
    );
  }
  return authority;
}

function requireRequiredness(requiredness) {
  if (requiredness === undefined) {
    return "required";
  }
  if (requiredness !== "required" && requiredness !== "optional") {
    throw new TypeError(
      'signals.publicInput(...) requiredness must be "required" or "optional" when provided',
    );
  }
  return requiredness;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
