import { isRouteLocation } from "../router_location.js";
import { isRawLocationAuthority } from "../url_authority/router_url_authority.js";

function createRouteSequenceScenario(routes, navigationSupport, sequence) {
  const normalizedSteps = normalizeRouteSequenceSteps(sequence);
  return Object.freeze({
    steps: normalizedSteps,
    async run(options = {}) {
      const story = navigationSupport.browserHistory.story();
      const stepResults = [];
      for (let index = 0; index < normalizedSteps.length; index += 1) {
        const step = normalizedSteps[index];
        const navigationKind = step.historyMethod ?? (index === 0 ? "load" : "push");
        const ingress = navigationSupport.browserHistory[navigationKind](
          step.target,
          createRouteSequenceIngressOptions(
            navigationSupport,
            story,
            step,
            navigationKind,
          ),
        );
        const facts = step.facts ?? options.facts ?? {};
        const report = await routes.admitBrowserHistoryIngress(ingress, facts);
        const event = story.record(report);
        stepResults.push(Object.freeze({
          index,
          targetHref: resolveRouteSequenceTargetHref(step.target),
          navigationKind: ingress.navigationKind,
          facts,
          report,
          event,
          current: story.current(),
          breadcrumbTrail: story.breadcrumbTrail(),
          backProvenance: story.backProvenance(),
        }));
      }
      const frozenSteps = Object.freeze(stepResults);
      return Object.freeze({
        story,
        steps: frozenSteps,
        replay: createRouteSequenceReplay(frozenSteps),
        diagnostics() {
          return createRouteSequenceDiagnostics(frozenSteps);
        },
      });
    },
  });
}

function createRouteSequenceReplay(steps) {
  return Object.freeze({
    outcomes() {
      return Object.freeze(steps.map((step) => step.report.outcome()));
    },
    breadcrumbTrail() {
      return Object.freeze(steps.map((step) => step.breadcrumbTrail));
    },
    backProvenance() {
      return Object.freeze(steps.map((step) => step.backProvenance));
    },
    currentEntries() {
      return Object.freeze(steps.map((step) => step.current));
    },
  });
}

function createRouteSequenceDiagnostics(steps) {
  const denied = [];
  const notAdmitted = [];
  for (const step of steps) {
    const outcome = step.report.outcome();
    if (outcome?.kind === "denied") {
      denied.push(Object.freeze({
        index: step.index,
        targetHref: step.targetHref,
        outcomeKind: outcome.kind,
        eventBoundaryArtifact: step.event.boundaryArtifact,
      }));
      continue;
    }
    if (outcome?.kind !== "admitted") {
      notAdmitted.push(Object.freeze({
        index: step.index,
        targetHref: step.targetHref,
        outcomeKind: outcome?.kind ?? "unknown",
        eventBoundaryArtifact: step.event.boundaryArtifact,
      }));
    }
  }
  return Object.freeze({
    hasFailures: denied.length > 0 || notAdmitted.length > 0,
    denied,
    notAdmitted,
  });
}

function normalizeRouteSequenceSteps(sequence) {
  if (!Array.isArray(sequence) || sequence.length === 0) {
    throw new TypeError(
      "routes.simulateSequence(...) requires a non-empty array of route targets or step declarations",
    );
  }
  return Object.freeze(sequence.map((step, index) => normalizeRouteSequenceStep(step, index)));
}

function normalizeRouteSequenceStep(step, index) {
  if (isRouteSequenceTarget(step)) {
    return Object.freeze({
      target: normalizeRouteSequenceTarget(step, `routes.simulateSequence(...)[${index}]`),
      historyMethod: undefined,
      carryBreadcrumbs: undefined,
      ingress: undefined,
      facts: undefined,
    });
  }
  if (step && typeof step === "object" && !Array.isArray(step)) {
    const {
      target,
      historyMethod,
      carryBreadcrumbs,
      ingress,
      facts,
      ...unknownFields
    } = step;
    const unknownKeys = Object.keys(unknownFields);
    if (unknownKeys.length > 0) {
      throw new TypeError(
        `routes.simulateSequence(...)[${index}] does not support: ${unknownKeys.join(", ")}`,
      );
    }
    return Object.freeze({
      target: normalizeRouteSequenceTarget(
        target,
        `routes.simulateSequence(...)[${index}].target`,
      ),
      historyMethod: normalizeOptionalHistoryMethod(
        historyMethod,
        `routes.simulateSequence(...)[${index}].historyMethod`,
      ),
      carryBreadcrumbs: normalizeOptionalBoolean(
        carryBreadcrumbs,
        `routes.simulateSequence(...)[${index}].carryBreadcrumbs`,
      ),
      ingress: normalizeOptionalIngress(
        ingress,
        `routes.simulateSequence(...)[${index}].ingress`,
      ),
      facts,
    });
  }
  throw new TypeError(
    `routes.simulateSequence(...)[${index}] requires a route location, href string, raw location authority, or step declaration object`,
  );
}

function createRouteSequenceIngressOptions(navigationSupport, story, step, historyMethod) {
  const baseOptions = step.ingress ? { ...step.ingress } : {};
  if (baseOptions.carriedBreadcrumbs !== undefined) {
    return baseOptions;
  }
  const shouldCarryBreadcrumbs = step.carryBreadcrumbs ?? historyMethod !== "load";
  if (
    !shouldCarryBreadcrumbs
    || typeof navigationSupport.carryBreadcrumbs !== "function"
  ) {
    return baseOptions;
  }
  return Object.freeze({
    ...baseOptions,
    carriedBreadcrumbs: navigationSupport.carryBreadcrumbs(story.breadcrumbTrail()),
  });
}

function isRouteSequenceTarget(value) {
  return typeof value === "string" || isRawLocationAuthority(value) || isRouteLocation(value);
}

function normalizeRouteSequenceTarget(value, operation) {
  if (typeof value === "string" || isRawLocationAuthority(value)) {
    return value;
  }
  if (isRouteLocation(value)) {
    return value.href;
  }
  throw new TypeError(
    `${operation} requires a route location, href string, or raw location authority`,
  );
}

function resolveRouteSequenceTargetHref(target) {
  if (typeof target === "string") {
    return target;
  }
  return target.href;
}

function normalizeOptionalHistoryMethod(value, operation) {
  if (value === undefined) {
    return undefined;
  }
  if (
    value === "load"
    || value === "push"
    || value === "replace"
    || value === "pop"
    || value === "manual"
    || value === "external"
  ) {
    return value;
  }
  throw new TypeError(
    `${operation} must be one of load, push, replace, pop, manual, external`,
  );
}

function normalizeOptionalBoolean(value, operation) {
  if (value === undefined) {
    return undefined;
  }
  if (typeof value === "boolean") {
    return value;
  }
  throw new TypeError(`${operation} must be a boolean when provided`);
}

function normalizeOptionalIngress(value, operation) {
  if (value === undefined) {
    return undefined;
  }
  if (value && typeof value === "object" && !Array.isArray(value)) {
    return Object.freeze({ ...value });
  }
  throw new TypeError(`${operation} must be an object when provided`);
}

export {
  createRouteSequenceScenario,
};
