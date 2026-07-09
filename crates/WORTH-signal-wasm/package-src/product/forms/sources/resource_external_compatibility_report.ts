import { stableValueDigest } from "../values/value_paths.js";

export function readFormResourceExternalCompatibilityReport(compatibility) {
  if (compatibility.kind === "native") {
    return Object.freeze({
      kind: "native",
      definitionId: null,
      version: null,
      requestContract: null,
      reconciliationContract: null,
      deliveryContract: "nativeInternalLine",
      digest: stableValueDigest({
        kind: "native",
        deliveryContract: "nativeInternalLine",
      }),
    });
  }
  return Object.freeze({
    kind: "externalDefinition",
    definitionId: compatibility.definitionId,
    version: compatibility.version,
    requestContract: compatibility.requestContract,
    reconciliationContract: compatibility.reconciliationContract,
    deliveryContract: "basisCompatV1",
    digest: stableValueDigest({
      kind: "externalDefinition",
      definitionId: compatibility.definitionId,
      version: compatibility.version,
      requestContract: compatibility.requestContract,
      reconciliationContract: compatibility.reconciliationContract,
      deliveryContract: "basisCompatV1",
    }),
  });
}
