import { readPath, stableValueDigest } from "../values/value_paths.js";
import { createFormReadView } from "../read_views.js";
import { normalizeValidationArtifact } from "./artifacts.js";

export function validateForm(fieldDeclarations, validationDeclarations, form, parseFailures, asyncValidationArtifacts = []) {
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const artifacts = [...parseFailures.values()].map((artifact) => normalizeValidationArtifact(artifact));
  artifacts.push(...asyncValidationArtifacts);
  const counters = validationCounterSeed();
  const validationReadView = createFormReadView(form);
  for (const declaration of syncValidationDeclarations(validationDeclarations)) {
    pushValidatorArtifacts(artifacts, declaration, validationReadView, declaredFieldIds);
    incrementValidationCounter(counters, declaration, fieldDeclarations);
  }
  pushImplicitValidArtifacts(artifacts, fieldDeclarations, validationDeclarations, form);
  return Object.freeze({
    artifacts: Object.freeze(artifacts),
    summary: validationSummary(artifacts),
    counters: Object.freeze(counters),
    dependencyBreadth: validationDependencyBreadth(validationDeclarations),
  });
}

function syncValidationDeclarations(validationDeclarations) {
  return validationDeclarations.filter((declaration) => declaration.kind !== "async");
}

function pushValidatorArtifacts(artifacts, declaration, validationReadView, declaredFieldIds) {
  const result = runValidator(declaration, validationReadView);
  const results = Array.isArray(result) ? result : [result];
  for (const entry of results) {
    artifacts.push(normalizeValidationArtifact(entry, declaration, declaredFieldIds));
  }
}

function runValidator(declaration, validationReadView) {
  if (declaration.breadth === "field") {
    const field = validationReadView.field(declaration.field);
    return declaration.validator(field.effectiveValue(), {
      field,
      form: validationReadView,
      sourceValue: field.sourceValue(),
    });
  }
  const values = Object.fromEntries(
    declaration.dependencies.map((fieldId) => {
      const field = validationReadView.field(fieldId);
      return [fieldId, field.effectiveValue()];
    }),
  );
  return declaration.validator(values, {
    form: validationReadView,
    dependencies: declaration.dependencies,
  });
}

function pushImplicitValidArtifacts(artifacts, fieldDeclarations, validationDeclarations, form) {
  for (const field of fieldDeclarations) {
    if (
      !artifacts.some((artifact) => artifact.field === field.id)
      && !validationDeclarations.some((declaration) => declaration.field === field.id)
    ) {
      artifacts.push(validArtifact(field.id, readPath(form.effective(), field.segments)));
    }
  }
}

function validationCounterSeed() {
  return {
    fieldLocal: 0,
    dependencyRegion: 0,
    wholeForm: 0,
  };
}

function incrementValidationCounter(counters, declaration, fieldDeclarations) {
  if (declaration.breadth === "field") {
    counters.fieldLocal += 1;
  } else if (declaration.dependencies.length === fieldDeclarations.length) {
    counters.wholeForm += 1;
  } else {
    counters.dependencyRegion += 1;
  }
}

function validationDependencyBreadth(validationDeclarations) {
  return Object.freeze(
    validationDeclarations.map((declaration) => ({
      id: declaration.id,
      ...(declaration.kind === "async" ? { kind: "async" } : {}),
      breadth: declaration.breadth,
      dependencies: declaration.dependencies,
      ...(declaration.triggerPolicy === undefined ? {} : { triggerPolicy: declaration.triggerPolicy }),
    })),
  );
}

function validationSummary(artifacts) {
  const summary = {
    valid: 0,
    warning: 0,
    invalid: 0,
    pending: 0,
    blocked: 0,
    unavailable: 0,
    parseFailure: 0,
  };
  for (const artifact of artifacts) {
    summary[artifact.kind] += 1;
  }
  return Object.freeze(summary);
}

function validArtifact(field, value) {
  return Object.freeze({
    kind: "valid",
    ...(field === undefined || field === null ? {} : { field }),
    digest: typeof value === "string" ? value : stableValueDigest(value),
  });
}
