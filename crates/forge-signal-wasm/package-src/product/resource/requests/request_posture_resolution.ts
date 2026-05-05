import { resolveResourceBaseUrl } from "./base_url_resolution.js";
import { requireResourceAuthPosture } from "./auth_posture.js";
import { requireResourceContinuationPosture } from "./continuation_posture.js";
import { resourceAuth } from "./resource_auth.js";
import { resourceContinuation } from "./resource_continuation.js";
import {
  requireResourceRequestContext,
  resourceRequestContext,
} from "./request_context.js";
import {
  readTaggedRequestSourceResolution,
} from "./request_source_metadata.js";

function resolveResourceAuthPosture(input, params, family) {
  if (input === undefined) {
    return Object.freeze({
      value: resourceAuth.anonymous(),
      source: Object.freeze({ source: "default.auth", overridden: false }),
    });
  }
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceAuthPosture(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceAuthPosture(value, family),
    source: Object.freeze({ source: "endpoint.auth", overridden: false }),
  });
}

function resolveResourceBaseUrlPosture(input, params, family) {
  return resolveResourceBaseUrl(
    input,
    params,
    family,
    readTaggedRequestSourceResolution,
  );
}

function resolveResourceRequestContext(input, params, family) {
  if (input === undefined) {
    return Object.freeze({
      value: resourceRequestContext(),
      source: Object.freeze({
        headers: Object.freeze({}),
        correlationId: null,
        branchId: null,
        basisId: null,
      }),
    });
  }
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceRequestContext(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceRequestContext(value, family),
    source: createDefaultContextSource(value),
  });
}

function resolveResourceContinuationPosture(input, params, family) {
  if (input === undefined) {
    return Object.freeze({
      value: resourceContinuation.none(),
      source: Object.freeze({
        source: "default.continuation",
        overridden: false,
      }),
    });
  }
  const tagged = readTaggedRequestSourceResolution(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceContinuationPosture(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceContinuationPosture(value, family),
    source: Object.freeze({
      source: "endpoint.continuation",
      overridden: false,
    }),
  });
}

function createDefaultContextSource(context) {
  const headers = {};
  for (const name of Object.keys(context.headers)) {
    headers[name] = Object.freeze({
      source: "endpoint.requestContext",
      overridden: false,
    });
  }
  return Object.freeze({
    headers: Object.freeze(headers),
    correlationId:
      context.correlationId === null
        ? null
        : Object.freeze({
            source: "endpoint.requestContext",
            overridden: false,
          }),
    branchId:
      context.branchId === null
        ? null
        : Object.freeze({
            source: "endpoint.requestContext",
            overridden: false,
          }),
    basisId:
      context.basisId === null
        ? null
        : Object.freeze({
            source: "endpoint.requestContext",
            overridden: false,
          }),
  });
}

export {
  resolveResourceBaseUrlPosture,
  resolveResourceAuthPosture,
  resolveResourceContinuationPosture,
  resolveResourceRequestContext,
};
