import { FormDeclarationError } from "../form_errors.js";
import { createFormReadView } from "../read_views.js";
import { stableValueDigest } from "../values/value_paths.js";
import { normalizeValidationArtifact } from "./artifacts.js";

export function createAsyncValidationStore(validationDeclarations, fieldDeclarations) {
  let nextOperationId = 1;
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const declarationsById = new Map(
    validationDeclarations
      .filter((declaration) => declaration.kind === "async")
      .map((declaration) => [declaration.id, declaration]),
  );
  const activeOperations = new Map();
  const activeOperationByValidationId = new Map();
  const currentArtifacts = new Map();
  const history = [];

  return Object.freeze({
    start(validationId, form) {
      const declaration = requireAsyncValidationDeclaration(declarationsById, validationId);
      const basisDigest = asyncValidationBasisDigest(declaration, form);
      const operation = Object.freeze({
        operationId: nextOperationId++,
        validationId,
        field: declaration.field,
        dependencies: declaration.dependencies,
        triggerPolicy: declaration.triggerPolicy,
        basisDigest,
      });
      supersedeActiveValidationOperation(operation);
      activeOperations.set(operation.operationId, operation);
      activeOperationByValidationId.set(validationId, operation);
      currentArtifacts.set(validationId, pendingValidationArtifact(operation));
      return recordAsyncValidationLifecycle(operation, "pending", {
        reason: "async validation is pending external settlement",
      });
    },
    fulfill(operationId, payload = {}, form) {
      return settleAsyncValidation(operationId, "fulfilled", payload, form);
    },
    reject(operationId, payload = {}, form) {
      return settleAsyncValidation(operationId, "rejected", payload, form);
    },
    cancel(operationId, payload = {}) {
      return terminateAsyncValidation(operationId, "cancelled", payload.reason ?? "async validation cancelled");
    },
    timeout(operationId, payload = {}) {
      return terminateAsyncValidation(operationId, "timedOut", payload.reason ?? "async validation timed out");
    },
    artifacts() {
      return Object.freeze([...currentArtifacts.values()]);
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function settleAsyncValidation(operationId, resultKind, payload, form) {
    const operation = activeOperations.get(operationId);
    if (!operation) {
      return recordStaleAsyncValidation(operationId, "async validation completion arrived after terminal settlement");
    }
    if (asyncValidationBasisDigest(operation, form) !== operation.basisDigest) {
      removeActiveOperation(operation);
      currentArtifacts.set(operation.validationId, staleBlockedValidationArtifact(operation));
      return recordStaleAsyncValidation(operationId, "async validation completion targeted a superseded form truth snapshot", operation);
    }
    removeActiveOperation(operation);
    currentArtifacts.set(operation.validationId, settledValidationArtifact(operation, resultKind, payload));
    return recordAsyncValidationLifecycle(operation, resultKind, {
      reason: payload.reason ?? `async validation ${resultKind}`,
    });
  }

  function terminateAsyncValidation(operationId, resultKind, reason) {
    const operation = activeOperations.get(operationId);
    if (!operation) {
      return recordStaleAsyncValidation(operationId, `async validation ${resultKind} targeted a settled operation`);
    }
    removeActiveOperation(operation);
    currentArtifacts.set(operation.validationId, staleBlockedValidationArtifact(operation, reason));
    return recordAsyncValidationLifecycle(operation, resultKind, { reason });
  }

  function supersedeActiveValidationOperation(nextOperation) {
    const previousOperation = activeOperationByValidationId.get(nextOperation.validationId);
    if (!previousOperation) {
      return;
    }
    removeActiveOperation(previousOperation);
    history.push(asyncValidationLifecycleArtifact({
      operationId: previousOperation.operationId,
      validationId: previousOperation.validationId,
      supersededByOperationId: nextOperation.operationId,
      field: previousOperation.field,
      dependencies: previousOperation.dependencies,
      triggerPolicy: previousOperation.triggerPolicy,
      basisDigest: previousOperation.basisDigest,
      resultKind: "superseded",
      stale: false,
      reason: "async validation operation was superseded by a newer run",
    }));
  }

  function removeActiveOperation(operation) {
    activeOperations.delete(operation.operationId);
    const activeForValidation = activeOperationByValidationId.get(operation.validationId);
    if (activeForValidation?.operationId === operation.operationId) {
      activeOperationByValidationId.delete(operation.validationId);
    }
  }

  function recordStaleAsyncValidation(targetOperationId, reason, targetOperation = null) {
    const artifact = asyncValidationLifecycleArtifact({
      operationId: nextOperationId++,
      targetOperationId,
      targetValidationId: targetOperation?.validationId ?? null,
      targetBasisDigest: targetOperation?.basisDigest ?? null,
      resultKind: "staleCompletion",
      stale: true,
      reason,
    });
    history.push(artifact);
    return artifact;
  }

  function recordAsyncValidationLifecycle(operation, resultKind, options) {
    const artifact = asyncValidationLifecycleArtifact({
      operationId: operation.operationId,
      validationId: operation.validationId,
      field: operation.field,
      dependencies: operation.dependencies,
      triggerPolicy: operation.triggerPolicy,
      basisDigest: operation.basisDigest,
      resultKind,
      stale: false,
      reason: options.reason,
    });
    history.push(artifact);
    return artifact;
  }

  function settledValidationArtifact(operation, resultKind, payload) {
    if (payload.artifact !== undefined) {
      return normalizeValidationArtifact(payload.artifact, operation, declaredFieldIds);
    }
    if (resultKind === "rejected") {
      return normalizeValidationArtifact({
        kind: "invalid",
        field: operation.field,
        message: {
          code: payload.code ?? "form.async.validation.rejected",
          message: payload.reason ?? "Async validation rejected the field",
          severity: "error",
          target: operation.field,
          audience: "user",
          visibility: "visible",
        },
      }, operation, declaredFieldIds);
    }
    return normalizeValidationArtifact({
      kind: "valid",
      field: operation.field,
      digest: operation.basisDigest,
    }, operation, declaredFieldIds);
  }
}

function requireAsyncValidationDeclaration(declarationsById, validationId) {
  const declaration = declarationsById.get(validationId);
  if (!declaration) {
    throw new FormDeclarationError("async validation is not declared", { validationId });
  }
  return declaration;
}

function asyncValidationBasisDigest(declaration, form) {
  const readView = createFormReadView(form);
  return stableValueDigest(
    Object.fromEntries(
      declaration.dependencies.map((fieldId) => [
        fieldId,
        readView.field(fieldId).effectiveValue(),
      ]),
    ),
  );
}

function pendingValidationArtifact(operation) {
  return Object.freeze({
    kind: "pending",
    field: operation.field,
    asyncValidationId: operation.validationId,
    operationId: operation.operationId,
    basisDigest: operation.basisDigest,
  });
}

function staleBlockedValidationArtifact(operation, reason = "async validation requires a fresh run") {
  return Object.freeze({
    kind: "blocked",
    field: operation.field,
    reason,
    blockers: Object.freeze([`asyncValidation:${operation.validationId}:stale`]),
  });
}

function asyncValidationLifecycleArtifact(options) {
  const artifact = {
    kind: "asyncValidation",
    operationId: options.operationId,
    targetOperationId: options.targetOperationId,
    supersededByOperationId: options.supersededByOperationId,
    validationId: options.validationId ?? null,
    targetValidationId: options.targetValidationId,
    field: options.field ?? null,
    dependencies: Object.freeze(options.dependencies ?? []),
    triggerPolicy: options.triggerPolicy ?? null,
    basisDigest: options.basisDigest ?? null,
    targetBasisDigest: options.targetBasisDigest,
    resultKind: options.resultKind,
    stale: options.stale,
    reason: options.reason,
  };
  return Object.freeze({
    ...artifact,
    lifecycleDigest: stableValueDigest(artifact),
  });
}
