import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

function requestWithoutSources(request) {
  const copy = normalizeForProof(request);
  delete copy.sources;
  delete copy.family.familyId;
  return copy;
}

function diagnosticsRequestWithoutSources(line) {
  const copy = normalizeForProof(line.diagnostics().request);
  delete copy.sources;
  return copy;
}

test("signals.api shared scoped defaults lower to the same admitted request posture as explicit endpoint declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, signals, signalsMod } = runtime;
    const scopedApi = signals.api({
      baseUrl: "/api",
      auth: signalsMod.resourceAuth.workspace(),
      headers: {
        authorization: "Bearer shared",
      },
      continuation: signalsMod.resourceContinuation.callback({
        callbackId: "shared-callback",
        returnTo: "/shared",
      }),
      processingJob: signalsMod.resourceProcessingJob.poll(),
      uploadTransport: signalsMod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
    }).scope({
      headers: ({ workspaceId }) => ({
        "x-workspace-id": workspaceId,
      }),
      requestContext: ({ workspaceId }) =>
        signalsMod.resourceRequestContext({
          correlationId: `workspace:${workspaceId}`,
          branchId: 7,
        }),
    });

    const scopedDetail = scopedApi.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId, productId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId, productId },
          `${workspaceId}:${productId}`,
        ),
      load: ({ productId }) => ({ id: productId }),
    });
    const explicitDetail = signals.resource.detail({
      params: signalsMod.resourceParams(),
      baseUrl: "/api",
      auth: signalsMod.resourceAuth.workspace(),
      requestContext: ({ workspaceId }) =>
        signalsMod.resourceRequestContext({
          headers: {
            authorization: "Bearer shared",
            "x-workspace-id": workspaceId,
          },
          correlationId: `workspace:${workspaceId}`,
          branchId: 7,
        }),
      continuation: signalsMod.resourceContinuation.callback({
        callbackId: "shared-callback",
        returnTo: "/shared",
      }),
      processingJob: signalsMod.resourceProcessingJob.poll(),
      uploadTransport: signalsMod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ workspaceId, productId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId, productId },
          `${workspaceId}:${productId}`,
        ),
      load: ({ productId }) => ({ id: productId }),
    });

    const scopedLine = scopedDetail.line({
      workspaceId: "demo",
      productId: "p1",
    });
    const explicitLine = explicitDetail.line({
      workspaceId: "demo",
      productId: "p1",
    });

    assert.deepEqual(
      requestWithoutSources(scopedLine.request()),
      requestWithoutSources(explicitLine.request()),
    );
    assert.deepEqual(
      diagnosticsRequestWithoutSources(scopedLine),
      diagnosticsRequestWithoutSources(explicitLine),
    );
    assert.equal(scopedLine.request().sources.auth.source, "apiRoot.auth");
    assert.deepEqual(scopedLine.request().sources.baseUrl, {
      sources: ["apiRoot.baseUrl"],
    });
    assert.equal(
      scopedLine.request().sources.context.headers.authorization.source,
      "apiRoot.headers",
    );
    assert.equal(
      scopedLine.request().sources.context.headers["x-workspace-id"].source,
      "apiScope[1].headers",
    );
    assert.equal(
      scopedLine.request().sources.context.correlationId.source,
      "apiScope[1].requestContext",
    );
    assert.equal(
      scopedLine.diagnosticsSummary().request.processingJob.kind,
      "poll",
    );
    assert.equal(
      scopedLine.diagnosticsSummary().request.uploadTransport.kind,
      "signed",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("signals.api nested scope and endpoint overrides stay deterministic and explainable", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, signals, signalsMod } = runtime;
    const api = signals.api({
      baseUrl: "/api",
      auth: signalsMod.resourceAuth.authenticated(),
      headers: {
        authorization: "Bearer root",
        "x-root": "root",
      },
    }).scope({
      auth: signalsMod.resourceAuth.workspace(),
      headers: ({ tenantId }) => ({
        authorization: `Bearer ${tenantId}`,
        "x-tenant-id": tenantId,
      }),
      requestContext: ({ tenantId }) =>
        signalsMod.resourceRequestContext({
          correlationId: `tenant:${tenantId}`,
          basisId: "basis-scope",
        }),
    }).scope({
      requestContext: () =>
        signalsMod.resourceRequestContext({
          branchId: 22,
        }),
    });

    const detail = api.detail({
      params: signalsMod.resourceParams(),
      baseUrl: "/products",
      auth: signalsMod.resourceAuth.anonymous(),
      headers: ({ productId }) => ({
        authorization: `Bearer product:${productId}`,
        "x-product-id": productId,
      }),
      requestContext: () =>
        signalsMod.resourceRequestContext({
          basisId: "basis-endpoint",
        }),
      normalizeParams: ({ tenantId, productId }) =>
        signalsMod.resourceParamIdentity(
          { tenantId, productId },
          `${tenantId}:${productId}`,
        ),
      load: ({ productId }) => ({ id: productId }),
    });

    const line = detail.line({ tenantId: "acme", productId: "p1" });

    assert.equal(line.request().auth.kind, "anonymous");
    assert.equal(line.request().baseUrl, "/api/products");
    assert.deepEqual(line.request().context.headers, {
      authorization: "Bearer product:p1",
      "x-root": "root",
      "x-tenant-id": "acme",
      "x-product-id": "p1",
    });
    assert.equal(line.request().context.correlationId, "tenant:acme");
    assert.equal(line.request().context.branchId, 22);
    assert.equal(line.request().context.basisId, "basis-endpoint");
    assert.deepEqual(line.request().sources.context.headers.authorization, {
      source: "endpoint.headers",
      overridden: true,
    });
    assert.deepEqual(line.request().sources.baseUrl, {
      sources: ["apiRoot.baseUrl", "endpoint.baseUrl"],
    });
    assert.deepEqual(line.request().sources.context.headers["x-root"], {
      source: "apiRoot.headers",
      overridden: false,
    });
    assert.deepEqual(line.request().sources.auth, {
      source: "endpoint.auth",
      overridden: true,
    });
    assert.deepEqual(line.request().sources.context.headers["x-tenant-id"], {
      source: "apiScope[1].headers",
      overridden: false,
    });
    assert.deepEqual(line.request().sources.context.headers["x-product-id"], {
      source: "endpoint.headers",
      overridden: false,
    });
    assert.deepEqual(line.request().sources.context.correlationId, {
      source: "apiScope[1].requestContext",
      overridden: false,
    });
    assert.deepEqual(line.request().sources.context.branchId, {
      source: "apiScope[2].requestContext",
      overridden: false,
    });
    assert.deepEqual(line.request().sources.context.basisId, {
      source: "endpoint.requestContext",
      overridden: true,
    });
    assert.deepEqual(
      line.diagnosticsSummary().request.sources,
      line.request().sources,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("scoped signal namespaces carry the api surface for feature-local request defaults", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const featureSignals = signals.scope("feature");
    const featureApi = featureSignals.api({
      headers: {
        "x-feature": "catalog",
      },
    });
    const detail = featureApi.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ productId }) =>
        signalsMod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const line = detail.line({ productId: "p1" });

    assert.deepEqual(line.request().context.headers, {
      "x-feature": "catalog",
    });
    assert.deepEqual(line.request().sources.context.headers["x-feature"], {
      source: "apiRoot.headers",
      overridden: false,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("signals.api composes inherited baseUrl prefixes deterministically and rejects absolute child overrides", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const scopedApi = signals.api({
      baseUrl: "https://api.example.com",
    }).scope({
      baseUrl: "/v1",
    });
    const detail = scopedApi.url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId }),
    });

    const line = detail.line({ userId: "u1" });

    assert.deepEqual(line.request().target, {
      baseUrl: "https://api.example.com/v1",
      requestPath: "/users/u1",
      url: "https://api.example.com/v1/users/u1",
    });
    assert.deepEqual(line.request().sources.baseUrl, {
      sources: ["apiRoot.baseUrl", "apiScope[1].baseUrl"],
    });
    assert.deepEqual(line.diagnosticsSummary().request.target, {
      baseUrl: "https://api.example.com/v1",
      requestPath: "/users/u1",
      url: "https://api.example.com/v1/users/u1",
    });

    assert.throws(
      () =>
        signals.api({ baseUrl: "/api" }).scope({
          baseUrl: "https://other.example.com",
        }).url("/users/:userId").detail({
          load: ({ userId }) => ({ id: userId }),
        }).line({ userId: "u1" }),
      /cannot compose an absolute baseUrl over inherited baseUrl/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("signals.api rejects baseUrl prefixes that bypass route-segment validity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;

    assert.throws(
      () =>
        signals.api({
          baseUrl: "/api//v1",
        }).url("/users/:userId").detail({
          load: ({ userId }) => ({ id: userId }),
        }).line({ userId: "u1" }),
      /baseUrl must not contain empty path segments/,
    );

    assert.throws(
      () =>
        signals.api({
          baseUrl: "https://api.example.com//v1",
        }).url("/users/:userId").detail({
          load: ({ userId }) => ({ id: userId }),
        }).line({ userId: "u1" }),
      /baseUrl must not contain empty path segments/,
    );
  } finally {
    await runtime.cleanup();
  }
});
