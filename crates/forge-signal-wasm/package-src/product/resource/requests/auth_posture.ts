const RESOURCE_AUTH_POSTURE_BRAND = Symbol("forgeSignal.resourceAuthPosture");

function createResourceAuthPosture(kind) {
  return Object.freeze({
    kind,
    [RESOURCE_AUTH_POSTURE_BRAND]: "resourceAuthPosture",
  });
}

function requireResourceAuthPosture(value, family) {
  if (
    !value ||
    value[RESOURCE_AUTH_POSTURE_BRAND] !== "resourceAuthPosture"
  ) {
    throw new TypeError(
      `${family} resources require auth created with resourceAuth.*()`,
    );
  }
  return value;
}

export { createResourceAuthPosture, requireResourceAuthPosture };
