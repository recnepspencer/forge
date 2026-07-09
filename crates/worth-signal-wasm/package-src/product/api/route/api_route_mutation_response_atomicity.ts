const MUTATION_RESPONSE_ATOMICITY_KINDS = Object.freeze([
  "allOrNone",
  "partialAllowed",
]);

function lowerMutationResponseAtomicity(route, atomicity) {
  if (atomicity === undefined) {
    return "allOrNone";
  }
  if (!MUTATION_RESPONSE_ATOMICITY_KINDS.includes(atomicity)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) atomicity must be one of ${MUTATION_RESPONSE_ATOMICITY_KINDS.join(", ")}`,
    );
  }
  return atomicity;
}

export { lowerMutationResponseAtomicity };
