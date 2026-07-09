import { requireResourcePatch } from "../reconciliation/resource_patch.js";
import { requireExternalResourceDelivery } from "../compatibility/resource_external_delivery.js";

const RESOURCE_DELIVERY_BRAND = Symbol("WORTHSignal.resourceDelivery");

const resourceDelivery = Object.freeze({
  replace(options) {
    const normalized = requireDeliveryOptions(
      options,
      "resourceDelivery.replace(...)",
    );
    return Object.freeze({
      kind: "replace",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      nextValue: options.nextValue,
      [RESOURCE_DELIVERY_BRAND]: "resourceDelivery",
    });
  },
  patch(options) {
    const normalized = requireDeliveryOptions(
      options,
      "resourceDelivery.patch(...)",
    );
    return Object.freeze({
      kind: "patch",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      patch: options.patch,
      [RESOURCE_DELIVERY_BRAND]: "resourceDelivery",
    });
  },
  invalidate(options) {
    const normalized = requireDeliveryOptions(
      options,
      "resourceDelivery.invalidate(...)",
    );
    return Object.freeze({
      kind: "invalidate",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      [RESOURCE_DELIVERY_BRAND]: "resourceDelivery",
    });
  },
});

function requireResourceDelivery(value, familyKind) {
  if (value && value[RESOURCE_DELIVERY_BRAND] === "resourceDelivery") {
    if (value.kind === "patch") {
      requireResourcePatch(value.patch, familyKind);
    }
    return value;
  }
  if (value && typeof value === "object" && value.kind === "basisRefresh") {
    return requireExternalResourceDelivery(value, familyKind);
  }
  if (
    value &&
    typeof value === "object" &&
    (value.kind === "replace"
      || value.kind === "patch"
      || value.kind === "invalidate")
  ) {
    return requireExternalResourceDelivery(value, familyKind);
  }
  {
    throw new TypeError(
      `${familyKind} resource lines require deliver(...) packets created with resourceDelivery.*(...)`,
    );
  }
}

function requireDeliveryOptions(options, label) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(`${label} requires an options object`);
  }
  return Object.freeze({
    packetId: requireString(options.packetId, "packetId", label),
    basisId: normalizeOptionalString(options.basisId, "basisId", label),
    nextBasisId:
      options.nextBasisId === undefined
        ? undefined
        : normalizeOptionalString(options.nextBasisId, "nextBasisId", label),
  });
}

function normalizeOptionalString(value, fieldName, label) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireString(value, fieldName, label);
}

function requireString(value, fieldName, label) {
  if (typeof value !== "string") {
    throw new TypeError(`${label} ${fieldName} must be a string`);
  }
  return value;
}

export { requireResourceDelivery, resourceDelivery };
