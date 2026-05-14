import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";

const REQUIRED_ACTION_HOST_CAPABILITIES = new Set([
  "online",
  "persistence",
  "credentials",
  "autofill",
]);

export function readHostReport(bindings) {
  const facts = Object.freeze({
    focus: readFocusFact(bindings.focus),
    visibility: readVisibilityFact(bindings.visibility),
    viewport: readViewportFact(bindings.viewport),
    online: readOnlineFact(bindings.online),
    persistence: readAvailabilityFact(bindings.persistence, "persistence"),
    credentials: readAvailabilityFact(bindings.credentials, "credentials"),
    autofill: readAvailabilityFact(bindings.autofill, "autofill"),
  });
  const counters = hostCounters(facts);
  return Object.freeze({
    facts,
    summary: Object.freeze({
      supported: counters.supportedFacts,
      unavailable: counters.unavailableFacts,
    }),
    counters,
    digest: stableValueDigest({
      facts,
      counters,
    }),
  });
}

export function hostRequirementBlockers(report, requirements, actionId = undefined) {
  const blockers = [];
  for (const capability of requirements ?? []) {
    if (!REQUIRED_ACTION_HOST_CAPABILITIES.has(capability)) {
      throw new FormDeclarationError("action host requirement is not supported", {
        capability,
      });
    }
    const fact = report.facts[capability];
    if (fact.posture !== "supported") {
      blockers.push(Object.freeze({
        kind: "host:unavailable",
        ...(actionId === undefined ? {} : { action: actionId }),
        capability,
        reason: `${capability} host fact is unavailable: ${fact.reason}`,
      }));
      continue;
    }
    if (capability === "online" && fact.state === "offline") {
      blockers.push(Object.freeze({
        kind: "host:offline",
        ...(actionId === undefined ? {} : { action: actionId }),
        capability,
        reason: "action requires online host connectivity",
      }));
      continue;
    }
    if (capability !== "online" && fact.available === false) {
      blockers.push(Object.freeze({
        kind: "host:unavailable",
        ...(actionId === undefined ? {} : { action: actionId }),
        capability,
        reason: `${capability} host capability is currently unavailable`,
      }));
    }
  }
  return Object.freeze(blockers);
}

function readFocusFact(binding) {
  if (binding == null) {
    return unavailableFact("focus", "host focus fact was not declared");
  }
  const focusedField = readBoundValue(binding, "focus");
  if (focusedField !== null && typeof focusedField !== "string") {
    throw new FormDeclarationError("form host focus binding must resolve to a string or null", {
      focusedField,
    });
  }
  return supportedFact("focus", {
    focusedField,
  });
}

function readVisibilityFact(binding) {
  if (binding == null) {
    return unavailableFact("visibility", "host visibility fact was not declared");
  }
  const state = isHostCapabilityHandle(binding, "visibility")
    ? binding.state()
    : readBoundValue(binding, "visibility");
  if (state !== "visible" && state !== "hidden") {
    throw new FormDeclarationError("form host visibility binding must resolve to visible or hidden", {
      visibility: state,
    });
  }
  return supportedFact("visibility", {
    state,
  });
}

function readViewportFact(binding) {
  if (binding == null) {
    return unavailableFact("viewport", "host viewport fact was not declared");
  }
  const rawViewport = isHostCapabilityHandle(binding, "viewport")
    ? binding.size()
    : readBoundValue(binding, "viewport");
  if (!rawViewport || typeof rawViewport !== "object") {
    throw new FormDeclarationError("form host viewport binding must resolve to an object", {
      viewport: rawViewport,
    });
  }
  const width = normalizeFiniteNumber(rawViewport.width, "viewport width");
  const height = normalizeFiniteNumber(rawViewport.height, "viewport height");
  return supportedFact("viewport", {
    size: Object.freeze({ width, height }),
  });
}

function readOnlineFact(binding) {
  if (binding == null) {
    return unavailableFact("online", "host online fact was not declared");
  }
  const state = isHostCapabilityHandle(binding, "online")
    ? binding.state()
    : normalizeOnlineState(readBoundValue(binding, "online"));
  return supportedFact("online", {
    state,
  });
}

function readAvailabilityFact(binding, fact) {
  if (binding == null) {
    return unavailableFact(fact, `${fact} host fact was not declared`);
  }
  const available = fact === "persistence" && isHostCapabilityHandle(binding, "persistence")
    ? true
    : readBooleanAvailability(binding, fact);
  const availabilityDigest = fact === "persistence" && isHostCapabilityHandle(binding, "persistence")
    ? stableValueDigest({
        available,
        persistedValue: cloneFormValue(binding.value()),
      })
    : stableValueDigest({ available });
  return Object.freeze({
    fact,
    declared: true,
    posture: "supported",
    available,
    reason: null,
    digest: availabilityDigest,
  });
}

function readBooleanAvailability(binding, fact) {
  const value = readBoundValue(binding, fact);
  if (typeof value !== "boolean") {
    throw new FormDeclarationError(`form host ${fact} binding must resolve to a boolean`, {
      [fact]: value,
    });
  }
  return value;
}

function supportedFact(fact, details) {
  return Object.freeze({
    fact,
    declared: true,
    posture: "supported",
    ...details,
    reason: null,
    digest: stableValueDigest({
      fact,
      ...details,
    }),
  });
}

function unavailableFact(fact, reason) {
  return Object.freeze({
    fact,
    declared: false,
    posture: "unavailable",
    reason,
    digest: stableValueDigest({
      fact,
      reason,
    }),
  });
}

function hostCounters(facts) {
  const entries = Object.values(facts);
  return Object.freeze({
    costBasis: "hostFactDerivedRead",
    incrementalStatus: "notIncremental",
    declaredFacts: entries.filter((fact) => fact.declared).length,
    supportedFacts: entries.filter((fact) => fact.posture === "supported").length,
    unavailableFacts: entries.filter((fact) => fact.posture === "unavailable").length,
    hostHandleFacts: entries.filter((fact) => (
      fact.fact === "visibility" ||
      fact.fact === "viewport" ||
      fact.fact === "online" ||
      fact.fact === "persistence"
    ) && fact.declared).length,
  });
}

function readBoundValue(binding, fact) {
  if (typeof binding === "function") {
    return binding();
  }
  if (binding && typeof binding.get === "function") {
    return binding.get();
  }
  return binding;
}

function normalizeOnlineState(value) {
  if (typeof value === "boolean") {
    return value ? "online" : "offline";
  }
  if (value === "online" || value === "offline") {
    return value;
  }
  throw new FormDeclarationError("form host online binding must resolve to a boolean or online/offline token", {
    online: value,
  });
}

function normalizeFiniteNumber(value, label) {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new FormDeclarationError(`form host ${label} must be a finite number`, {
      value,
    });
  }
  return value;
}

function isHostCapabilityHandle(binding, family) {
  return binding && typeof binding === "object" &&
    typeof binding.descriptor === "function" &&
    binding.descriptor()?.family === family;
}
