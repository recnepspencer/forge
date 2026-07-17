# Admit

The happy path is one call:

```ts
const outcome = await routes.admit("/projects/p7", admissionFacts);
```

Admission turns a structural match into an explicit route outcome. As an
application grows, the same call can evaluate named prerequisites, declared
fact sources, redirects, failures, recovery, and forms authority without
scattering policy across components.

## Declare Facts That Policy Is Allowed To Consume

Raw `facts` are convenient at the call site. A prerequisite should name the
host, resource, or graph values it is authorized to read.

```ts
const signedIn = signals.router.host.boolean("signedIn");
const projectAvailable = signals.router.resource.boolean("projectAvailable");

const mayOpenProject = signals.router.prerequisite("may-open-project", {
  consumes: [signedIn, projectAvailable] as const,
  evaluate: ({ consume, allow, redirect, notFound }) => {
    if (!consume(signedIn)) {
      return redirect({ href: "/sign-in", reason: "signInRequired" });
    }

    return consume(projectAvailable)
      ? allow({ reason: "projectAvailable" })
      : notFound({ reason: "projectMissing" });
  },
});
```

The plain-English rule is simple: a policy may only read facts it declared.
The declared source family records where the value came from; it does not fetch
or manufacture the value.

```ts
const routes = signals.router.define({
  project: signals.router.route("/projects/:projectId", {
    admission: [mayOpenProject],
  }),
});

const outcome = await routes.admit("/projects/p7", {
  signedIn: true,
  projectAvailable: true,
});
```

## Handle The Outcome You Actually Received

`outcome.kind` is one of `admitted`, `redirect`, `notFound`, `forbidden`,
`unavailable`, or `denied`. Only an admitted outcome has `route()`.

```ts
if (outcome.kind === "admitted") {
  renderRoute(outcome.route());
} else {
  renderRouteFailure(outcome.kind, outcome.artifact());
}
```

Prerequisites run in declaration order and the first non-allow result stops the
ordinary admission path. Recovery may then handle an eligible terminal result;
a redirect is already a decision and is not recovered.

## Inspect Before Guessing

- `outcome.diagnostics()` summarizes the result.
- `outcome.provenance()` preserves prerequisite decisions and recovery trail.
- `candidate.admission(facts)` exposes `prerequisiteNames()`,
  `recoveryNames()`, and plan provenance before `resolve()`.

Do not repeat the same permission test in the view. Doing so creates two
answers to “may this route be visible?”

Next: [Prerequisites](./prerequisites.md), [Route Outcomes](./route_outcomes.md),
and [Stale Deep Link Recovery](../recovery/stale_deep_link_recovery.md).
