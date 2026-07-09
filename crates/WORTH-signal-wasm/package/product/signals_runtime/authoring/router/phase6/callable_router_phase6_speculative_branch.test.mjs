import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import {
  createHistoryStub,
  createSpecialistStub,
} from "./callable_router_phase6_speculative_branch_support.mjs";

test("phase-6 projected candidates compile branch-native speculative navigation plans with explicit lifecycle posture", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const candidate = routes.project("/users/u1");
    assert.ok(candidate);

    const speculativePlan = candidate.speculate({
      branchName: "speculative-user-detail",
      commitPosture: "merge-preview-before-commit",
      discardPosture: "discard-speculative-branch",
      visiblePosture: "preserve-visible-until-commit",
    });

    assert.equal(speculativePlan.kind, "speculativeBranchPlan");
    assert.equal(speculativePlan.routeId, "detail");
    assert.equal(
      speculativePlan.branching().candidateTruth,
      "branch-native-candidate-route",
    );
    assert.equal(
      speculativePlan.branching().branchLifecycle,
      "create-branch-before-commit",
    );
    assert.equal(
      speculativePlan.branching().dirtyExit,
      "evaluate-dirty-before-commit",
    );
    assert.equal(
      speculativePlan.diagnostics().flickerSuppression,
      "suppresses-visible-flicker-until-commit",
    );
    assert.equal(
      speculativePlan.diagnostics().commitDisposition,
      "requires-merge-preview-before-commit",
    );
    assert.match(
      speculativePlan.verification().speculativeBranchDigest,
      /speculative-branch-plan/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 resolved trees expose speculative planning without admitting browser-router-shaped provisional truth", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const speculativePlan = routes.speculate("/users/u1", {
      commitPosture: "direct-merge-commit",
      discardPosture: "keep-branch-pending",
      visiblePosture: "allow-visible-flicker-before-commit",
    });
    assert.ok(speculativePlan);
    assert.equal(
      speculativePlan.branching().commitPosture,
      "direct-merge-commit",
    );
    assert.equal(
      speculativePlan.branching().discardPosture,
      "keep-branch-pending",
    );
    assert.equal(
      speculativePlan.diagnostics().discardDisposition,
      "discard-keeps-branch-pending",
    );
    assert.equal(
      speculativePlan.diagnostics().pendingDisposition,
      "candidate-route-remains-pending-until-commit",
    );
    assert.equal(routes.speculate("/missing"), null);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative evaluation produces unified pending and redirect outcomes before branch open", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const requireSignedIn = signals.router.prerequisite("signed-in", ({ facts, allow, redirect }) => (
    facts.signedIn === true
      ? allow({ reason: "signed-in" })
      : redirect({ href: "/sign-in", reason: "sign-in-required" })
  ));
  const routes = signals.router.define({
    signIn: signals.router.route("/sign-in"),
    detail: signals.router.route("/users/:userId", {
      admission: [requireSignedIn],
    }),
  });

  try {
    const pendingPlan = routes.speculate("/users/u1");
    assert.ok(pendingPlan);
    const pendingOutcome = await pendingPlan.evaluate({ signedIn: true });
    assert.equal(pendingOutcome.kind, "pending");
    assert.equal(pendingOutcome.routeOutcome()?.kind, "admitted");
    assert.equal(
      pendingOutcome.diagnostics().branchLifecycleResult,
      "notOpened",
    );
    assert.equal(
      pendingOutcome.diagnostics().branchDisposition,
      "candidate-route-remains-pending-until-commit",
    );
    assert.match(
      pendingOutcome.verification().speculativeOutcomeDigest,
      /speculative-branch-outcome/,
    );

    const redirectPlan = routes.speculate("/users/u1");
    assert.ok(redirectPlan);
    const redirectOutcome = await redirectPlan.evaluate({ signedIn: false });
    assert.equal(redirectOutcome.kind, "redirect");
    assert.equal(redirectOutcome.routeOutcome()?.kind, "redirect");
    assert.equal(
      redirectOutcome.diagnostics().branchDisposition,
      "redirect-before-branch-open",
    );
    assert.equal(
      redirectOutcome.routeOutcome()?.artifact().href,
      "/sign-in",
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 default speculative branch names stay bound to candidate route truth instead of route id alone", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const firstPlan = routes.speculate("/users/u1");
    const secondPlan = routes.speculate("/users/u2");

    assert.ok(firstPlan);
    assert.ok(secondPlan);
    assert.notEqual(firstPlan.branching().branchName, secondPlan.branching().branchName);
    assert.match(firstPlan.branching().branchName, /speculative:detail:/);
    assert.match(secondPlan.branching().branchName, /speculative:detail:/);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative branch plans open explicit history-backed sessions and use proof-bearing merge preview requests", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const history = createHistoryStub();
    const speculativePlan = routes.speculate("/users/u1", {
      branchName: "router-speculative-u1",
      discardPosture: "keep-branch-pending",
    });
    assert.ok(speculativePlan);

    const session = await speculativePlan.open(history);
    assert.equal(session.outcome().kind, "pending");
    assert.equal(
      session.outcome().diagnostics().branchLifecycleResult,
      "activeBranchPending",
    );
    const cleanExit = await session.dirtyExit(createSpecialistStub());
    const preview = await session.commitPreview({
      conflict_policy_name: "prefer-source",
    });
    const commit = await session.commit(preview, cleanExit);

    assert.equal(session.originBranch().id, 7);
    assert.equal(session.branch().id, 8);
    assert.equal(
      session.lifecycle().branchBinding,
      "candidate-route-bound-to-history-branch",
    );
    assert.equal(session.lifecycle().branchState, "active-speculative-branch");
    assert.equal(
      preview.verification().speculativeCommitPreviewDigest.includes("speculative-branch-commit-preview"),
      true,
    );
    assert.equal(
      commit.previewDisposition,
      "committed-from-preview-artifact",
    );
    assert.equal(commit.outcome().kind, "committed");
    assert.equal(
      commit.outcome().diagnostics().branchLifecycleResult,
      "merged",
    );
    assert.equal(
      commit.outcome().diagnostics().branchDisposition,
      "committed-through-history-merge",
    );
    assert.equal(history.current_branch().id, 7);
    assert.deepEqual(history.calls, [
      ["current_branch"],
      ["create_branch", "router-speculative-u1"],
      ["switch_branch", 8],
      [
        "plan_merge_policy_preview_with_proof",
        {
          source_branch_id: 8,
          target_branch_id: 7,
          conflict_policy_name: "prefer-source",
        },
      ],
      ["merge_branches_with_proof", 8, 7],
      ["switch_branch", 7],
      ["current_branch"],
    ]);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative sessions discard honestly and become terminal after commit or discard", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const commitHistory = createHistoryStub();
    const commitPlan = routes.speculate("/users/u1");
    assert.ok(commitPlan);
    const committedSession = await commitPlan.open(commitHistory);
    const committedExit = await committedSession.dirtyExit(createSpecialistStub());
    const preview = await committedSession.commitPreview();
    await committedSession.commit(preview, committedExit);

    await assert.rejects(
      () => committedSession.discard(),
      /cannot run after session commit/,
    );

    const discardHistory = createHistoryStub();
    const discardPlan = routes.speculate("/users/u1", {
      discardPosture: "keep-branch-pending",
    });
    assert.ok(discardPlan);
    const discardedSession = await discardPlan.open(discardHistory);
    const discard = await discardedSession.discard();

    assert.equal(discard.disposition, "keep-branch-pending-without-merge");
    const retainedPendingBranch = discard.pendingBranch();
    assert.ok(retainedPendingBranch);
    assert.equal(discard.outcome().kind, "pending");
    assert.equal(
      discard.outcome().diagnostics().branchLifecycleResult,
      "remainedPending",
    );
    assert.equal(
      discard.outcome().diagnostics().discardDisposition,
      "keep-branch-pending-without-merge",
    );
    const resumedSession = await retainedPendingBranch.resume(discardHistory);
    assert.equal(resumedSession.branch().id, retainedPendingBranch.branch().id);
    assert.equal(resumedSession.outcome().kind, "pending");
    const resumedExit = await resumedSession.dirtyExit(createSpecialistStub());
    const resumedPreview = await resumedSession.commitPreview();
    const resumedCommit = await resumedSession.commit(resumedPreview, resumedExit);
    assert.equal(resumedCommit.outcome().kind, "committed");
    assert.equal(discardHistory.current_branch().id, 7);
    await assert.rejects(
      () => discardedSession.commitPreview(),
      /cannot run after session discard/,
    );

    const abandonedHistory = createHistoryStub();
    const abandonedPlan = routes.speculate("/users/u1", {
      discardPosture: "discard-speculative-branch",
    });
    assert.ok(abandonedPlan);
    const abandonedSession = await abandonedPlan.open(abandonedHistory);
    const abandonedDiscard = await abandonedSession.discard();
    assert.equal(abandonedDiscard.pendingBranch(), null);
    assert.equal(abandonedDiscard.outcome().kind, "discarded");
    assert.equal(
      abandonedDiscard.outcome().diagnostics().branchLifecycleResult,
      "discarded",
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative planning fails closed for invalid branch lifecycle options", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const candidate = routes.project("/users/u1");
    assert.ok(candidate);

    assert.throws(
      () => candidate.speculate({ branchName: "" }),
      /branchName must be a non-empty string/,
    );
    assert.throws(
      () => candidate.speculate({ commitPosture: "branchLater" }),
      /commitPosture must be one of/,
    );
    assert.throws(
      () => candidate.speculate({ discardPosture: "WORTHtIt" }),
      /discardPosture must be one of/,
    );
    assert.throws(
      () => candidate.speculate({ visiblePosture: "maybePreserve" }),
      /visiblePosture must be one of/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 speculative sessions fail closed for invalid history surfaces and preview overrides", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const candidate = routes.project("/users/u1");
    assert.ok(candidate);
    const plan = candidate.speculate();

    assert.throws(
      () => plan.open({}),
      /requires history.current_branch/,
    );
    const history = createHistoryStub();
    assert.throws(
      () => plan.open({
        current_branch: history.current_branch,
        create_branch: history.create_branch,
        plan_merge_policy_preview_with_proof: history.plan_merge_policy_preview_with_proof,
        merge_branches_with_proof: history.merge_branches_with_proof,
      }),
      /requires history.switch_branch/,
    );
    const session = await plan.open(history);
    await assert.rejects(
      () => session.commitPreview("bad"),
      /commitPreview\(\.\.\.\) options must be an object/,
    );
    await assert.rejects(
      () => session.commitPreview({ merge_kind: "later" }),
      /does not support: merge_kind/,
    );
    await assert.rejects(
      () => session.dirtyExit({}),
      /requires a specialist.evaluateDirty/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});
