import { createFormReadView } from "../read_views.js";
import { FormDeclarationError } from "../form_errors.js";
import { admissionReadinessBlockers } from "../admission/artifacts.js";
import { availabilityReadinessBlockers } from "../availability/artifacts.js";
import { validationReadinessBlockers } from "../validation/artifacts.js";

const STEP_POSTURES = new Set([
  "active",
  "optional",
  "skipped",
  "blocked",
  "removed",
  "unavailable",
]);

export function evaluateSteps(stepDeclarations, form) {
  const validation = form.validation();
  const availability = form.availability();
  const admission = form.admission();
  const dirty = form.dirty();
  const patchPlan = form.patchPlan();
  const messages = form.visibleMessages();
  const readView = createFormReadView(form);
  const artifacts = stepDeclarations.map((declaration) =>
    stepArtifact(declaration, readView, {
      validation,
      availability,
      admission,
      dirty,
      patchPlan,
      messages,
    }),
  );
  return Object.freeze({
    artifacts: Object.freeze(artifacts),
    summary: stepSummary(artifacts),
    counters: stepCounters(stepDeclarations, artifacts),
    dependencyBreadth: Object.freeze(
      stepDeclarations.map((declaration) => ({
        id: declaration.id,
        fields: declaration.fields,
        dependencies: declaration.dependencies,
        routeCoupled: declaration.routeCoupled,
        layout: declaration.layout,
      })),
    ),
  });
}

function stepCounters(declarations, artifacts) {
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    incrementalStatus: "notIncremental",
    declarations: declarations.length,
    routeCoupledDeclarations: declarations.filter((declaration) => declaration.routeCoupled).length,
    stepFieldMemberships: declarations.reduce((total, declaration) => (
      total + declaration.fields.length
    ), 0),
    dependencyReads: declarations.reduce((total, declaration) => (
      total + declaration.dependencies.length
    ), 0),
    readinessBlockers: artifacts.reduce((total, artifact) => (
      total + artifact.readiness.blockers.length
    ), 0),
    projectedPatchOperations: artifacts.reduce((total, artifact) => (
      total + artifact.patch.operations.length + (artifact.patch.replacement === null ? 0 : 1)
    ), 0),
    projectedValidationArtifacts: artifacts.reduce((total, artifact) => (
      total + artifact.validation.artifacts.length
    ), 0),
    uniqueProjectedValidationArtifacts: uniqueProjectionCount(
      artifacts.flatMap((artifact) => artifact.validation.artifacts),
      validationProjectionKey,
    ),
    projectedMessages: artifacts.reduce((total, artifact) => (
      total + artifact.messages.length
    ), 0),
    uniqueProjectedMessages: uniqueProjectionCount(
      artifacts.flatMap((artifact) => artifact.messages),
      messageProjectionKey,
    ),
  });
}

function uniqueProjectionCount(entries, keyForEntry) {
  return new Set(entries.map((entry) => keyForEntry(entry))).size;
}

function validationProjectionKey(artifact) {
  return JSON.stringify({
    kind: artifact.kind,
    field: artifact.field ?? null,
    code: artifact.message?.code ?? null,
    digest: artifact.digest ?? artifact.rawDigest ?? null,
    asyncValidationId: artifact.asyncValidationId ?? null,
    reason: artifact.reason ?? null,
  });
}

function messageProjectionKey(message) {
  return JSON.stringify({
    code: message.code,
    target: message.target ?? null,
    severity: message.severity,
    audience: message.audience,
    visibility: message.visibility,
  });
}

function stepArtifact(declaration, readView, formArtifacts) {
  const posture = declaration.routeCoupled
    ? routeCoupledStepPosture()
    : normalizeStepPosture(runStepResolver(declaration, readView), declaration);
  const fieldSet = new Set(declaration.fields);
  const dirtyFields = formArtifacts.dirty.fields.filter((field) => fieldSet.has(field.field));
  const patchOperations = formArtifacts.patchPlan.operations.filter((operation) =>
    fieldSet.has(operation.field),
  );
  const patchReplacement = formArtifacts.patchPlan.replacement !== null
    && formArtifacts.patchPlan.replacement.fields.some((field) => fieldSet.has(field))
    ? formArtifacts.patchPlan.replacement
    : null;
  const validationArtifacts = formArtifacts.validation.artifacts.filter((artifact) =>
    artifact.field === undefined || fieldSet.has(artifact.field),
  );
  const messages = formArtifacts.messages.filter((message) =>
    message.target === undefined || fieldSet.has(message.target),
  );
  const blockers = stepReadinessBlockers(fieldSet, posture, formArtifacts);
  const canEnter = posture.posture === "active" || posture.posture === "optional";
  const canComplete = canEnter && blockers.length === 0;
  return Object.freeze({
    kind: "step",
    id: declaration.id,
    group: declaration.group,
    order: declaration.order,
    orderDeclared: declaration.orderDeclared,
    fields: declaration.fields,
    routeCoupled: declaration.routeCoupled,
    layout: declaration.layout,
    posture: posture.posture,
    ...(posture.reason === undefined ? {} : { reason: posture.reason }),
    readiness: Object.freeze({
      canEnter,
      canComplete,
      blockers: Object.freeze(blockers),
    }),
    dirty: Object.freeze({
      isDirty: dirtyFields.length > 0,
      fields: Object.freeze(dirtyFields),
    }),
    patch: Object.freeze({
      empty: patchOperations.length === 0 && patchReplacement === null,
      operations: Object.freeze(patchOperations),
      replacement: patchReplacement,
    }),
    validation: Object.freeze({
      artifacts: Object.freeze(validationArtifacts),
      blockers: Object.freeze(
        validationReadinessBlockers(formArtifacts.validation).filter((blocker) =>
          blocker.field === undefined || fieldSet.has(blocker.field),
        ),
      ),
    }),
    messages: Object.freeze(messages),
    progress: stepProgress(posture.posture, blockers, dirtyFields),
  });
}

function runStepResolver(declaration, readView) {
  if (declaration.resolve === null) {
    return declaration.defaultPosture;
  }
  const values = Object.fromEntries(
    declaration.dependencies.map((fieldId) => [
      fieldId,
      readView.field(fieldId).effectiveValue(),
    ]),
  );
  return declaration.resolve(values, {
    form: readView,
    step: declaration.id,
    fields: declaration.fields,
    dependencies: declaration.dependencies,
  });
}

function normalizeStepPosture(posture, declaration) {
  if (posture == null || posture === true) {
    return Object.freeze({ posture: declaration.defaultPosture });
  }
  if (typeof posture === "string") {
    return stepPostureArtifact(posture);
  }
  if (!posture || typeof posture !== "object") {
    throw new FormDeclarationError("step resolver returned an undeclared posture shape", {
      stepId: declaration.id,
      posture,
    });
  }
  return stepPostureArtifact(posture.posture ?? declaration.defaultPosture, posture.reason);
}

function stepPostureArtifact(posture, reason) {
  if (!STEP_POSTURES.has(posture)) {
    throw new FormDeclarationError("step posture is not supported", { posture });
  }
  return Object.freeze({
    posture,
    ...(reason === undefined ? {} : { reason: String(reason) }),
  });
}

function stepReadinessBlockers(fieldSet, posture, formArtifacts) {
  const blockers = [];
  if (posture.posture === "blocked" || posture.posture === "unavailable") {
    blockers.push({
      kind: posture.routeCoupled === true ? "step:deferred" : `step:${posture.posture}`,
      reason: posture.reason ?? `step is ${posture.posture}`,
    });
  }
  blockers.push(
    ...validationReadinessBlockers(formArtifacts.validation).filter((blocker) =>
      blocker.field === undefined || fieldSet.has(blocker.field),
    ),
  );
  blockers.push(
    ...availabilityReadinessBlockers(formArtifacts.availability).filter((blocker) =>
      blockerAppliesToStepFields(blocker, fieldSet),
    ),
  );
  blockers.push(
    ...admissionReadinessBlockers(formArtifacts.admission).filter((blocker) =>
      blocker.field === undefined || fieldSet.has(blocker.field),
    ),
  );
  return blockers;
}

function blockerAppliesToStepFields(blocker, fieldSet) {
  if (blocker.field !== undefined) {
    return fieldSet.has(blocker.field);
  }
  if (Array.isArray(blocker.fields) && blocker.fields.length > 0) {
    return blocker.fields.some((field) => fieldSet.has(field));
  }
  return true;
}

function stepProgress(posture, blockers, dirtyFields) {
  if (posture === "removed" || posture === "skipped") {
    return posture;
  }
  if (blockers.length > 0) {
    return "blocked";
  }
  if (dirtyFields.length > 0) {
    return "changed";
  }
  return "complete";
}

function routeCoupledStepPosture() {
  return Object.freeze({
    posture: "unavailable",
    reason: "route-coupled step behavior requires route authority outside controller-local navigation",
    routeCoupled: true,
  });
}

function stepSummary(artifacts) {
  const summary = {
    total: artifacts.length,
    active: 0,
    optional: 0,
    skipped: 0,
    blocked: 0,
    removed: 0,
    unavailable: 0,
    complete: 0,
    changed: 0,
  };
  for (const artifact of artifacts) {
    summary[artifact.posture] += 1;
    if (artifact.progress === "complete") {
      summary.complete += 1;
    }
    if (artifact.progress === "changed") {
      summary.changed += 1;
    }
  }
  return Object.freeze(summary);
}
