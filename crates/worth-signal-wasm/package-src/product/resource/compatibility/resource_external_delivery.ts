import { requireResourcePatch } from "../reconciliation/resource_patch.js";

const EXTERNAL_RESOURCE_DELIVERY_BRAND = Symbol(
  "WorthSignal.externalResourceDelivery",
);

const EXTERNAL_RESOURCE_DELIVERY_VERSION = "worth-resource-external-delivery-v1";
const EXTERNAL_RESOURCE_DELIVERY_CONTRACT = "basis-compat-v1";

const resourceExternalDelivery = Object.freeze({
  replace(options) {
    const normalized = requireExternalDeliveryOptions(
      options,
      "signals.resource.compatibility.delivery.replace(...)",
    );
    return Object.freeze({
      kind: "replace",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      nextValue: options.nextValue,
      version: EXTERNAL_RESOURCE_DELIVERY_VERSION,
      contract: EXTERNAL_RESOURCE_DELIVERY_CONTRACT,
      [EXTERNAL_RESOURCE_DELIVERY_BRAND]: "externalResourceDelivery",
    });
  },
  patch(options) {
    const normalized = requireExternalDeliveryOptions(
      options,
      "signals.resource.compatibility.delivery.patch(...)",
    );
    return Object.freeze({
      kind: "patch",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      patch: options.patch,
      version: EXTERNAL_RESOURCE_DELIVERY_VERSION,
      contract: EXTERNAL_RESOURCE_DELIVERY_CONTRACT,
      [EXTERNAL_RESOURCE_DELIVERY_BRAND]: "externalResourceDelivery",
    });
  },
  invalidate(options) {
    const normalized = requireExternalDeliveryOptions(
      options,
      "signals.resource.compatibility.delivery.invalidate(...)",
    );
    return Object.freeze({
      kind: "invalidate",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      version: EXTERNAL_RESOURCE_DELIVERY_VERSION,
      contract: EXTERNAL_RESOURCE_DELIVERY_CONTRACT,
      [EXTERNAL_RESOURCE_DELIVERY_BRAND]: "externalResourceDelivery",
    });
  },
  basisRefresh(options) {
    const normalized = requireExternalDeliveryOptions(
      options,
      "signals.resource.compatibility.delivery.basisRefresh(...)",
      true,
    );
    return Object.freeze({
      kind: "basisRefresh",
      packetId: normalized.packetId,
      basisId: normalized.basisId,
      nextBasisId: normalized.nextBasisId,
      version: EXTERNAL_RESOURCE_DELIVERY_VERSION,
      contract: EXTERNAL_RESOURCE_DELIVERY_CONTRACT,
      [EXTERNAL_RESOURCE_DELIVERY_BRAND]: "externalResourceDelivery",
    });
  },
});

function requireExternalResourceDelivery(value, familyKind) {
  if (
    !value ||
    value[EXTERNAL_RESOURCE_DELIVERY_BRAND] !== "externalResourceDelivery"
  ) {
    throw new TypeError(
      `${familyKind} resource lines require external deliver(...) packets created with signals.resource.compatibility.delivery.*(...)`,
    );
  }
  if (value.contract !== EXTERNAL_RESOURCE_DELIVERY_CONTRACT) {
    throw new TypeError(
      `${familyKind} resource lines require external deliver(...) packets with contract "${EXTERNAL_RESOURCE_DELIVERY_CONTRACT}"`,
    );
  }
  if (value.version !== EXTERNAL_RESOURCE_DELIVERY_VERSION) {
    throw new TypeError(
      `${familyKind} resource lines require external deliver(...) packets with version "${EXTERNAL_RESOURCE_DELIVERY_VERSION}"`,
    );
  }
  if (value.kind === "patch") {
    requireResourcePatch(value.patch, familyKind);
  }
  return value;
}

function requireExternalDeliveryOptions(options, label, requireNextBasisId = false) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(`${label} requires an options object`);
  }
  const nextBasisId =
    options.nextBasisId === undefined
      ? undefined
      : normalizeOptionalString(options.nextBasisId, "nextBasisId", label);
  if (requireNextBasisId && (nextBasisId === undefined || nextBasisId === null)) {
    throw new TypeError(`${label} nextBasisId must be a string`);
  }
  return Object.freeze({
    packetId: requireString(options.packetId, "packetId", label),
    basisId: normalizeOptionalString(options.basisId, "basisId", label),
    nextBasisId,
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

export {
  requireExternalResourceDelivery,
  resourceExternalDelivery,
};
