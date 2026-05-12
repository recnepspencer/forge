import { resourceDelivery } from "../resource/delivery/resource_delivery.js";
import { resourcePatch } from "../resource/reconciliation/resource_patch.js";
import { readApiFamilyReconcileCapabilities } from "./api_family_reconcile_capabilities.js";

function attachApiFamilyDeliveryHelpers(familyKind, family, declaration) {
  const wrappedFamily = Object.create(
    Object.getPrototypeOf(family),
    Object.getOwnPropertyDescriptors(family),
  );
  Object.defineProperty(wrappedFamily, "delivery", {
    value: createApiFamilyDeliveryHelpers(familyKind, declaration),
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return Object.freeze(wrappedFamily);
}

function createApiFamilyDeliveryHelpers(familyKind, declaration) {
  const helpers = {
    replace(options) {
      return resourceDelivery.replace(options);
    },
    patch(options) {
      return resourceDelivery.patch(options);
    },
    invalidate(options) {
      return resourceDelivery.invalidate(options);
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
      return resourceDelivery.patch(
        withPatchDelivery(
          options,
          resourcePatch.field({
            field: options.field,
            value: options.value,
          }),
        ),
      );
    };
  }
  if (capabilities.hasRegions) {
    helpers.region = function region(options) {
      return resourceDelivery.patch(
        withPatchDelivery(
          options,
          resourcePatch.region({
            region: options.region,
            value: options.value,
          }),
        ),
      );
    };
  }
  if (capabilities.hasJsonPaths) {
    helpers.jsonPath = function jsonPath(options) {
      return resourceDelivery.patch(
        withPatchDelivery(
          options,
          resourcePatch.jsonPath({
            path: options.path,
            value: options.value,
          }),
        ),
      );
    };
  }
  helpers.item = function item(options) {
    return resourceDelivery.patch(
      withPatchDelivery(
        options,
        resourcePatch.item({
          itemId: options.itemId,
          nextItem: options.nextItem,
        }),
      ),
    );
  };
  if (capabilities.hasAspects) {
    helpers.itemAspect = function itemAspect(options) {
      return resourceDelivery.patch(
        withPatchDelivery(
          options,
          resourcePatch.itemAspect({
            itemId: options.itemId,
            aspect: options.aspect,
            value: options.value,
          }),
        ),
      );
    };
  }
  if (capabilities.admitsSummary) {
    helpers.summary = function summary(options) {
      return resourceDelivery.patch(
        withPatchDelivery(
          options,
          resourcePatch.summary({
            summary: options.summary,
            value: options.value,
          }),
        ),
      );
    };
  }
  return Object.freeze(helpers);
}

function withPatchDelivery(options, patch) {
  return {
    packetId: options.packetId,
    basisId: options.basisId,
    nextBasisId: options.nextBasisId,
    patch,
  };
}

export { attachApiFamilyDeliveryHelpers };
