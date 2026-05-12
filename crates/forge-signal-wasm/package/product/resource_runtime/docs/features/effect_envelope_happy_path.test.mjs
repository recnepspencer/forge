import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resource-contracts/effect-envelope.md",
);

test("effect envelope doc covers runtime-issued proof fields", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /ResourceEffectEnvelope/);
  assert.match(doc, /runtime-issued effect envelope/i);
  assert.match(doc, /authority/i);
  assert.match(doc, /counters/i);
  assert.match(doc, /forged or tampered/i);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "effect-envelope-doc");
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/doc-effect-envelope")
      .items((task) => task.id)
      .aspect("title", (task) => task.title, (task, title) => ({
        ...task,
        title,
      }))
      .list({
        load: () => [{ id: "task:1", title: "First" }],
      });

    const line = tasks.line({});
    line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Draft",
    }));

    const effect = line.diagnostics().lastEffect;

    assert.equal(effect.version, "resource-effect-envelope-v1");
    assert.equal(effect.profile.name, "branchNative");
    assert.equal(effect.provenance, "localPatch");
    assert.equal(effect.patch.scope, "aspect");
    assert.equal(effect.locus.kind, "itemAspect");
    assert.equal(typeof effect.authority.envelopeDigest, "string");
    assert.equal(effect.counters.rollbackReadinessBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});
