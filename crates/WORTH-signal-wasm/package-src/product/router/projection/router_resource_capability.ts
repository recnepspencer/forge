import {
  createCanonicalDigest,
} from "../url_authority/router_verification_packages.js";
import {
  routeResourceMatchesWarmupTrigger,
} from "./router_resource_declaration.js";

function createProjectedRouteResourceCapability(routeLocation, name, resourceDeclaration) {
  return Object.freeze({
    kind: "projectedRouteResourceCapability",
    routeId: routeLocation.routeId,
    name,
    prefetchPosture() {
      return resourceDeclaration.prefetch;
    },
    prefetch(trigger = resourceDeclaration.prefetch) {
      if (trigger !== resourceDeclaration.prefetch) {
        throw new TypeError(
          `projected route resource "${routeLocation.routeId}.${name}" requires prefetch trigger "${resourceDeclaration.prefetch}", not "${trigger}"`,
        );
      }
      return createRouteResourceWarmupArtifact(
        routeLocation,
        name,
        resourceDeclaration,
        trigger,
      );
    },
    warmup(trigger = resourceDeclaration.prefetch) {
      return createRouteResourceWarmupArtifact(
        routeLocation,
        name,
        resourceDeclaration,
        trigger,
      );
    },
    verification() {
      return {
        routeResourceBindingDigest: createCanonicalDigest("route-resource-binding", {
          routeId: routeLocation.routeId,
          name,
          prefetchPosture: resourceDeclaration.prefetch,
        }),
      };
    },
  });
}

function createAdmittedRouteResourceCapability(routeLocation, name, resourceDeclaration) {
  return Object.freeze({
    kind: "admittedRouteResourceCapability",
    routeId: routeLocation.routeId,
    name,
    prefetchPosture() {
      return resourceDeclaration.prefetch;
    },
    line() {
      return materializeRouteResourceLine(routeLocation, resourceDeclaration);
    },
    current() {
      return readRouteResourceCurrentSummary(
        materializeRouteResourceLine(routeLocation, resourceDeclaration),
      );
    },
    verification() {
      return {
        routeResourceBindingDigest: createCanonicalDigest("route-resource-binding", {
          routeId: routeLocation.routeId,
          name,
          prefetchPosture: resourceDeclaration.prefetch,
        }),
      };
    },
  });
}

function materializeRouteResourceLine(routeLocation, resourceDeclaration) {
  return resourceDeclaration.family.line(
    resourceDeclaration.resolveParams({
      routeId: routeLocation.routeId,
      href: routeLocation.href,
      params: routeLocation.params,
      search: routeLocation.search,
      hash: routeLocation.hash,
    }),
  );
}

function readRouteResourceCurrentSummary(line) {
  return Object.freeze({
    descriptor: line.descriptor(),
    status: line.status(),
    freshness: line.freshness(),
    diagnosticsSummary: line.diagnosticsSummary(),
  });
}

function createRouteResourceWarmupArtifact(
  routeLocation,
  name,
  resourceDeclaration,
  trigger,
) {
  if (!routeResourceMatchesWarmupTrigger(resourceDeclaration, trigger)) {
    throw new TypeError(
      `projected route resource "${routeLocation.routeId}.${name}" does not warm for trigger "${trigger}"`,
    );
  }
  const line = materializeRouteResourceLine(routeLocation, resourceDeclaration);
  return Object.freeze({
    kind: "routeResourcePrefetch",
    routeId: routeLocation.routeId,
    href: routeLocation.href,
    name,
    prefetchPosture: resourceDeclaration.prefetch,
    trigger,
    line() {
      return line;
    },
    current() {
      return readRouteResourceCurrentSummary(line);
    },
    free() {
      line.free();
    },
    [Symbol.dispose]() {
      line.free();
    },
    verification() {
      const summary = readRouteResourceCurrentSummary(line);
      return {
        routeResourcePrefetchDigest: createCanonicalDigest("route-resource-prefetch", {
          routeId: routeLocation.routeId,
          href: routeLocation.href,
          name,
          prefetchPosture: resourceDeclaration.prefetch,
          trigger,
          familyId: summary.descriptor.family.familyId,
          canonicalKey: summary.descriptor.canonicalParams.canonicalKey,
          status: summary.status,
          freshness: summary.freshness,
        }),
      };
    },
  });
}

export {
  createAdmittedRouteResourceCapability,
  createProjectedRouteResourceCapability,
  createRouteResourceWarmupArtifact,
};
