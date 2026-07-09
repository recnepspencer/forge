import assert from "node:assert/strict";

export function createRouteCoupledForm(signals, options = {}) {
  const {
    source = { title: "Ship docs" },
    host,
    routeAction = {},
  } = options;
  return signals.form({
    source,
    ...(host === undefined ? {} : { host }),
    fields: ({ field }) => ({
      title: field("title"),
    }),
    steps: ({ step }) => ({
      review: step("review", ["title"], { routeCoupled: true }),
    }),
    actions: ({ step, submit }) => ({
      reviewRoute: step("reviewRoute", "review", "jump", {
        routeCoupled: true,
        ...routeAction,
      }),
      ...(routeAction.includeSubmit === true
        ? {
          submit: submit({ patchPolicy: "requiresNonEmpty" }),
        }
        : {}),
    }),
  });
}

export async function createAdmittedAuthorityArtifact(signals, surfaceId, continuity) {
  const routes = signals.router.define({
    review: signals.router.route("/review", {
      forms: signals.router.forms(surfaceId, { continuity }),
    }),
  });
  const outcome = await routes.admit("/review");
  assert.equal(outcome.kind, "admitted");
  return outcome.route().formsAuthority();
}
