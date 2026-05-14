import { createResponseLensProof } from "./resource_response_lens_proof.js";
import { resourceDetailFields } from "../reconciliation/resource_detail_fields.js";
import {
  requireResourceDetailRegions,
  resourceDetailRegions,
} from "../reconciliation/resource_detail_regions.js";
import {
  requireResourceDetailJsonPaths,
  resourceDetailJsonPaths,
} from "../reconciliation/resource_detail_json_paths.js";

const RESOURCE_DETAIL_RESPONSE = Symbol("forgeSignal.resourceDetailResponse");

function detail() {
  return function defineDetailResponse(fields = undefined) {
    const source = "resource.response.detail<T>()";
    const detailFields =
      fields === undefined
        ? null
        : createObjectDetailFields(fields, source);
    return Object.freeze({
      kind: "detail",
      source,
      fields: detailFields,
      jsonPaths: null,
      regions: null,
      lensProof: createResponseLensProof({
        source,
        topology: "detail",
        fieldNames:
          detailFields === null
            ? []
            : Object.keys(detailFields.definitions),
        regionNames: [],
        jsonPathNames: [],
        itemField: null,
        aspectNames: [],
        jsonAspectNames: [],
        summaryNames: [],
        summaryPatchScope: null,
      }),
      [RESOURCE_DETAIL_RESPONSE]: "resourceDetailResponse",
    });
  };
}

function detailJsonPaths() {
  return function defineDetailJsonPathResponse(paths) {
    const source = "resource.response.detailJsonPaths<T>()";
    let detailJsonPaths;
    try {
      detailJsonPaths = requireResourceDetailJsonPaths(paths, source);
    } catch {
      detailJsonPaths = resourceDetailJsonPaths(paths);
    }
    return Object.freeze({
      kind: "detail",
      source,
      fields: null,
      jsonPaths: detailJsonPaths,
      regions: null,
      lensProof: createResponseLensProof({
        source,
        topology: "detail",
        fieldNames: [],
        regionNames: [],
        jsonPathNames: Object.keys(detailJsonPaths.definitions),
        itemField: null,
        aspectNames: [],
        jsonAspectNames: [],
        summaryNames: [],
        summaryPatchScope: null,
      }),
      [RESOURCE_DETAIL_RESPONSE]: "resourceDetailResponse",
    });
  };
}

function detailRegions() {
  return function defineDetailRegionResponse(regions) {
    const source = "resource.response.detailRegions<T>()";
    let detailRegions;
    try {
      detailRegions = requireResourceDetailRegions(regions, source);
    } catch {
      detailRegions = resourceDetailRegions(regions);
    }
    return Object.freeze({
      kind: "detail",
      source,
      fields: null,
      jsonPaths: null,
      regions: detailRegions,
      lensProof: createResponseLensProof({
        source,
        topology: "detail",
        fieldNames: [],
        regionNames: Object.keys(detailRegions.definitions),
        jsonPathNames: [],
        itemField: null,
        aspectNames: [],
        jsonAspectNames: [],
        summaryNames: [],
        summaryPatchScope: null,
      }),
      [RESOURCE_DETAIL_RESPONSE]: "resourceDetailResponse",
    });
  };
}

function createObjectDetailFields(fields, source) {
  if (!fields || typeof fields !== "object" || Array.isArray(fields)) {
    throw new TypeError(`${source} requires a detail field object`);
  }
  const definitions = {};
  for (const [fieldName, objectField] of readDetailFieldDeclarations(fields, source)) {
    if (typeof fieldName !== "string" || fieldName.length === 0) {
      throw new TypeError(`${source} field names must be non-empty strings`);
    }
    if (typeof objectField !== "string" || objectField.length === 0) {
      throw new TypeError(
        `${source} field "${fieldName}" requires a non-empty object field name`,
      );
    }
    requireSafeObjectDetailFieldName(source, fieldName);
    requireSafeObjectDetailBackingField(source, fieldName, objectField);
    definitions[fieldName] = Object.freeze({
      read(value) {
        return readObjectDetailValueField(source, value, fieldName, objectField);
      },
      extract(value) {
        return extractObjectDetailValueField(source, value, fieldName, objectField);
      },
      write(value, fieldValue) {
        return writeObjectDetailValueField(
          source,
          value,
          fieldName,
          objectField,
          fieldValue,
        );
      },
    });
  }
  return resourceDetailFields(definitions);
}

function readDetailFieldDeclarations(fields, source) {
  const declarations = [];
  for (const key of Reflect.ownKeys(fields)) {
    if (typeof key !== "string") {
      continue;
    }
    const descriptor = Object.getOwnPropertyDescriptor(fields, key);
    if (descriptor === undefined || !descriptor.enumerable) {
      continue;
    }
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `${source} rejects accessor detail field declaration "${key}"`,
      );
    }
    declarations.push(Object.freeze([key, descriptor.value]));
  }
  return declarations;
}

function requireSafeObjectDetailFieldName(source, fieldName) {
  if (
    fieldName === "__proto__" ||
    fieldName === "constructor" ||
    fieldName === "prototype"
  ) {
    throw new TypeError(`${source} rejects unsafe detail field "${fieldName}"`);
  }
}

function requireSafeObjectDetailBackingField(source, fieldName, objectField) {
  if (
    objectField === "__proto__" ||
    objectField === "constructor" ||
    objectField === "prototype"
  ) {
    throw new TypeError(
      `${source} field "${fieldName}" rejects unsafe object field "${objectField}"`,
    );
  }
}

function requireObjectDetailValue(source, value, fieldName) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${source} field "${fieldName}" requires object detail values`);
  }
  return value;
}

function readObjectDetailValueField(source, value, fieldName, objectField) {
  const descriptor = readObjectDetailValueFieldDescriptor(
    source,
    value,
    fieldName,
    objectField,
  );
  return descriptor === null ? undefined : descriptor.value;
}

function extractObjectDetailValueField(source, value, fieldName, objectField) {
  const descriptor = readObjectDetailValueFieldDescriptor(
    source,
    value,
    fieldName,
    objectField,
  );
  if (descriptor === null) {
    return Object.freeze({
      present: false,
      value: undefined,
    });
  }
  return Object.freeze({
    present: true,
    value: descriptor.value,
  });
}

function writeObjectDetailValueField(source, value, fieldName, objectField, fieldValue) {
  const objectValue = requireObjectDetailValue(source, value, fieldName);
  const nextValue = Object.create(Object.getPrototypeOf(objectValue));
  let fieldWritten = false;
  for (const key of Reflect.ownKeys(objectValue)) {
    const descriptor = Object.getOwnPropertyDescriptor(objectValue, key);
    if (descriptor === undefined) {
      continue;
    }
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `${source} field "${fieldName}" rejects accessor detail value property ${formatObjectDetailPropertyLabel(key)}`,
      );
    }
    if (key === objectField) {
      fieldWritten = true;
      Object.defineProperty(nextValue, key, {
        ...descriptor,
        value: fieldValue,
      });
      continue;
    }
    Object.defineProperty(nextValue, key, descriptor);
  }
  if (!fieldWritten) {
    Object.defineProperty(nextValue, objectField, {
      configurable: true,
      enumerable: true,
      writable: true,
      value: fieldValue,
    });
  }
  return nextValue;
}

function readObjectDetailValueFieldDescriptor(source, value, fieldName, objectField) {
  const objectValue = requireObjectDetailValue(source, value, fieldName);
  const descriptor = Object.getOwnPropertyDescriptor(objectValue, objectField);
  if (descriptor === undefined) {
    return null;
  }
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `${source} field "${fieldName}" rejects accessor detail value field "${objectField}"`,
    );
  }
  return descriptor;
}

function formatObjectDetailPropertyLabel(key) {
  return typeof key === "string" ? `"${key}"` : String(key);
}

function requireResourceDetailResponse(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_RESPONSE] !== "resourceDetailResponse"
  ) {
    throw new TypeError(`${kind} requires a resource.response detail contract`);
  }
  return value;
}

function isResourceDetailResponse(value) {
  return Boolean(
    value &&
    typeof value === "object" &&
    value[RESOURCE_DETAIL_RESPONSE] === "resourceDetailResponse",
  );
}

export {
  detail,
  detailRegions,
  detailJsonPaths,
  isResourceDetailResponse,
  requireResourceDetailResponse,
};
