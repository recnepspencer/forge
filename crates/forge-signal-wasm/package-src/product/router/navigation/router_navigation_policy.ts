const NAVIGATION_INTENT_KINDS = Object.freeze([
  "push",
  "replace",
  "canonicalize",
  "softRefresh",
  "sameRouteMutation",
  "breadcrumbReturn",
  "restoreBack",
]);

const NAVIGATION_CONTINUITY_POLICIES = Object.freeze([
  "refresh-immediately",
  "preserve-visible-while-pending",
  "preserve-visible-until-explicit-refresh",
]);

const NAVIGATION_PROJECTION_REFRESH_POLICIES = Object.freeze([
  "immediate",
  "after-admission",
  "explicit",
]);

const NAVIGATION_ARTIFACT_POLICIES = Object.freeze([
  "minimal",
  "diagnostics",
]);

const NAVIGATION_COMMIT_POLICIES = Object.freeze([
  "directCommit",
  "speculativeBranch",
]);

const NAVIGATION_REDIRECT_POLICIES = Object.freeze([
  "followRedirect",
  "surfaceRedirect",
]);

const NAVIGATION_DEPLOYMENTS = Object.freeze([
  "workerFirst",
  "mainThreadCompatibility",
]);

function normalizeNavigationIntentKind(kind) {
  if (kind === undefined) {
    return "push";
  }
  if (!NAVIGATION_INTENT_KINDS.includes(kind)) {
    throw new TypeError(
      `router navigation intent kind must be one of ${NAVIGATION_INTENT_KINDS.join(", ")}`,
    );
  }
  return kind;
}

function normalizeNavigationPolicy(policy) {
  if (policy === undefined) {
    return Object.freeze({
      continuity: "refresh-immediately",
      projectionRefresh: "immediate",
      artifactPolicy: "minimal",
      commit: "directCommit",
      redirect: "followRedirect",
      deployment: "mainThreadCompatibility",
    });
  }
  if (!isPlainObject(policy)) {
    throw new TypeError("router navigation policy must be an object when provided");
  }
  const continuity = normalizePolicyValue(
    policy.continuity,
    NAVIGATION_CONTINUITY_POLICIES,
    "router navigation continuity policy",
    "refresh-immediately",
  );
  const projectionRefresh = normalizePolicyValue(
    policy.projectionRefresh,
    NAVIGATION_PROJECTION_REFRESH_POLICIES,
    "router navigation projectionRefresh policy",
    "immediate",
  );
  const artifactPolicy = normalizePolicyValue(
    policy.artifactPolicy,
    NAVIGATION_ARTIFACT_POLICIES,
    "router navigation artifactPolicy",
    "minimal",
  );
  const commit = normalizePolicyValue(
    policy.commit,
    NAVIGATION_COMMIT_POLICIES,
    "router navigation commit policy",
    "directCommit",
  );
  const redirect = normalizePolicyValue(
    policy.redirect,
    NAVIGATION_REDIRECT_POLICIES,
    "router navigation redirect policy",
    "followRedirect",
  );
  const deployment = normalizePolicyValue(
    policy.deployment,
    NAVIGATION_DEPLOYMENTS,
    "router navigation deployment",
    "mainThreadCompatibility",
  );
  return Object.freeze(validateFreshnessPolicyConsistency({
    continuity,
    projectionRefresh,
    artifactPolicy,
    commit,
    redirect,
    deployment,
  }));
}

function mergeNavigationPolicy(currentPolicy, nextPolicy) {
  if (nextPolicy === undefined) {
    return currentPolicy;
  }
  if (!isPlainObject(nextPolicy)) {
    throw new TypeError("router navigation policy must be an object when provided");
  }
  return {
    continuity: nextPolicy.continuity ?? currentPolicy.continuity,
    projectionRefresh: nextPolicy.projectionRefresh ?? currentPolicy.projectionRefresh,
    artifactPolicy: nextPolicy.artifactPolicy ?? currentPolicy.artifactPolicy,
    commit: nextPolicy.commit ?? currentPolicy.commit,
    redirect: nextPolicy.redirect ?? currentPolicy.redirect,
    deployment: nextPolicy.deployment ?? currentPolicy.deployment,
  };
}

function normalizePolicyValue(value, allowedValues, label, fallback) {
  if (value === undefined) {
    return fallback;
  }
  if (!allowedValues.includes(value)) {
    throw new TypeError(`${label} must be one of ${allowedValues.join(", ")}`);
  }
  return value;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function validateFreshnessPolicyConsistency(policy) {
  if (
    policy.continuity === "refresh-immediately"
    && policy.projectionRefresh !== "immediate"
  ) {
    throw new TypeError(
      "router navigation continuity policy refresh-immediately requires projectionRefresh immediate",
    );
  }
  if (
    policy.continuity === "preserve-visible-while-pending"
    && policy.projectionRefresh !== "after-admission"
  ) {
    throw new TypeError(
      "router navigation continuity policy preserve-visible-while-pending requires projectionRefresh after-admission",
    );
  }
  if (
    policy.continuity === "preserve-visible-until-explicit-refresh"
    && policy.projectionRefresh !== "explicit"
  ) {
    throw new TypeError(
      "router navigation continuity policy preserve-visible-until-explicit-refresh requires projectionRefresh explicit",
    );
  }
  return policy;
}

export {
  mergeNavigationPolicy,
  normalizeNavigationIntentKind,
  normalizeNavigationPolicy,
};
