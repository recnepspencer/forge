function normalizeSemanticMutationDeclaration(route, declaration) {
  if (!declaration || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new TypeError(`api.url("${route}").mutation(...) requires a declaration object`);
  }
  const { semantics, method, ...rest } = declaration;
  if (semantics !== "create" && semantics !== "update" && semantics !== "remove") {
    throw new TypeError(
      `api.url("${route}").mutation(...) requires semantics to be "create", "update", or "remove"`,
    );
  }
  return Object.freeze({
    method,
    semanticFinalizer: semantics,
    declaration: rest,
  });
}

function normalizeCommandMutationDeclaration(route, declaration) {
  if (!declaration || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new TypeError(`api.url("${route}").command(...) requires a declaration object`);
  }
  const { semantics, method, reconciles, identity, diagnostics, ...rest } = declaration;
  if (
    semantics !== "command"
    && semantics !== "relationshipUpdate"
    && semantics !== "aggregateMutation"
    && semantics !== "sideEffect"
  ) {
    throw new TypeError(
      `api.url("${route}").command(...) requires a command semantics value`,
    );
  }
  if (identity !== undefined) {
    throw new TypeError(
      `api.url("${route}").command(...) does not admit identity(...) because command routes cannot pretend to migrate visible topology identity`,
    );
  }
  if (diagnostics !== undefined) {
    throw new TypeError(
      `api.url("${route}").command(...) does not admit diagnostics(...) because command routes only support fallback-only mutation response targets`,
    );
  }
  if (reconciles !== undefined) {
    for (const target of reconciles) {
      if (target.detail || target.collection || target.summary) {
        throw new TypeError(
          `api.url("${route}").command(...) only admits fallback-only reconciles(...) targets`,
        );
      }
    }
  }
  return Object.freeze({
    method,
    semanticFinalizer: "update",
    declaration: {
      ...rest,
      ...(reconciles === undefined ? {} : { reconciles }),
    },
  });
}

export {
  normalizeCommandMutationDeclaration,
  normalizeSemanticMutationDeclaration,
};
