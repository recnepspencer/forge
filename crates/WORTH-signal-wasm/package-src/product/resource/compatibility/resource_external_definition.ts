const RESOURCE_EXTERNAL_DEFINITION_VERSION = "WORTH-resource-external-v1";
const RESOURCE_EXTERNAL_REQUEST_CONTRACT = "native-v1";

function admitExternalDetailDefinition(definition) {
  const external = requireExternalDefinitionEnvelope("detail", definition);
  if (external.reconciliationContract !== "none") {
    throw new TypeError(
      `external detail resource definitions require reconciliationContract "none"; received "${external.reconciliationContract}"`,
    );
  }
  return createAdmittedExternalDefinition(external);
}

function admitExternalCollectionDefinition(definition) {
  const external = requireExternalDefinitionEnvelope("collection", definition);
  const expectedContract =
    external.declaration.reconcile === undefined ? "none" : "collection-v1";
  if (external.reconciliationContract !== expectedContract) {
    throw new TypeError(
      `external collection resource definitions require reconciliationContract "${expectedContract}"`,
    );
  }
  return createAdmittedExternalDefinition(external);
}

function admitExternalPagedDefinition(definition) {
  const external = requireExternalDefinitionEnvelope("paged", definition);
  const expectedContract =
    external.declaration.reconcile === undefined ? "none" : "paged-v1";
  if (external.reconciliationContract !== expectedContract) {
    throw new TypeError(
      `external paged resource definitions require reconciliationContract "${expectedContract}"`,
    );
  }
  return createAdmittedExternalDefinition(external);
}

function requireExternalDefinitionEnvelope(expectedFamily, definition) {
  if (
    !definition ||
    typeof definition !== "object" ||
    Array.isArray(definition)
  ) {
    throw new TypeError(
      `external ${expectedFamily} resource definition must be an object`,
    );
  }
  if (definition.version !== RESOURCE_EXTERNAL_DEFINITION_VERSION) {
    throw new TypeError(
      `external ${expectedFamily} resource definition version must be "${RESOURCE_EXTERNAL_DEFINITION_VERSION}"`,
    );
  }
  if (definition.family !== expectedFamily) {
    throw new TypeError(
      `external ${expectedFamily} resource definition must declare family "${expectedFamily}"`,
    );
  }
  if (
    typeof definition.definitionId !== "string" ||
    definition.definitionId.trim().length === 0
  ) {
    throw new TypeError(
      `external ${expectedFamily} resource definitions require non-empty definitionId`,
    );
  }
  if (definition.requestContract !== RESOURCE_EXTERNAL_REQUEST_CONTRACT) {
    throw new TypeError(
      `external ${expectedFamily} resource definitions require requestContract "${RESOURCE_EXTERNAL_REQUEST_CONTRACT}"`,
    );
  }
  if (
    !definition.declaration ||
    typeof definition.declaration !== "object" ||
    Array.isArray(definition.declaration)
  ) {
    throw new TypeError(
      `external ${expectedFamily} resource definitions require declaration`,
    );
  }
  return definition;
}

function createAdmittedExternalDefinition(external) {
  return Object.freeze({
    declaration: external.declaration,
    compatibility: Object.freeze({
      kind: "externalDefinition",
      version: external.version,
      definitionId: external.definitionId,
      requestContract: external.requestContract,
      reconciliationContract: external.reconciliationContract,
    }),
  });
}

export {
  RESOURCE_EXTERNAL_DEFINITION_VERSION,
  RESOURCE_EXTERNAL_REQUEST_CONTRACT,
  admitExternalCollectionDefinition,
  admitExternalDetailDefinition,
  admitExternalPagedDefinition,
};
