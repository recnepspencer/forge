import { resourcePatch } from "../resource/reconciliation/resource_patch.js";
import { readApiFamilyReconcileCapabilities } from "./api_family_reconcile_capabilities.js";

function attachApiFamilyPatchHelpers(familyKind, family, declaration) {
  return Object.freeze({
    ...family,
    patch: createApiFamilyPatchHelpers(familyKind, declaration),
  });
}

function createApiFamilyPatchHelpers(familyKind, declaration) {
  const helpers = {
    replace(nextValue) {
      return resourcePatch.replace(nextValue);
    },
  };
  const capabilities = readApiFamilyReconcileCapabilities(
    familyKind,
    declaration,
  );
  if (!capabilities.hasReconcile) {
    return Object.freeze(helpers);
  }
  helpers.item = function item(options) {
    return resourcePatch.item(options);
  };
  if (capabilities.hasAspects) {
    helpers.itemAspect = function itemAspect(options) {
      return resourcePatch.itemAspect(options);
    };
  }
  if (capabilities.admitsSummary) {
    helpers.summary = function summary(options) {
      return resourcePatch.summary(options);
    };
  }
  return Object.freeze(helpers);
}

export { attachApiFamilyPatchHelpers };
