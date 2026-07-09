import { isPlainObject } from "../authoring_option_validation.js";
import {
  createTaggedRequestSourceInput,
} from "../resource/requests/request_source_metadata.js";
import {
  resolveHeaderObject,
  validateBaseUrlInput,
  validateHeadersInput,
  validatePostureInput,
} from "./api_layer_input_validation.js";
import {
  composeResourceBaseUrl,
  requireResourceBaseUrl,
} from "../resource/requests/base_url_resolution.js";
import { requireResourceAuthPosture } from "../resource/requests/auth_posture.js";
import {
  requireResourceContinuationPosture,
} from "../resource/requests/continuation_posture.js";
import {
  requireResourceRequestContext,
  resourceRequestContext,
} from "../resource/requests/request_context.js";
import {
  requireResourceProcessingJobPosture,
} from "../resource/processing/processing_job_posture.js";
import {
  requireResourceUploadTransportPosture,
} from "../resource/uploads/upload_transport_posture.js";
import {
  requireResourceEffectProfile,
} from "../resource/effects/resource_effect_profile.js";

const API_LAYER_KEYS = new Set([
  "baseUrl",
  "auth",
  "headers",
  "requestContext",
  "continuation",
  "processingJob",
  "uploadTransport",
  "effects",
]);

function normalizeApiLayer(label, options = {}) {
  if (!isPlainObject(options)) {
    throw new TypeError("signals.api(...) expects a plain object of scoped request defaults");
  }
  for (const key of Object.keys(options)) {
    if (!API_LAYER_KEYS.has(key)) {
      throw new TypeError(
        `signals.api(...) does not admit unknown scoped default "${key}"`,
      );
    }
  }
  validateBaseUrlInput(options.baseUrl);
  validatePostureInput("auth", options.auth, requireResourceAuthPosture);
  validateHeadersInput(options.headers);
  validatePostureInput(
    "requestContext",
    options.requestContext,
    requireResourceRequestContext,
  );
  validatePostureInput(
    "continuation",
    options.continuation,
    requireResourceContinuationPosture,
  );
  validatePostureInput(
    "processingJob",
    options.processingJob,
    requireResourceProcessingJobPosture,
  );
  validatePostureInput(
    "uploadTransport",
    options.uploadTransport,
    requireResourceUploadTransportPosture,
  );
  validatePostureInput(
    "effects",
    options.effects,
    requireResourceEffectProfile,
  );
  return Object.freeze({
    label,
    baseUrl: options.baseUrl,
    auth: options.auth,
    headers: options.headers,
    requestContext: options.requestContext,
    continuation: options.continuation,
    processingJob: options.processingJob,
    uploadTransport: options.uploadTransport,
    effects: options.effects,
  });
}

function mergeApiDeclaration(layers, declaration) {
  if (!isPlainObject(declaration)) {
    throw new TypeError("api family declarations must be plain objects");
  }
  validateBaseUrlInput(declaration.baseUrl);
  validateHeadersInput(declaration.headers);
  const merged = { ...declaration };
  delete merged.headers;
  merged.baseUrl = mergeBaseUrlInput(layers, declaration.baseUrl);
  merged.auth = mergeTaggedInput(
    layers,
    declaration.auth,
    "auth",
    "default.auth",
  );
  merged.requestContext = mergeRequestContextInput(
    layers,
    declaration.headers,
    declaration.requestContext,
  );
  merged.continuation = mergeTaggedInput(
    layers,
    declaration.continuation,
    "continuation",
    "default.continuation",
  );
  merged.processingJob = mergeTaggedInput(
    layers,
    declaration.processingJob,
    "processingJob",
    "default.processingJob",
  );
  merged.uploadTransport = mergeTaggedInput(
    layers,
    declaration.uploadTransport,
    "uploadTransport",
    "default.uploadTransport",
  );
  merged.effects = mergeTaggedInput(
    layers,
    declaration.effects,
    "effects",
    "default.effects",
  );
  return merged;
}

function mergeTaggedInput(
  layers,
  endpointInput,
  field,
  defaultSource,
) {
  if (
    endpointInput === undefined
    && layers.every((layer) => layer[field] === undefined)
  ) {
    return undefined;
  }
  return createTaggedRequestSourceInput((params) => {
    let resolvedValue = undefined;
    let source = freezeTaggedSource(defaultSource, false);
    let resolved = false;
    for (const layer of layers) {
      if (layer[field] === undefined) {
        continue;
      }
      resolvedValue = resolveInputValue(layer[field], params, field);
      source = freezeTaggedSource(`${layer.label}.${field}`, resolved);
      resolved = true;
    }
    if (endpointInput !== undefined) {
      resolvedValue = resolveInputValue(endpointInput, params, field);
      source = freezeTaggedSource(`endpoint.${field}`, resolved);
    }
    return Object.freeze({
      value: resolvedValue,
      source,
    });
  });
}

function mergeBaseUrlInput(layers, endpointInput) {
  if (
    endpointInput === undefined
    && layers.every((layer) => layer.baseUrl === undefined)
  ) {
    return undefined;
  }
  return createTaggedRequestSourceInput((params) => {
    let value = null;
    const sources = [];
    for (const layer of layers) {
      if (layer.baseUrl === undefined) {
        continue;
      }
      value = composeResourceBaseUrl(
        value,
        requireResourceBaseUrl(resolveInputValue(layer.baseUrl, params, "baseUrl"), "api"),
        `${layer.label}.baseUrl`,
      );
      sources.push(`${layer.label}.baseUrl`);
    }
    if (endpointInput !== undefined) {
      value = composeResourceBaseUrl(
        value,
        requireResourceBaseUrl(resolveInputValue(endpointInput, params, "baseUrl"), "api"),
        "endpoint.baseUrl",
      );
      sources.push("endpoint.baseUrl");
    }
    return Object.freeze({
      value,
      source: Object.freeze({
        sources: Object.freeze([...sources]),
      }),
    });
  });
}

function mergeRequestContextInput(layers, endpointHeaders, endpointRequestContext) {
  if (
    endpointHeaders === undefined
    && endpointRequestContext === undefined
    && layers.every(
      (layer) =>
        layer.headers === undefined && layer.requestContext === undefined,
    )
  ) {
    return undefined;
  }
  return createTaggedRequestSourceInput((params) => {
    const headers = {};
    const headerSources = {};
    let correlationId = null;
    let correlationSource = null;
    let branchId = null;
    let branchSource = null;
    let basisId = null;
    let basisSource = null;

    for (const layer of layers) {
      applyHeaders(
        headers,
        headerSources,
        resolveOptionalHeaders(layer.headers, params),
        `${layer.label}.headers`,
      );
      applyContext(
        headers,
        headerSources,
        resolveOptionalContext(layer.requestContext, params),
        `${layer.label}.requestContext`,
        (nextValue, nextSource) => {
          correlationId = nextValue;
          correlationSource = nextSource;
        },
        () => correlationId,
        (nextValue, nextSource) => {
          branchId = nextValue;
          branchSource = nextSource;
        },
        () => branchId,
        (nextValue, nextSource) => {
          basisId = nextValue;
          basisSource = nextSource;
        },
        () => basisId,
      );
    }

    applyHeaders(
      headers,
      headerSources,
      resolveOptionalHeaders(endpointHeaders, params),
      "endpoint.headers",
    );
    applyContext(
      headers,
      headerSources,
      resolveOptionalContext(endpointRequestContext, params),
      "endpoint.requestContext",
      (nextValue, nextSource) => {
        correlationId = nextValue;
        correlationSource = nextSource;
      },
      () => correlationId,
      (nextValue, nextSource) => {
        branchId = nextValue;
        branchSource = nextSource;
      },
      () => branchId,
      (nextValue, nextSource) => {
        basisId = nextValue;
        basisSource = nextSource;
      },
      () => basisId,
    );

    return Object.freeze({
      value: resourceRequestContext({
        headers,
        correlationId,
        branchId,
        basisId,
      }),
      source: Object.freeze({
        headers: Object.freeze(headerSources),
        correlationId: correlationSource,
        branchId: branchSource,
        basisId: basisSource,
      }),
    });
  });
}

function applyContext(
  headers,
  headerSources,
  context,
  source,
  setCorrelationId,
  readCorrelationId,
  setBranchId,
  readBranchId,
  setBasisId,
  readBasisId,
) {
  if (context === null) {
    return;
  }
  applyHeaders(headers, headerSources, context.headers, source);
  if (context.correlationId !== null) {
    setCorrelationId(
      context.correlationId,
      freezeFieldSource(source, readCorrelationId() !== null),
    );
  }
  if (context.branchId !== null) {
    setBranchId(
      context.branchId,
      freezeFieldSource(source, readBranchId() !== null),
    );
  }
  if (context.basisId !== null) {
    setBasisId(
      context.basisId,
      freezeFieldSource(source, readBasisId() !== null),
    );
  }
}

function applyHeaders(currentHeaders, currentSources, nextHeaders, source) {
  if (nextHeaders === null) {
    return;
  }
  for (const [name, value] of Object.entries(nextHeaders)) {
    const overridden = Object.prototype.hasOwnProperty.call(currentHeaders, name);
    currentHeaders[name] = value;
    currentSources[name] = freezeFieldSource(source, overridden);
  }
}

function resolveOptionalHeaders(input, params) {
  if (input === undefined) {
    return null;
  }
  return resolveHeaderObject(resolveInputValue(input, params, "headers"));
}

function resolveOptionalContext(input, params) {
  if (input === undefined) {
    return null;
  }
  return requireResourceRequestContext(
    resolveInputValue(input, params, "requestContext"),
    "api",
  );
}

function resolveInputValue(input, params, field) {
  return typeof input === "function" ? input(params) : input;
}

function freezeFieldSource(source, overridden) {
  return Object.freeze({ source, overridden });
}

function freezeTaggedSource(source, overridden) {
  return Object.freeze({ source, overridden });
}

export { mergeApiDeclaration, normalizeApiLayer };
