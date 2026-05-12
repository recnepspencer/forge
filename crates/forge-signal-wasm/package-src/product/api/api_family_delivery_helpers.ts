import { resourceDelivery } from "../resource/delivery/resource_delivery.js";
import { resourcePatch } from "../resource/reconciliation/resource_patch.js";
import { readApiFamilyReconcileCapabilities } from "./api_family_reconcile_capabilities.js";

function attachApiFamilyDeliveryHelpers(familyKind, family, declaration) {
  return Object.freeze({
    ...family,
    delivery: createApiFamilyDeliveryHelpers(familyKind, declaration),
  });
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
