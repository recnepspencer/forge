# Stale Deep Link Recovery

Recovery gives a declared route a principled fallback when admission reaches a
terminal failure. The easy version is “a deleted project goes to the project
index.” The full model preserves the attempted route, the failure, and the
fallback admission as separate facts.

```ts
const recoverDeletedProject = signals.router.recovery(
  "recover-deleted-project",
  ({ terminalArtifact, fallback }) => {
    if (terminalArtifact.kind !== "notFound") {
      return null;
    }

    return fallback({
      href: "/projects",
      reason: "projectNoLongerExists",
    });
  },
);
```

Attach the named recovery beside the prerequisite that may fail:

```ts
const projectAvailable = signals.router.resource.boolean("projectAvailable");

const requireProject = signals.router.prerequisite("require-project", {
  consumes: [projectAvailable] as const,
  evaluate: ({ consume, allow, notFound }) => (
    consume(projectAvailable)
      ? allow({ reason: "projectAvailable" })
      : notFound({ reason: "projectMissing" })
  ),
});

const routes = signals.router.define({
  projects: signals.router.route("/projects"),
  project: signals.router.route("/projects/:projectId", {
    admission: [requireProject],
    recovery: [recoverDeletedProject],
  }),
});
```

## What Happens

1. The original URL projects and enters admission.
2. A prerequisite produces a terminal artifact.
3. Recovery declarations run in order.
4. A returned fallback URL must project to a declared route.
5. The fallback goes through normal admission.
6. The final outcome retains the attempted and resolved route provenance.

```ts
const outcome = await routes.admit("/projects/p7", {
  projectAvailable: false,
});

console.log(outcome.kind);
console.log(outcome.provenance().attemptedHref);
console.log(outcome.provenance().resolvedHref);
console.log(outcome.recovery());
```

Recovery fails closed. An undeclared or non-admissible fallback does not become
route truth. Redirects also dominate recovery: a redirect is already an
intentional destination, not a stale-link failure to reinterpret.

Use recovery for “this once-valid deep link has a nearest valid home.” Use a
redirect for ordinary navigation policy.

Next: [Nearest Valid Truth](./nearest_valid_truth.md) and
[Recovery Provenance](./recovery_provenance.md).
