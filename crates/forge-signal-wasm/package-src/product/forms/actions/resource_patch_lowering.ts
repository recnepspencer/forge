import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";

export function stageResourcePatchLowering(line, fieldDeclarations, patchPlan, actionId) {
  if (patchPlan.replacement !== null) {
    if (line.reconciliation().broadReplace !== true) {
      return denied(
        `resource-line action "${actionId}" requires whole-resource replace admission for ${patchPlan.replacement.reason}`,
      );
    }
    return Object.freeze({
      kind: "admitted",
      loweredPlans: Object.freeze([Object.freeze({
        field: null,
        path: null,
        locusKind: "wholeForm",
        locus: "wholeForm",
        patch: resourcePatch.replace(patchPlan.replacement.value),
        patchKind: "replace",
        operationKind: "set",
      })]),
    });
  }
  const reconciliation = line.reconciliation();
  const loweredPlans = [];
  for (const operation of patchPlan.operations) {
    const declaration = fieldDeclarations.find((field) => field.id === operation.field);
    if (!declaration) {
      return denied(
        `resource-line action "${actionId}" could not resolve form field "${operation.field}"`,
      );
    }
    const mapped = mapOperationToResourcePatch(operation, declaration, reconciliation, actionId);
    if (mapped.kind === "denied") {
      return mapped;
    }
    loweredPlans.push(mapped.loweredPlan);
  }
  return Object.freeze({
    kind: "admitted",
    loweredPlans: Object.freeze(loweredPlans),
  });
}

function mapOperationToResourcePatch(operation, declaration, reconciliation, actionId) {
  if (operation.kind === "set" || operation.kind === "attach" || operation.kind === "detach") {
    const mapped = mapValueOperationToResourcePatch(operation, declaration, reconciliation);
    if (mapped === null) {
      return denied(
        `resource-line action "${actionId}" has no declared resource locus for form field path "${declaration.path}"`,
      );
    }
    return Object.freeze({
      kind: "admitted",
      loweredPlan: Object.freeze({
        field: declaration.id,
        path: declaration.path,
        locusKind: mapped.locusKind,
        locus: mapped.locus,
        patch: mapped.patch,
        patchKind: mapped.patch.kind,
        operationKind: operation.kind,
      }),
    });
  }
  if (declaration.resourceLocus?.kind !== "collectionItems") {
    return denied(
      `resource-line action "${actionId}" requires repeated field "${declaration.id}" to declare resourceLocus.collectionItems`,
    );
  }
  if (reconciliation.narrowItem !== true) {
    return denied(
      `resource-line action "${actionId}" requires collection item patch admission on the backing resource line`,
    );
  }
  const loweredPlan = operation.kind === "replaceItem"
    ? Object.freeze({
        field: declaration.id,
        path: declaration.path,
        locusKind: "collectionItem",
        locus: operation.itemId,
        patch: resourcePatch.item({
          itemId: operation.itemId,
          nextItem: operation.value,
        }),
        patchKind: "item",
        operationKind: operation.kind,
      })
    : operation.kind === "insertItem"
      ? Object.freeze({
          field: declaration.id,
          path: declaration.path,
          locusKind: "collectionItem",
          locus: operation.itemId,
          patch: resourcePatch.insert({
            itemId: operation.itemId,
            placement: operation.placement,
            nextItem: operation.value,
          }),
          patchKind: "insert",
          operationKind: operation.kind,
        })
      : Object.freeze({
          field: declaration.id,
          path: declaration.path,
          locusKind: "collectionItem",
          locus: operation.itemId,
          patch: resourcePatch.delete({
            itemId: operation.itemId,
          }),
          patchKind: "delete",
          operationKind: operation.kind,
        });
  return Object.freeze({
    kind: "admitted",
    loweredPlan,
  });
}

function mapValueOperationToResourcePatch(operation, declaration, reconciliation) {
  const value = operation.kind === "detach" ? null : operation.value;
  if (declaration.resourceLocus?.kind === "field") {
    return Object.freeze({
      locusKind: "field",
      locus: declaration.resourceLocus.field,
      patch: resourcePatch.field({
        field: declaration.resourceLocus.field,
        value,
      }),
    });
  }
  if (declaration.resourceLocus?.kind === "jsonPath") {
    return Object.freeze({
      locusKind: "jsonPath",
      locus: declaration.resourceLocus.path,
      patch: resourcePatch.jsonPath({
        path: declaration.resourceLocus.path,
        value,
      }),
    });
  }
  if (declaration.resourceLocus?.kind === "region") {
    return Object.freeze({
      locusKind: "region",
      locus: declaration.resourceLocus.region,
      patch: resourcePatch.region({
        region: declaration.resourceLocus.region,
        value,
      }),
    });
  }
  if (declaration.resourceLocus?.kind === "itemAspect") {
    return Object.freeze({
      locusKind: "aspect",
      locus: declaration.resourceLocus.aspect,
      patch: resourcePatch.itemAspect({
        itemId: declaration.resourceLocus.itemId,
        aspect: declaration.resourceLocus.aspect,
        value,
      }),
    });
  }
  if (declaration.resourceLocus?.kind === "summary") {
    return Object.freeze({
      locusKind: "summary",
      locus: declaration.resourceLocus.summary,
      patch: resourcePatch.summary({
        summary: declaration.resourceLocus.summary,
        value,
      }),
    });
  }
  const path = declaration.path;
  if (reconciliation.fieldNames.includes(path)) {
    return Object.freeze({
      locusKind: "field",
      locus: path,
      patch: resourcePatch.field({
        field: path,
        value,
      }),
    });
  }
  if (reconciliation.jsonPathNames.includes(path)) {
    return Object.freeze({
      locusKind: "jsonPath",
      locus: path,
      patch: resourcePatch.jsonPath({
        path,
        value,
      }),
    });
  }
  if (reconciliation.regionNames.includes(path)) {
    return Object.freeze({
      locusKind: "region",
      locus: path,
      patch: resourcePatch.region({
        region: path,
        value,
      }),
    });
  }
  if (reconciliation.aspectNames.includes(path)) {
    return null;
  }
  if (reconciliation.summaryNames.includes(path)) {
    return Object.freeze({
      locusKind: "summary",
      locus: path,
      patch: resourcePatch.summary({
        summary: path,
        value,
      }),
    });
  }
  return null;
}

function denied(reason) {
  return Object.freeze({
    kind: "denied",
    reason,
  });
}
