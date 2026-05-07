import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import {
  assertSecretAbsentFromArtifacts,
  createRequestArtifactDigest,
} from "../runtime_fixture/proof/request_artifacts.mjs";

test("equivalent auth and request-context declarations lower to one canonical request story", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    const secretToken = "bearer secret-token";
    const direct = resource.detail({
      params: mod.resourceParams(),
      auth: mod.resourceAuth.workspace(),
      requestContext: mod.resourceRequestContext({
        headers: {
          authorization: secretToken,
          "x-workspace-id": "demo",
        },
        correlationId: "trace-7",
        branchId: 42,
        basisId: "basis-1",
      }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });
    const derived = resource.detail({
      params: mod.resourceParams(),
      auth: ({ workspaceId }) =>
        workspaceId === "demo"
          ? mod.resourceAuth.workspace()
          : mod.resourceAuth.anonymous(),
      requestContext: ({ workspaceId }) =>
        mod.resourceRequestContext({
          headers: {
            authorization: secretToken,
            "x-workspace-id": workspaceId,
          },
          correlationId: "trace-7",
          branchId: 42,
          basisId: "basis-1",
        }),
      normalizeParams: ({ workspaceId, productId }) =>
        mod.resourceParamIdentity(
          { workspaceId, productId },
          `${workspaceId}:${productId}`,
        ),
      load: ({ productId }) => ({ id: productId }),
    });

    const directLine = direct.line({ productId: "p1" });
    const derivedLine = derived.line({
      workspaceId: "demo",
      productId: "p1",
    });

    assert.equal(
      createRequestArtifactDigest(directLine),
      createRequestArtifactDigest(derivedLine),
    );
    assert.deepEqual(directLine.diagnostics().request.context.headerNames, [
      "authorization",
      "x-workspace-id",
    ]);
    assert.equal(directLine.request().context.headers.authorization, secretToken);
    assert.equal(typeof directLine.history().availability.replay.kind, "string");
    assertSecretAbsentFromArtifacts(directLine, secretToken);

    directLine.refresh();
    derivedLine.refresh();
    assert.equal(
      createRequestArtifactDigest(directLine),
      createRequestArtifactDigest(derivedLine),
    );

    directLine.free();
    const rematerialized = direct.line({ productId: "p1" });
    assert.equal(
      createRequestArtifactDigest(rematerialized),
      createRequestArtifactDigest(derivedLine),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("incompatible auth and request-context posture is denied before load work begins", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    let invalidAuthLoadCalled = false;
    let invalidContextLoadCalled = false;
    const invalidAuth = resource.detail({
      params: mod.resourceParams(),
      auth: () => ({ kind: "workspace" }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        invalidAuthLoadCalled = true;
        return { id: productId };
      },
    });
    const invalidContext = resource.detail({
      params: mod.resourceParams(),
      requestContext: () => ({ headers: { authorization: "secret" } }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        invalidContextLoadCalled = true;
        return { id: productId };
      },
    });

    assert.throws(
      () => invalidAuth.line({ productId: "p1" }),
      /auth created with resourceAuth/,
    );
    assert.throws(
      () => invalidContext.line({ productId: "p1" }),
      /requestContext created with resourceRequestContext/,
    );
    assert.equal(invalidAuthLoadCalled, false);
    assert.equal(invalidContextLoadCalled, false);
  } finally {
    await runtime.cleanup();
  }
});
