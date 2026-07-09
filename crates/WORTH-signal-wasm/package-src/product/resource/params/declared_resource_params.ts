const DECLARED_RESOURCE_PARAMS_BRAND = Symbol(
  "WORTHSignal.declaredResourceParams",
);

function resourceParams() {
  return Object.freeze({
    [DECLARED_RESOURCE_PARAMS_BRAND]: "declaredResourceParams",
  });
}

function requireDeclaredResourceParams(value, family) {
  if (
    !value ||
    value[DECLARED_RESOURCE_PARAMS_BRAND] !== "declaredResourceParams"
  ) {
    throw new TypeError(
      `${family} resources require params created with resourceParams(...)`,
    );
  }
  return value;
}

export { requireDeclaredResourceParams, resourceParams };
