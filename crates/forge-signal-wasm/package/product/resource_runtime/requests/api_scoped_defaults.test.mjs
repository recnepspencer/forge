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
      auth: signalsMod.resourceAuth.authenticated(),
      headers: {
        authorization: "Bearer root",
        "x-root": "root",
      },
    }).scope({
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
    assert.deepEqual(line.request().sources.context.headers["x-root"], {
      source: "apiRoot.headers",
      overridden: false,
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

test("signals.api rejects baseUrl before the url kernel exists", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () => runtime.signals.api({ baseUrl: "/api" }),
      /does not admit baseUrl in the current DX slice/,
    );
  } finally {
    await runtime.cleanup();
  }
});
