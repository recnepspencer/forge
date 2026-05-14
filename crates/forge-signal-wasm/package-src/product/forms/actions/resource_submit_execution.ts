import { readResourceLineHandle } from "../sources/form_sources.js";
import { readResourceLineProof } from "../sources/resource_line_proof.js";
import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";

export function executeResourceBackedSubmit(source, fieldDeclarations, plan) {
  if (plan.id !== "submit") {
    return null;
  }
  const line = readResourceLineHandle(source);
  if (
    line === null ||
    typeof line.patch !== "function" ||
    typeof line.reconciliation !== "function" ||
    plan.patch.empty
  ) {
    return null;
  }
  const reconciliation = line.reconciliation();
  const loweredPlans = [];
  for (const operation of plan.patch.operations) {
    const declaration = fieldDeclarations.find((field) => field.id === operation.field);
    if (!declaration) {
      return deniedResourceSubmit(`resource-backed submit could not resolve form field "${operation.field}"`);
    }
    const mapped = mapOperationToResourcePatch(declaration.path, operation.value, reconciliation);
    if (mapped === null) {
      return deniedResourceSubmit(
        `resource-backed submit has no declared resource locus for form field path "${declaration.path}"`,
      );
    }
    loweredPlans.push(Object.freeze({
      field: declaration.id,
      path: declaration.path,
      locusKind: mapped.locusKind,
      locus: mapped.locus,
      patch: mapped.patch,
      patchKind: mapped.patch.kind,
    }));
  }
  const lowered = [];
  for (const loweredPlan of loweredPlans) {
    const patchResult = line.patch(loweredPlan.patch);
    const latest = line.diagnosticsSummary().latest;
    lowered.push(Object.freeze({
      field: loweredPlan.field,
      path: loweredPlan.path,
      locusKind: loweredPlan.locusKind,
      locus: loweredPlan.locus,
      patchKind: loweredPlan.patchKind,
      patchResultKind: patchResult.kind,
      patchScope: patchResult.scope,
      effectDigest: latest.effect === null ? null : stableValueDigest(latest.effect),
      basisId: latest.basisCurrentId ?? null,
    }));
  }
  const request = line.request();
  const summary = line.summary();
  const mutationResponse = line.mutationResponse();
  const proof = readResourceLineProof(line, request, summary, mutationResponse);
  const canonicalValue = cloneFormValue(line.value());
  const resourceSubmission = Object.freeze({
    sourceKind: "resourceLine",
    patchCount: lowered.length,
    patches: Object.freeze(lowered),
    effectProfile: proof.effectProfile,
    rollback: proof.rollback,
    visibleSelection: proof.visibleSelection,
    mutationResponse: proof.mutationResponse,
    verification: proof.verification,
    digest: stableValueDigest({
      patches: lowered,
      effectProfile: proof.effectProfile,
      rollback: proof.rollback,
      visibleSelection: proof.visibleSelection,
      mutationResponse: proof.mutationResponse,
      verification: proof.verification,
    }),
  });
  return Object.freeze({
    resultKind: "fulfilled",
    reason: "resource-backed submit applied through resource line patch effects",
    effectStarted: true,
    canonicalValue,
    resourceSubmission,
  });
}

function deniedResourceSubmit(reason) {
  return Object.freeze({
    resultKind: "denied",
    reason,
    effectStarted: false,
    resourceSubmission: null,
  });
}

function mapOperationToResourcePatch(path, value, reconciliation) {
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
  return null;
}
