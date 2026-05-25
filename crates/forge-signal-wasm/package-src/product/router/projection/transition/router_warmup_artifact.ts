import {
  ROUTE_PREFETCH_ARTIFACT,
} from "../../router_symbols.js";
import {
  createCanonicalDigest,
} from "../../url_authority/router_verification_packages.js";

function createProjectedRoutePrefetchArtifact(candidate, trigger) {
  const declaredResourceNames = Object.freeze(candidate.route().resourceNames());
  if (declaredResourceNames.length === 0) {
    throw new TypeError(
      `route candidate "${candidate.routeId}" does not declare any route-local resources to prefetch or warm`,
    );
  }
  const resources = Object.freeze(declaredResourceNames.flatMap((name) => {
    const resource = candidate.route().resource(name);
    if (trigger !== "intent" && resource.prefetchPosture() !== trigger) {
      return [];
    }
    return [Object.freeze({
      name,
      prefetch: resource.warmup(trigger),
    })];
  }));
  if (resources.length === 0) {
    throw new TypeError(
      `route candidate "${candidate.routeId}" does not declare any resources for trigger "${trigger}"`,
    );
  }
  const skippedResourceNames = Object.freeze(
    declaredResourceNames.filter((name) => !resources.some((entry) => entry.name === name)),
  );
  return Object.freeze({
    [ROUTE_PREFETCH_ARTIFACT]: true,
    kind: "routePrefetchAdmission",
    routeId: candidate.routeId,
    href: candidate.href,
    trigger,
    candidate() {
      return candidate;
    },
    declaredResourceNames() {
      return declaredResourceNames;
    },
    resourceNames() {
      return Object.freeze(resources.map((entry) => entry.name));
    },
    skippedResourceNames() {
      return skippedResourceNames;
    },
    resource(name) {
      const entry = resources.find((resource) => resource.name === name);
      if (entry === undefined) {
        throw new TypeError(
          `route prefetch artifact "${candidate.routeId}" does not expose resource "${String(name)}"`,
        );
      }
      return entry.prefetch;
    },
    resources() {
      return resources.map((entry) => entry.prefetch);
    },
    async admit(facts = {}) {
      return candidate.admission(facts).resolve();
    },
    free() {
      for (const resource of resources) {
        resource.prefetch.free();
      }
    },
    [Symbol.dispose]() {
      this.free();
    },
    verification() {
      return Object.freeze({
        routePrefetchDigest: createCanonicalDigest("route-prefetch-admission", {
          routeId: candidate.routeId,
          href: candidate.href,
          trigger,
          projectedCandidateDigest: candidate.verification().projectedCandidateDigest,
          declaredResourceNames,
          skippedResourceNames,
          resources: resources.map((entry) => ({
            name: entry.name,
            routeResourcePrefetchDigest: entry.prefetch.verification().routeResourcePrefetchDigest,
          })),
        }),
      });
    },
  });
}

function tryCreateProjectedRouteWarmupArtifact(candidate, trigger) {
  const declaredResourceNames = candidate.route().resourceNames();
  if (declaredResourceNames.length === 0) {
    return null;
  }
  const matchingResourceNames = declaredResourceNames.filter((name) => {
    return trigger === "intent" || candidate.route().resource(name).prefetchPosture() === trigger;
  });
  if (matchingResourceNames.length === 0) {
    return null;
  }
  return createProjectedRoutePrefetchArtifact(candidate, trigger);
}

function isProjectedRoutePrefetchArtifact(value) {
  return Boolean(value && value[ROUTE_PREFETCH_ARTIFACT] === true);
}

export {
  createProjectedRoutePrefetchArtifact,
  isProjectedRoutePrefetchArtifact,
  tryCreateProjectedRouteWarmupArtifact,
};
