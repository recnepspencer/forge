import { FormDeclarationError } from "../form_errors.js";

const ACTION_DECLARATION_BRAND = Symbol("forge.form.actionDeclaration");
const ACTION_KINDS = new Set(["submit", "custom", "step"]);
const PATCH_POLICIES = new Set(["requiresNonEmpty", "allowEmpty", "ignore"]);
const IDEMPOTENCY_POLICIES = new Set(["none", "collapse", "supersede", "queue", "deny"]);
const EFFECT_POLICIES = new Set(["deferred", "none", "controllerLocal"]);
const STEP_COMMANDS = new Set(["next", "back", "jump", "skip", "revisit", "custom"]);
const HOST_REQUIREMENTS = new Set(["online", "persistence", "credentials", "autofill"]);

export function materializeActionDeclarations(declaration, stepDeclarations) {
  const declaredStepIds = new Set(stepDeclarations.map((step) => step.id));
  const factory = createActionFactory(declaredStepIds);
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

function createActionFactory(declaredStepIds) {
  return {
    submit(options = {}) {
      return actionDeclaration("submit", {
        ...options,
        kind: "submit",
        patchPolicy: options.patchPolicy ?? "requiresNonEmpty",
        admissionCapability: options.admissionCapability ?? "submit",
        effectPolicy: options.effectPolicy ?? "deferred",
      });
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
        patchPolicy: options.patchPolicy ?? "allowEmpty",
        admissionCapability: options.admissionCapability ?? "action",
        effectPolicy: options.effectPolicy ?? "deferred",
      });
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
      });
    },
  };
}

function actionDeclaration(actionId, options) {
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

function normalizeEnum(value, allowed, label) {
  if (typeof value !== "string" || !allowed.has(value)) {
    throw new FormDeclarationError(`${label} is not supported`, { value });
  }
  return value;
}
