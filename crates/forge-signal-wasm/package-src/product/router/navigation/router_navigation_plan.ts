import {
  createNavigationIntentVerification,
  createNavigationPlanVerification,
} from "../url_authority/router_verification_packages.js";
import {
  mergeNavigationPolicy,
  normalizeNavigationIntentKind,
  normalizeNavigationPolicy,
} from "./router_navigation_policy.js";
import {
  createFreshnessDiagnostics,
  createNavigationCost,
  createExecutionContract,
  createNavigationExplanation,
  createProjectionPolicy,
  createTransitionPolicy,
} from "./router_navigation_projection_policy.js";

function createNavigationIntentBuilder(location, options = {}) {
  const intentKind = normalizeNavigationIntentKind(options.kind);
  const policy = normalizeNavigationPolicy(options.policy);
  const descriptor = Object.freeze({
    kind: intentKind,
    routeId: location.routeId,
    href: location.href,
    params: location.params,
    search: location.search,
    hash: location.hash,
    canonical() {
      return location.canonical();
    },
  });
  return Object.freeze({
    descriptor() {
      return descriptor;
    },
    verification() {
      return createNavigationIntentVerification(
        location.route.verification(),
        location.canonical(),
        intentKind,
        policy,
      );
    },
    policy(nextPolicy) {
      return createNavigationIntentBuilder(location, {
        kind: intentKind,
        policy: mergeNavigationPolicy(policy, nextPolicy),
      });
    },
    compile() {
      return createNavigationPlan(location, intentKind, policy);
    },
  });
}

function createNavigationPlan(location, intentKind, policy) {
  const normalizedPolicy = normalizeNavigationPolicy(policy);
  const resolvedNavigationPolicy = Object.freeze({
    ...normalizedPolicy,
    intentKind,
  });
  const cost = createNavigationCost(location, intentKind, resolvedNavigationPolicy);
  const transitionPolicy = createTransitionPolicy(resolvedNavigationPolicy);
  const executionContract = createExecutionContract(resolvedNavigationPolicy);
  const projectionPolicy = createProjectionPolicy(resolvedNavigationPolicy);
  const freshness = createFreshnessDiagnostics(resolvedNavigationPolicy);
  const explanation = createNavigationExplanation(
    location,
    intentKind,
    resolvedNavigationPolicy,
    transitionPolicy,
    executionContract,
    cost,
    projectionPolicy,
    freshness,
  );
  return Object.freeze({
    kind: intentKind,
    routeId: location.routeId,
    href: location.href,
    params: location.params,
    search: location.search,
    hash: location.hash,
    canonical() {
      return location.canonical();
    },
    descriptor() {
      return location.descriptor();
    },
    cost() {
      return cost;
    },
    policy() {
      return transitionPolicy;
    },
    execution() {
      return executionContract;
    },
    explain() {
      return explanation;
    },
    freshness() {
      return freshness;
    },
    verification() {
      return createNavigationPlanVerification(
        location.route.verification(),
        location.canonical(),
        intentKind,
        resolvedNavigationPolicy,
        explanation,
      );
    },
    projectionPolicy() {
      return projectionPolicy;
    },
  });
}

export {
  createNavigationIntentBuilder,
  createNavigationPlan,
};
