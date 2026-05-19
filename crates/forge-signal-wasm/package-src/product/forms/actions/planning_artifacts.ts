import { stableValueDigest } from "../values/value_paths.js";
import {
  isResolvedResourceEffectProfile, profileDigest,
} from "./resource_effect_profile_binding.js";
import {
  isResolvedResourceActionBinding,
} from "./resource_action_binding.js";

export function createActionProofBinding(form, fieldDeclarations) {
  const sourceDigest = stableValueDigest(form.source());
  const draftDigest = stableValueDigest(form.draft());
  const effectiveDigest = stableValueDigest(form.effective());
  const schemaDigest = stableValueDigest(
    fieldDeclarations.map((field) => ({
      id: field.id,
      path: field.path,
      inputAdapterTier: field.inputAdapter.tier,
      resourceLocusKind: field.resourceLocus?.kind ?? null,
      resourceLocusPlacement: field.resourceLocus?.placement ?? null,
    })),
  );
  return Object.freeze({
    sourceDigest,
    draftDigest,
    effectiveDigest,
    schemaDigest,
    bindingDigest: stableValueDigest({
      sourceDigest,
      draftDigest,
      effectiveDigest,
      schemaDigest,
    }),
  });
}

export function actionCatalogEntry(entry) {
  const resourceAction = isResolvedResourceActionBinding(entry.resourceAction)
    ? entry.resourceAction
    : Object.freeze({
      declared: false,
      action: null,
      source: entry.id === "submit" ? "submitPatchPlan" : "none",
      blockers: Object.freeze([]),
    });
  const resourceEffectProfile = isResolvedResourceEffectProfile(entry.resourceEffectProfile)
    ? entry.resourceEffectProfile
    : Object.freeze({
      declared: entry.resourceEffectProfile === null ? null : profileDigest(entry.resourceEffectProfile),
      effective: null,
      source: entry.resourceEffectProfile === null ? "none" : "declaredWithoutResourceLine",
      closeoutMatrixDigest: null,
    });
  return Object.freeze({
    id: entry.id,
    name: entry.name,
    kind: entry.kind,
    label: entry.label,
    patchPolicy: entry.patchPolicy,
    admissionCapability: entry.admissionCapability,
    destructive: entry.destructive,
    idempotency: entry.idempotency,
    effectPolicy: entry.effectPolicy,
    hostEffect: entry.hostEffect,
    hostRequirements: entry.hostRequirements,
    resourceAction: Object.freeze({
      declared: resourceAction.declared,
      action: resourceAction.action,
      source: resourceAction.source,
    }),
    resourceEffectProfile,
    schema: entry.schema,
    step: entry.step,
  });
}

export function actionSummary(plans) {
  const summary = {
    total: plans.length,
    accepted: 0,
    denied: 0,
    unavailable: 0,
    cancelled: 0,
    superseded: 0,
    rejected: 0,
    fulfilled: 0,
    noOp: 0,
    destructive: 0,
    step: 0,
  };
  for (const plan of plans) {
    summary[plan.status] += 1;
    if (plan.patch.empty) {
      summary.noOp += 1;
    }
    if (plan.destructive) {
      summary.destructive += 1;
    }
    if (plan.kind === "step") {
      summary.step += 1;
    }
  }
  return Object.freeze(summary);
}

export function actionCounters(declarations, plans) {
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    incrementalStatus: "notIncremental",
    declarations: declarations.length,
    plans: plans.length,
    deniedPlans: plans.filter((plan) => plan.status === "denied").length,
    destructivePlans: plans.filter((plan) => plan.destructive).length,
    stepPlans: plans.filter((plan) => plan.kind === "step").length,
    routeAuthorityRequiredPlans: plans.filter((plan) => plan.diagnostics.routeSemantics === "routeAuthorityRequired").length,
    hostRequiredPlans: plans.filter((plan) => plan.host.requirements.length > 0).length,
    nonEmptyPatchPlans: plans.filter((plan) => !plan.patch.empty).length,
  });
}

export function actionReportDigests(plans) {
  const catalogDigest = stableValueDigest(plans.map((plan) => ({
    id: plan.id,
    kind: plan.kind,
    patchPolicy: plan.patchPolicy,
    admissionCapability: plan.admissionCapability,
    destructive: plan.destructive,
    idempotency: plan.idempotency,
    effectPolicy: plan.effectPolicy,
    hostEffect: plan.hostEffect,
    hostRequirements: plan.hostRequirements,
    resourceEffectProfile: plan.resourceEffectProfile,
    schema: plan.schema,
    step: plan.step,
  })));
  const readinessAdmissionDigest = stableValueDigest(plans.map((plan) => ({
    id: plan.id,
    status: plan.status,
    blockers: plan.readiness.blockers,
    host: plan.host,
    admission: plan.admission,
    regulatedActionBindings: plan.regulatedActionBindings,
  })));
  const planDigests = Object.freeze(
    Object.fromEntries(plans.map((plan) => [plan.id, plan.planDigest])),
  );
  return Object.freeze({
    catalogDigest,
    readinessAdmissionDigest,
    planDigestSetDigest: stableValueDigest(planDigests),
    submitPlanDigest: planDigests.submit ?? null,
    planDigests,
  });
}
