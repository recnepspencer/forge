function createMutationResponseDeclaredFieldDigest(fieldNames) {
  if (fieldNames === null) {
    return "mutation-response-declared-fields|not-applicable";
  }
  if (fieldNames.length === 0) {
    return "mutation-response-declared-fields|none";
  }
  return `mutation-response-declared-fields|${fieldNames.join(",")}`;
}

function createMutationResponseUnknownFieldPosture(fieldNames, responseValue) {
  if (fieldNames === null) {
    return Object.freeze({
      kind: "notApplicable",
      fields: Object.freeze([]),
      digest: "mutation-response-unknown-fields|not-applicable",
    });
  }
  const unknownFields = readUnknownResponseFields(fieldNames, responseValue);
  if (unknownFields.length === 0) {
    return Object.freeze({
      kind: "none",
      fields: unknownFields,
      digest: "mutation-response-unknown-fields|none",
    });
  }
  return Object.freeze({
    kind: "present",
    fields: unknownFields,
    digest: `mutation-response-unknown-fields|${unknownFields.join(",")}`,
  });
}

function readUnknownResponseFields(fieldNames, responseValue) {
  if (!responseValue || typeof responseValue !== "object" || Array.isArray(responseValue)) {
    return Object.freeze([]);
  }
  const prototype = Object.getPrototypeOf(responseValue);
  if (prototype !== Object.prototype && prototype !== null) {
    return Object.freeze([]);
  }
  const declaredFields = new Set(fieldNames);
  const unknownFields = [];
  for (const key of Object.keys(responseValue).sort()) {
    const descriptor = Object.getOwnPropertyDescriptor(responseValue, key);
    if (descriptor === undefined) {
      continue;
    }
    if ("get" in descriptor || "set" in descriptor) {
      throw new TypeError(
        `mutation response unknown-field posture cannot inspect accessor-backed property "${key}"`,
      );
    }
    if (!declaredFields.has(key)) {
      unknownFields.push(key);
    }
  }
  return Object.freeze(unknownFields);
}

export {
  createMutationResponseDeclaredFieldDigest,
  createMutationResponseUnknownFieldPosture,
};
