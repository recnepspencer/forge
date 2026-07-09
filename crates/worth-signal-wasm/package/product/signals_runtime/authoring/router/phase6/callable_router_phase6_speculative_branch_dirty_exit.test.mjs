import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import {
  createHistoryStub,
  createSpecialistStub,
} from "./callable_router_phase6_speculative_branch_support.mjs";

test("phase-6 speculative sessions expose dirty-exit posture and explicit confirmation witnesses", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const cleanSpecialist = createSpecialistStub();
    const dirtySpecialist = createSpecialistStub({ touchedNodes: 3, nodesEvaluated: 2 });
    const history = createHistoryStub();
    const speculativePlan = routes.speculate("/users/u1");
    assert.ok(speculativePlan);
    const cleanSession = await speculativePlan.open(history);

    const cleanExit = await cleanSession.dirtyExit(cleanSpecialist);
    assert.equal(cleanExit.disposition, "clean-exit");
    assert.equal(cleanExit.confirmationRequired, false);
    assert.equal(cleanExit.confirm(), null);

    const dirtySession = await speculativePlan.open(createHistoryStub());
    const dirtyExit = await dirtySession.dirtyExit(dirtySpecialist);
    const dirtyConfirmation = dirtyExit.confirm();

    assert.equal(dirtyExit.disposition, "dirty-exit-requires-confirmation");
    assert.equal(dirtyExit.confirmationRequired, true);
    assert.ok(dirtyConfirmation);
    assert.equal(dirtyExit.runSummary.touchedNodes, 3);
    assert.match(
      dirtyExit.verification().speculativeDirtyExitDigest,
      /speculative-branch-dirty-exit/,
    );
    assert.match(
      dirtyConfirmation.verification().speculativeDirtyExitConfirmationDigest,
      /speculative-branch-dirty-exit-confirmation/,
    );
    assert.deepEqual(cleanSpecialist.calls, [["evaluateDirty"]]);
    assert.deepEqual(dirtySpecialist.calls, [["evaluateDirty"]]);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative commit fails closed without matching dirty-exit proof and confirmation", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const history = createHistoryStub();
    const speculativePlan = routes.speculate("/users/u1");
    assert.ok(speculativePlan);
    const session = await speculativePlan.open(history);
    const preview = await session.commitPreview();

    await assert.rejects(
      () => session.commit(preview),
      /requires a dirty-exit artifact/,
    );

    const dirtyExit = await session.dirtyExit(
      createSpecialistStub({ touchedNodes: 2, nodesEvaluated: 1 }),
    );
    await assert.rejects(
      () => session.commit(preview, dirtyExit),
      /requires an explicit dirty-exit confirmation witness/,
    );
    await assert.rejects(
      () => session.commit(preview, dirtyExit, {}),
      /requires a confirmation witness returned by dirtyExitArtifact\.confirm\(\)/,
    );

    const otherDirtyExit = await session.dirtyExit(
      createSpecialistStub({ touchedNodes: 7, nodesEvaluated: 4 }),
    );
    const otherConfirmation = otherDirtyExit.confirm();
    assert.ok(otherConfirmation);
    await assert.rejects(
      () => session.commit(preview, dirtyExit, otherConfirmation),
      /dirty-exit confirmation proof does not match the supplied dirty-exit artifact/,
    );

    const dirtyConfirmation = dirtyExit.confirm();
    assert.ok(dirtyConfirmation);
    const commit = await session.commit(preview, dirtyExit, dirtyConfirmation);
    assert.equal(commit.outcome().kind, "committed");
  } finally {
    signals.free();
    await cleanup();
  }
});
