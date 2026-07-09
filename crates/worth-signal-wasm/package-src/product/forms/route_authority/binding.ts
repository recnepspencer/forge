export function resolveRouteAuthorityBinding(input) {
  if (input === null || input === undefined) {
    return {
      kind: "missing",
      reason: "route authority binding requires an admitted route or route forms authority artifact",
    };
  }
  if (typeof input.formsAuthority === "function") {
    const authority = input.formsAuthority();
    if (authority !== null && authority !== undefined) {
      return {
        kind: "authority",
        authority,
      };
    }
    return {
      kind: "missing",
      reason: "admitted route does not declare a forms authority surface to bind",
    };
  }
  return {
    kind: "authority",
    authority: input,
  };
}
