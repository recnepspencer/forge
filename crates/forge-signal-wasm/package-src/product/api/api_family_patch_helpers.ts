import { resourcePatch } from "../resource/reconciliation/resource_patch.js";
import { readApiFamilyReconcileCapabilities } from "./api_family_reconcile_capabilities.js";

function attachApiFamilyPatchHelpers(familyKind, family, declaration) {
  const wrappedFamily = Object.create(
    Object.getPrototypeOf(family),
    Object.getOwnPropertyDescriptors(family),
  );
  Object.defineProperty(wrappedFamily, "patch", {
    value: createApiFamilyPatchHelpers(familyKind, declaration),
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return Object.freeze(wrappedFamily);
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
  if (capabilities.hasFields) {
    helpers.field = function field(options) {
      return resourcePatch.field(options);
    };
  }
  if (capabilities.hasRegions) {
    helpers.region = function region(options) {
      return resourcePatch.region(options);
    };
  }
  if (capabilities.hasJsonPaths) {
    helpers.jsonPath = function jsonPath(options) {
      return resourcePatch.jsonPath(options);
    };
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
