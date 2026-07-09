import { FormDeclarationError } from "../form_errors.js";
import { requireResourceEffectProfile } from "../../resource/effects/resource_effect_profile.js";

const ACTION_DECLARATION_BRAND = Symbol("WORTH.form.actionDeclaration");
const ACTION_KINDS = new Set(["submit", "custom", "step"]);
const PATCH_POLICIES = new Set(["requiresNonEmpty", "allowEmpty", "ignore"]);
const IDEMPOTENCY_POLICIES = new Set(["none", "collapse", "supersede", "queue", "deny"]);
const EFFECT_POLICIES = new Set(["deferred", "none", "controllerLocal"]);
const STEP_COMMANDS = new Set(["next", "back", "jump", "skip", "revisit", "custom"]);
const HOST_REQUIREMENTS = new Set(["online", "persistence", "credentials", "autofill"]);

export function materializeActionDeclarations(declaration, stepDeclarations, fieldDeclarations) {
  const declaredStepIds = new Set(stepDeclarations.map((step) => step.id));
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const factory = createActionFactory(declaredStepIds, declaredFieldIds);
  const declared = declaration.actions === undefined
    ? {}
    : typeof declaration.actions === "function"
      ? declaration.actions({
          submit: factory.submit,
          action: factory.action,
          step: factory.step,
        })
      : declaration.actions;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form actions must be declared as an object");
  }
  const declarations = Object.entries(declared).map(([name, action]) =>
    normalizeActionDeclaration(name, action),
  );
  if (!declarations.some((action) => action.id === "submit")) {
    declarations.unshift(normalizeActionDeclaration("submit", factory.submit()));
  }
  denyDuplicateActionIds(declarations);
  return Object.freeze(declarations);
}

function createActionFactory(declaredStepIds, declaredFieldIds) {
  return {
    submit(options = {}) {
      return actionDeclaration("submit", {
        ...options,
        kind: "submit",
        patchPolicy: options.patchPolicy ?? "requiresNonEmpty",
        admissionCapability: options.admissionCapability ?? "submit",
        effectPolicy: options.effectPolicy ?? "deferred",
      }, declaredFieldIds);
    },
    action(actionId, options = {}) {
      requireActionId(actionId);
      requireRouteCoupledOption(actionId, options.routeCoupled);
      if (options.kind === "submit" || options.kind === "step") {
        throw new FormDeclarationError("custom actions cannot impersonate built-in action kinds", {
          actionId,
          kind: options.kind,
        });
      }
      if (options.routeCoupled === true) {
        throw new FormDeclarationError("only step actions may declare route-coupled posture", {
          actionId,
        });
      }
      return actionDeclaration(actionId, {
        ...options,
        kind: options.kind ?? "custom",
        patchPolicy: options.patchPolicy ?? defaultPatchPolicyForAction(options.resourceAction),
        admissionCapability: options.admissionCapability ?? "action",
        effectPolicy: options.effectPolicy ?? "deferred",
      }, declaredFieldIds);
    },
    step(actionId, stepId, command, options = {}) {
      requireActionId(actionId);
      requireStepId(declaredStepIds, stepId);
      requireStepCommand(command);
      requireRouteCoupledOption(actionId, options.routeCoupled);
      return actionDeclaration(actionId, {
        ...options,
        kind: "step",
        patchPolicy: options.patchPolicy ?? "allowEmpty",
        admissionCapability: options.admissionCapability ?? "action",
        effectPolicy: options.effectPolicy ?? "controllerLocal",
        step: Object.freeze({
          stepId,
          command,
          routeCoupled: options.routeCoupled === true,
        }),
      }, declaredFieldIds);
    },
  };
}

function actionDeclaration(actionId, options, declaredFieldIds = new Set()) {
  const kind = normalizeEnum(options.kind, ACTION_KINDS, "action kind");
  const patchPolicy = normalizeEnum(options.patchPolicy, PATCH_POLICIES, "action patch policy");
  const idempotency = normalizeEnum(
    options.idempotency ?? "none",
    IDEMPOTENCY_POLICIES,
    "action idempotency policy",
  );
  const effectPolicy = normalizeEnum(
    options.effectPolicy ?? "deferred",
    EFFECT_POLICIES,
    "action effect policy",
  );
  const resourceAction = normalizeResourceAction(actionId, kind, options, effectPolicy, declaredFieldIds);
  if (kind !== "step" && options.step !== undefined) {
    throw new FormDeclarationError("only step actions may declare step navigation metadata", {
      actionId,
    });
  }
  return Object.freeze({
    [ACTION_DECLARATION_BRAND]: true,
    id: actionId,
    kind,
    label: options.label === undefined ? actionId : String(options.label),
    patchPolicy,
    admissionCapability: String(options.admissionCapability ?? "action"),
    destructive: options.destructive === true,
    idempotency,
    effectPolicy,
    hostEffect: options.hostEffect === undefined ? null : String(options.hostEffect),
    hostRequirements: normalizeHostRequirements(options.hostRequirements),
    resourceAction,
    resourceEffectProfile: options.resourceEffectProfile === undefined
      ? null
      : requireResourceEffectProfile(
        options.resourceEffectProfile,
        `form action "${actionId}" resourceEffectProfile`,
      ),
    schema: options.schema === undefined ? null : options.schema,
    step: options.step ?? null,
  });
}

function normalizeActionDeclaration(name, action) {
  if (!action || action[ACTION_DECLARATION_BRAND] !== true) {
    throw new FormDeclarationError("action entries must be declared with actions.submit/action/step", {
      name,
    });
  }
  return Object.freeze({
    ...action,
    name,
  });
}

function denyDuplicateActionIds(declarations) {
  const seen = new Set();
  for (const declaration of declarations) {
    if (seen.has(declaration.id)) {
      throw new FormDeclarationError("action declaration ids must be unique", {
        id: declaration.id,
      });
    }
    seen.add(declaration.id);
  }
}

function requireActionId(actionId) {
  if (typeof actionId !== "string" || actionId.length === 0) {
    throw new FormDeclarationError("action id must be a non-empty string");
  }
}

function requireStepId(declaredStepIds, stepId) {
  if (!declaredStepIds.has(stepId)) {
    throw new FormDeclarationError("step action references an undeclared step", { stepId });
  }
}

function requireStepCommand(command) {
  normalizeEnum(command, STEP_COMMANDS, "step action command");
}

function requireRouteCoupledOption(actionId, routeCoupled) {
  if (routeCoupled !== undefined && typeof routeCoupled !== "boolean") {
    throw new FormDeclarationError("action routeCoupled posture must be a boolean", {
      actionId,
      routeCoupled,
    });
  }
}

function normalizeHostRequirements(requirements) {
  if (requirements === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(requirements)) {
    throw new FormDeclarationError("action host requirements must be an array", {
      hostRequirements: requirements,
    });
  }
  const seen = new Set();
  for (const requirement of requirements) {
    if (typeof requirement !== "string" || !HOST_REQUIREMENTS.has(requirement)) {
      throw new FormDeclarationError("action host requirement is not supported", {
        requirement,
      });
    }
    if (seen.has(requirement)) {
      throw new FormDeclarationError("action host requirements must be unique", {
        requirement,
      });
    }
    seen.add(requirement);
  }
  return Object.freeze([...requirements]);
}

function normalizeResourceAction(actionId, kind, options, effectPolicy, declaredFieldIds) {
  if (options.resourceAction === undefined) {
    return null;
  }
  if (!options.resourceAction || typeof options.resourceAction !== "object" || Array.isArray(options.resourceAction)) {
    throw new FormDeclarationError("resource-line action declaration must be an object", {
      actionId,
      resourceAction: options.resourceAction,
    });
  }
  if (kind === "step") {
    throw new FormDeclarationError("step actions cannot declare resource-line execution", {
      actionId,
    });
  }
  if (options.hostEffect !== undefined) {
    throw new FormDeclarationError("resource-line actions cannot also declare hostEffect", {
      actionId,
    });
  }
  if (effectPolicy !== "deferred") {
    throw new FormDeclarationError("resource-line actions require deferred effect policy", {
      actionId,
      effectPolicy,
    });
  }
  if (options.resourceAction.kind === "patchPlan") {
    if (options.patchPolicy !== undefined && options.patchPolicy !== "requiresNonEmpty") {
      throw new FormDeclarationError("resource-line patch actions require requiresNonEmpty patch policy", {
        actionId,
        patchPolicy: options.patchPolicy,
      });
    }
    return Object.freeze({
      kind: "patchPlan",
      fields: normalizeScopedPatchFields(actionId, options.resourceAction.fields, declaredFieldIds),
    });
  }
  if (options.resourceAction.kind === "refresh" || options.resourceAction.kind === "revalidate") {
    if (options.patchPolicy !== undefined && options.patchPolicy !== "ignore") {
      throw new FormDeclarationError("resource-line lifecycle actions require ignore patch policy", {
        actionId,
        patchPolicy: options.patchPolicy,
      });
    }
    if (options.resourceEffectProfile !== undefined) {
      throw new FormDeclarationError("resource-line lifecycle actions cannot declare resourceEffectProfile", {
        actionId,
      });
    }
    return Object.freeze({
      kind: options.resourceAction.kind,
    });
  }
  if (
    options.resourceAction.kind === "replayExact"
    || options.resourceAction.kind === "restoreExact"
    || options.resourceAction.kind === "rollbackLastEffect"
  ) {
    if (options.patchPolicy !== undefined && options.patchPolicy !== "ignore") {
      throw new FormDeclarationError("resource-line recovery actions require ignore patch policy", {
        actionId,
        patchPolicy: options.patchPolicy,
      });
    }
    if (options.resourceEffectProfile !== undefined) {
      throw new FormDeclarationError("resource-line recovery actions cannot declare resourceEffectProfile", {
        actionId,
      });
    }
    return Object.freeze({
      kind: options.resourceAction.kind,
    });
  }
  {
    throw new FormDeclarationError("resource-line action kind is not supported", {
      actionId,
      kind: options.resourceAction.kind,
    });
  }
}

function normalizeEnum(value, allowed, label) {
  if (typeof value !== "string" || !allowed.has(value)) {
    throw new FormDeclarationError(`${label} is not supported`, { value });
  }
  return value;
}

function normalizeScopedPatchFields(actionId, fields, declaredFieldIds) {
  if (fields === undefined) {
    return null;
  }
  if (!Array.isArray(fields) || fields.length === 0) {
    throw new FormDeclarationError("resource-line scoped patch actions require a non-empty fields array", {
      actionId,
      fields,
    });
  }
  const seen = new Set();
  for (const fieldId of fields) {
    if (typeof fieldId !== "string" || fieldId.length === 0) {
      throw new FormDeclarationError("resource-line scoped patch field ids must be non-empty strings", {
        actionId,
        fieldId,
      });
    }
    if (seen.has(fieldId)) {
      throw new FormDeclarationError("resource-line scoped patch field ids must be unique", {
        actionId,
        fieldId,
      });
    }
    if (!declaredFieldIds.has(fieldId)) {
      throw new FormDeclarationError("resource-line scoped patch actions cannot reference undeclared fields", {
        actionId,
        fieldId,
      });
    }
    seen.add(fieldId);
  }
  return Object.freeze([...fields]);
}

function defaultPatchPolicyForAction(resourceAction) {
  if (resourceAction === undefined) {
    return "allowEmpty";
  }
  return resourceAction.kind === "patchPlan" ? "requiresNonEmpty" : "ignore";
}
