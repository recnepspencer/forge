import { PRODUCT_SIGNAL_KIND, PUBLIC_GRAPH_INPUT } from "./symbols.js";

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
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

function normalizeOptions(options) {
  if (options === undefined) {
    return { authority: "writable" };
  }
  if (!isPlainObject(options)) {
    throw new TypeError("signals.publicInput(...) options must be an object when provided");
  }
  return {
    authority: requireAuthority(options.authority),
  };
}

export function isPublicGraphInputEntry(candidate) {
  return isPlainObject(candidate) && candidate[PUBLIC_GRAPH_INPUT] === true;
}

export function createPublicGraphInputEntry(handle, options) {
  if (typeof handle !== "function" || handle[PRODUCT_SIGNAL_KIND] !== "input") {
    throw new TypeError("signals.publicInput(...) expects an input handle created by this package");
  }

  const normalizedOptions = normalizeOptions(options);

  return Object.freeze({
    handle,
    authority: normalizedOptions.authority,
    [PUBLIC_GRAPH_INPUT]: true,
  });
}
