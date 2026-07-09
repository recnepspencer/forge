import {
  createCanonicalDigest,
} from "../url_authority/router_verification_packages.js";

function createProjectedCandidateVerification(
  canonicalUrlAuthority,
  projectedRouteCapability,
  projectedLayoutPlacements,
  projectedOutletContracts,
) {
  const projectedOutletDescriptors = projectedOutletContracts.map((contract) => contract.descriptor());
  const routeCompositionDigest = createCanonicalDigest("projected-route-composition", {
    routeId: projectedRouteCapability.routeId,
    controllerNames: projectedRouteCapability.controllerNames(),
    graphNames: projectedRouteCapability.graphNames(),
    graphIds: projectedRouteCapability.graphNames().map(
      (name) => projectedRouteCapability.graph(name).graphId,
    ),
    resourceNames: projectedRouteCapability.resourceNames(),
    resourcePrefetchPostures: projectedRouteCapability.resourceNames().map(
      (name) => projectedRouteCapability.resource(name).prefetchPosture(),
    ),
  });
  return Object.freeze({
    canonicalUrlDigest: canonicalUrlAuthority.canonicalUrlDigest,
    projectedRouteDigest: createCanonicalDigest("projected-route", {
      routeId: projectedRouteCapability.routeId,
      href: projectedRouteCapability.href,
    }),
    routeCompositionDigest,
    layoutStackDigest: createCanonicalDigest(
      "projected-layout-stack",
      projectedLayoutPlacements.map((placement) => ({
        routeId: placement.routeId,
        outletId: placement.outletId,
      })),
    ),
    outletDigest: projectedOutletContracts.at(-1).verification().outletDigest,
    outletStackDigest: createCanonicalDigest("projected-outlet-stack", projectedOutletDescriptors),
    projectedCandidateDigest: createCanonicalDigest("projected-candidate", {
      routeId: projectedRouteCapability.routeId,
      canonicalUrlDigest: canonicalUrlAuthority.canonicalUrlDigest,
      routeCompositionDigest,
      layouts: projectedLayoutPlacements.map((placement) => ({
        routeId: placement.routeId,
        outletId: placement.outletId,
      })),
      outlets: projectedOutletDescriptors,
    }),
  });
}

export {
  createProjectedCandidateVerification,
};
