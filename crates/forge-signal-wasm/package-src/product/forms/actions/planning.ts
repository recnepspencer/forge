import { FormDeclarationError } from "../form_errors.js";
import { hostRequirementBlockers } from "../host/artifacts.js";
import { controllerLocalNavigationBlockers } from "../navigation/semantics.js";
import { stableValueDigest } from "../values/value_paths.js";
import { recoveryActionsForBlockers } from "./recovery.js";

export function planActions(actionDeclarations, form, fieldDeclarations) {
  const patchPlan = form.patchPlan();
  const validation = form.validation();
  const availability = form.availability();
  const admission = form.admission();
  const readiness = form.readiness();
  const host = form.host();
  const steps = form.steps();
  const navigation = form.navigation();
  const binding = currentActionBinding(form, fieldDeclarations, patchPlan);
  const plans = actionDeclarations.map((declaration) =>
    actionPlan(declaration, {
      patchPlan,
      validation,
      availability,
      admission,
      readiness,
      host,
      steps,
      navigation,
      binding,
    }),
  );
  return Object.freeze({
    catalog: Object.freeze(actionDeclarations.map(actionCatalogEntry)),
    plans: Object.freeze(plans),
    host,
    summary: actionSummary(plans),
    counters: actionCounters(actionDeclarations, plans),
    digests: actionReportDigests(actionDeclarations, plans),
  });
}

export function findActionPlan(actionDeclarations, form, fieldDeclarations, actionId) {
  const plan = planActions(actionDeclarations, form, fieldDeclarations)
    .plans
    .find((entry) => entry.id === actionId);
  if (!plan) {
    throw new FormDeclarationError("form action is not declared", { actionId });
  }
  return plan;
}

function actionPlan(declaration, context) {
  const blockers = actionReadinessBlockers(declaration, context);
  const status = blockers.length === 0 ? "accepted" : "denied";
  const actionSchemaDigest = stableValueDigest(declaration.schema);
  const effectDigest = stableValueDigest({
    effectPolicy: declaration.effectPolicy,
    hostEffect: declaration.hostEffect,
    step: declaration.step,
  });
  const proof = Object.freeze({
    sourceDigest: context.binding.sourceDigest,
    draftDigest: context.binding.draftDigest,
    effectiveDigest: context.binding.effectiveDigest,
    patchDigest: context.binding.patchDigest,
    schemaDigest: context.binding.schemaDigest,
    actionSchemaDigest,
    effectDigest,
    bindingDigest: context.binding.bindingDigest,
  });
  const planSeed = {
    id: declaration.id,
    kind: declaration.kind,
    patchPolicy: declaration.patchPolicy,
    admissionCapability: declaration.admissionCapability,
    destructive: declaration.destructive,
    idempotency: declaration.idempotency,
    effectPolicy: declaration.effectPolicy,
    hostEffect: declaration.hostEffect,
    hostRequirements: declaration.hostRequirements,
    actionSchemaDigest,
    effectDigest,
    step: declaration.step,
    blockers,
    proof,
  };
  const planDigest = stableValueDigest(planSeed);
  return Object.freeze({
    ...actionCatalogEntry(declaration),
    status,
    resultKind: status,
    readiness: Object.freeze({
      action: declaration.id,
      canRun: blockers.length === 0,
      blockers: Object.freeze(blockers),
    }),
    recoveryActions: recoveryActionsForBlockers(blockers),
    patch: actionPatchArtifact(declaration, context.patchPlan),
    validation: Object.freeze({
      summary: context.validation.summary,
      artifactCount: context.validation.artifacts.length,
    }),
    availability: Object.freeze({
      summary: context.availability.summary,
      artifactCount: context.availability.artifacts.length,
    }),
    admission: Object.freeze({
      summary: context.admission.summary,
      artifactCount: context.admission.artifacts.length,
    }),
    host: Object.freeze({
      requirements: declaration.hostRequirements,
      blockers: hostRequirementBlockers(context.host, declaration.hostRequirements, declaration.id),
      digest: context.host.digest,
    }),
    proof,
    planDigest,
    regulatedActionBindings: regulatedActionBindings(declaration, context.admission, planDigest),
    diagnostics: Object.freeze({
      deniedBeforeEffects: blockers.length > 0,
      consumesLoweredPlan: true,
      routeSemantics: routeSemantics(declaration),
      repeatedAttemptPolicy: declaration.idempotency,
    }),
  });
}

function actionCatalogEntry(declaration) {
  return Object.freeze({
    id: declaration.id,
    name: declaration.name,
    kind: declaration.kind,
    label: declaration.label,
    patchPolicy: declaration.patchPolicy,
    admissionCapability: declaration.admissionCapability,
    destructive: declaration.destructive,
    idempotency: declaration.idempotency,
    effectPolicy: declaration.effectPolicy,
    hostEffect: declaration.hostEffect,
    hostRequirements: declaration.hostRequirements,
    schema: declaration.schema,
    step: declaration.step,
  });
}

function actionPatchArtifact(declaration, patchPlan) {
  if (declaration.patchPolicy === "ignore") {
    return Object.freeze({
      policy: "ignore",
      empty: true,
      operations: Object.freeze([]),
      equivalenceDigest: stableValueDigest({ ignoredPatchDigest: patchPlan.equivalenceDigest }),
    });
  }
  return Object.freeze({
    policy: declaration.patchPolicy,
    empty: patchPlan.empty,
    semanticDirty: patchPlan.semanticDirty,
    operations: patchPlan.operations,
    blocked: patchPlan.blocked,
    equivalenceDigest: patchPlan.equivalenceDigest,
  });
}

function actionReadinessBlockers(declaration, context) {
  const blockers = [...hostRequirementBlockers(context.host, declaration.hostRequirements, declaration.id)];
  if (declaration.step?.routeCoupled === true) {
    blockers.push(Object.freeze({
      kind: "action:deferred",
      action: declaration.id,
      reason: "route-coupled step action is deferred until router integration exists",
    }));
  }
  if (declaration.step?.routeCoupled !== true && declaration.kind === "step") {
    blockers.push(
      ...controllerLocalNavigationBlockers(
        declaration.step,
        {
          currentStepId: context.navigation.summary.currentStepId,
          localStepIds: context.navigation.summary.localStepIds,
          visitedStepIds: context.navigation.summary.visitedStepIds,
          skippedStepIds: context.navigation.summary.skippedStepIds,
          localSteps: context.steps.artifacts.filter((step) => step.routeCoupled !== true),
        },
        declaration.id,
      ),
    );
  }
  blockers.push(
    ...context.readiness.blockers.filter((blocker) => {
      if (blocker.kind === "host:offline" || blocker.kind === "host:unavailable") {
        return false;
      }
      if (blocker.kind === "unchanged" && declaration.patchPolicy !== "requiresNonEmpty") {
        return false;
      }
      return blocker.action === declaration.id || blocker.action === undefined;
    }),
  );
  return Object.freeze(blockers);
}

function currentActionBinding(form, fieldDeclarations, patchPlan) {
  const sourceDigest = stableValueDigest(form.source());
  const draftDigest = stableValueDigest(form.draft());
  const effectiveDigest = stableValueDigest(form.effective());
  const patchDigest = patchPlan.equivalenceDigest;
  const schemaDigest = stableValueDigest(
    fieldDeclarations.map((field) => ({
      id: field.id,
      path: field.path,
      inputAdapterTier: field.inputAdapter.tier,
    })),
  );
  return Object.freeze({
    sourceDigest,
    draftDigest,
    effectiveDigest,
    patchDigest,
    schemaDigest,
    bindingDigest: stableValueDigest({
      sourceDigest,
      draftDigest,
      effectiveDigest,
      patchDigest,
      schemaDigest,
    }),
  });
}

function regulatedActionBindings(declaration, admission, planDigest) {
  return Object.freeze(
    admission.artifacts
      .filter((artifact) => (
        artifact.scope === "action" &&
        artifact.ownerId === declaration.id &&
        artifact.binding !== undefined
      ))
      .map((artifact) => {
        const binding = {
          admissionId: artifact.id,
          capability: artifact.capability,
          posture: artifact.posture,
          actorDigest: artifact.actorDigest,
          policyDigest: artifact.policyDigest,
          admissionBindingDigest: artifact.binding.bindingDigest,
          actionPlanDigest: planDigest,
        };
        return Object.freeze({
          ...binding,
          attestationDigest: stableValueDigest(binding),
        });
      }),
  );
}

function routeSemantics(declaration) {
  if (declaration.kind !== "step") {
    return "notStepNavigation";
  }
  return declaration.step?.routeCoupled === true
    ? "routeCoupledDeferred"
    : "controllerLocalOnly";
}

function actionSummary(plans) {
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

function actionCounters(declarations, plans) {
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    incrementalStatus: "notIncremental",
    declarations: declarations.length,
    plans: plans.length,
    deniedPlans: plans.filter((plan) => plan.status === "denied").length,
    destructivePlans: plans.filter((plan) => plan.destructive).length,
    stepPlans: plans.filter((plan) => plan.kind === "step").length,
    routeCoupledDeferredPlans: plans.filter((plan) => plan.diagnostics.routeSemantics === "routeCoupledDeferred").length,
    hostRequiredPlans: plans.filter((plan) => plan.host.requirements.length > 0).length,
    nonEmptyPatchPlans: plans.filter((plan) => !plan.patch.empty).length,
  });
}

function actionReportDigests(declarations, plans) {
  const catalogDigest = stableValueDigest(declarations.map((declaration) => ({
    id: declaration.id,
    kind: declaration.kind,
    patchPolicy: declaration.patchPolicy,
    admissionCapability: declaration.admissionCapability,
    destructive: declaration.destructive,
    idempotency: declaration.idempotency,
    effectPolicy: declaration.effectPolicy,
    hostEffect: declaration.hostEffect,
    hostRequirements: declaration.hostRequirements,
    schema: declaration.schema,
    step: declaration.step,
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
