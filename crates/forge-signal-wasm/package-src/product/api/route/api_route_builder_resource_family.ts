import { attachApiFamilyDeliveryHelpers } from "../api_family_delivery_helpers.js";
import { attachApiFamilyPatchHelpers } from "../api_family_patch_helpers.js";

function attachDetailApiRouteFamily(signalNamespace, lowered) {
  return attachApiFamilyDeliveryHelpers(
    "detail",
    attachApiFamilyPatchHelpers(
      "detail",
      signalNamespace.resource.detail(lowered),
      lowered,
    ),
    lowered,
  );
}

function attachCollectionApiRouteFamily(familyKind, signalNamespace, lowered) {
  return attachApiFamilyDeliveryHelpers(
    familyKind,
    attachApiFamilyPatchHelpers(
      familyKind,
      signalNamespace.resource[familyKind](lowered),
      lowered,
    ),
    lowered,
  );
}

export {
  attachCollectionApiRouteFamily,
  attachDetailApiRouteFamily,
};
