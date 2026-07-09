import {
  ROUTE_OUTLET_CONTRACT,
} from "../router_symbols.js";
import {
  createCanonicalDigest,
} from "../url_authority/router_verification_packages.js";

function finalizeProjectedOutletContracts(layoutPlacementRecords, projectedRouteCapability) {
  const layoutPlacements = Object.freeze(layoutPlacementRecords.map((record) => record.placement));
  const outletContracts = Object.freeze(layoutPlacementRecords.length === 0
    ? [createProjectedOutletContract(null, projectedRouteCapability)]
    : layoutPlacementRecords.map((record, index) => createProjectedOutletContract(
      record.placement,
      index === layoutPlacementRecords.length - 1
        ? projectedRouteCapability
        : layoutPlacementRecords[index + 1].placement,
    )));
  for (let index = 0; index < layoutPlacementRecords.length; index += 1) {
    layoutPlacementRecords[index].bindOutlet(outletContracts[index]);
  }
  return {
    layoutPlacements,
    outletContracts,
  };
}

function createProjectedOutletContract(parentLayoutPlacement, occupant) {
  const descriptor = Object.freeze({
    outletId: parentLayoutPlacement === null ? null : parentLayoutPlacement.outletId,
    parentLayoutRouteId: parentLayoutPlacement === null ? null : parentLayoutPlacement.routeId,
    occupantRouteId: occupant.routeId,
    occupantKind: occupant.kind,
  });
  const occupantDigest = createProjectedOutletOccupantDigest(occupant);
  const verification = Object.freeze({
    outletDigest: createCanonicalDigest("projected-outlet", descriptor),
    occupantDigest,
    outletContractDigest: createCanonicalDigest("projected-outlet-contract", {
      ...descriptor,
      occupantDigest,
    }),
  });
  return Object.freeze({
    [ROUTE_OUTLET_CONTRACT]: true,
    kind: "projectedOutletContract",
    outletId: descriptor.outletId,
    parentLayoutRouteId: descriptor.parentLayoutRouteId,
    occupantRouteId: descriptor.occupantRouteId,
    occupant() {
      return occupant;
    },
    descriptor() {
      return descriptor;
    },
    verification() {
      return verification;
    },
  });
}

function createProjectedOutletOccupantDigest(occupant) {
  if (occupant.kind === "projectedLayoutPlacement") {
    return createCanonicalDigest("projected-outlet-layout-occupant", {
      routeId: occupant.routeId,
      verification: occupant.verification().canonicalUrlDigest,
    });
  }
  return createCanonicalDigest("projected-outlet-route-occupant", {
    routeId: occupant.routeId,
    verification: occupant.verification().canonicalUrlDigest,
  });
}

export {
  finalizeProjectedOutletContracts,
};
