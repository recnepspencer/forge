import { resourcePatch } from "../resource/reconciliation/resource_patch.js";

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
  const reconcile = declaration.reconcile ?? null;
  if (reconcile === null) {
    return Object.freeze(helpers);
  }
  helpers.item = function item(options) {
    return resourcePatch.item(options);
  };
  const aspectNames = Object.keys(reconcile.aspects?.definitions ?? {});
  if (aspectNames.length > 0) {
    helpers.itemAspect = function itemAspect(options) {
      return resourcePatch.itemAspect(options);
    };
  }
  const summaries = reconcile.summaries ?? null;
  const summaryNames = Object.keys(summaries?.definitions ?? {});
  const admitsSummary =
    summaryNames.length > 0
    && (familyKind === "collection" || summaries.patchScope === "pageWindow");
  if (admitsSummary) {
    helpers.summary = function summary(options) {
      return resourcePatch.summary(options);
    };
  }
  return Object.freeze(helpers);
}

export { attachApiFamilyPatchHelpers };
