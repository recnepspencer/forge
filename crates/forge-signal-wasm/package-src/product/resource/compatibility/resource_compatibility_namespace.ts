import { createCollectionFamily } from "../families/collection_family.js";
import { createDetailFamily } from "../families/detail_family.js";
import { nextResourceFamilyId } from "../families/family_id_sequence.js";
import { createPagedFamily } from "../families/paged_family.js";
import {
  admitExternalCollectionDefinition,
  admitExternalDetailDefinition,
  admitExternalPagedDefinition,
} from "./resource_external_definition.js";
import { resourceExternalDelivery } from "./resource_external_delivery.js";

function createResourceCompatibilityNamespace(signalNamespace, rawSignals) {
  return Object.freeze({
    delivery: resourceExternalDelivery,
    detail(definition) {
      const admitted = admitExternalDetailDefinition(definition);
      return createDetailFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "detail"),
        admitted.declaration,
        admitted.compatibility,
      );
    },
    collection(definition) {
      const admitted = admitExternalCollectionDefinition(definition);
      return createCollectionFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "collection"),
        admitted.declaration,
        admitted.compatibility,
      );
    },
    paged(definition) {
      const admitted = admitExternalPagedDefinition(definition);
      return createPagedFamily(
        signalNamespace,
        nextResourceFamilyId(rawSignals, "paged"),
        admitted.declaration,
        admitted.compatibility,
      );
    },
  });
}

export { createResourceCompatibilityNamespace };
