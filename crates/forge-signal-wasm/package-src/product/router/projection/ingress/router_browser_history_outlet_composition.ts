import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

function createBrowserHistoryOutletCompositionArtifact(
  routeId,
  href,
  layouts,
  outlet,
  outlets,
) {
  const frozenLayouts = Object.freeze(layouts.slice());
  const frozenOutlets = Object.freeze(outlets.slice());
  const summary = Object.freeze({
    layoutCount: frozenLayouts.length,
    outletCount: frozenOutlets.length,
    layoutRouteIds: Object.freeze(frozenLayouts.map((layout) => layout.routeId)),
    outletIds: Object.freeze(frozenOutlets.map((contract) => contract.outletId)),
    occupantRouteIds: Object.freeze(
      frozenOutlets.map((contract) => contract.occupantRouteId),
    ),
  });
  const verification = Object.freeze({
    outletCompositionDigest: createCanonicalDigest("browser-history-outlet-composition", {
      routeId,
      href,
      layoutVerificationDigests: frozenLayouts.map(
        (layout) => layout.verification().canonicalUrlDigest,
      ),
      outletContractDigests: frozenOutlets.map(
        (contract) => contract.verification().outletContractDigest,
      ),
      activeOutletDigest: outlet.verification().outletContractDigest,
      summary,
    }),
  });
  return Object.freeze({
    kind: "browserHistoryOutletComposition",
    routeId,
    href,
    layouts() {
      return frozenLayouts;
    },
    outlet() {
      return outlet;
    },
    outlets() {
      return frozenOutlets;
    },
    summary() {
      return summary;
    },
    verification() {
      return verification;
    },
  });
}

export {
  createBrowserHistoryOutletCompositionArtifact,
};
