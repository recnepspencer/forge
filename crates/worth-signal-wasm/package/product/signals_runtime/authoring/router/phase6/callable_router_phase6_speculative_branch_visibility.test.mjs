import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import {
  createHistoryStub,
  createSpecialistStub,
} from "./callable_router_phase6_speculative_branch_support.mjs";

test("phase-6 visible posture becomes a real pending visibility contract before and after branch open", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId"),
  });

  try {
    const preservedPlan = routes.speculate("/users/u1", {
      visiblePosture: "preserve-visible-until-commit",
    });
    const flickeringPlan = routes.speculate("/users/u1", {
      visiblePosture: "allow-visible-flicker-before-commit",
    });
    assert.ok(preservedPlan);
    assert.ok(flickeringPlan);

    const preservedPending = await preservedPlan.evaluate();
    const flickeringPending = await flickeringPlan.evaluate();
    assert.equal(
      preservedPending.visibleProjection().state,
      "candidateSuppressedUntilCommit",
    );
    assert.equal(preservedPending.visibleProjection().href, null);
    assert.equal(
      flickeringPending.visibleProjection().state,
      "candidateVisibleWhilePending",
    );
    assert.equal(flickeringPending.visibleProjection().href, "/users/u1");

    const preservedSession = await preservedPlan.open(createHistoryStub());
    const flickeringSession = await flickeringPlan.open(createHistoryStub());
    assert.equal(
      preservedSession.outcome().visibleProjection().state,
      "candidateSuppressedUntilCommit",
    );
    assert.equal(
      flickeringSession.outcome().visibleProjection().state,
      "candidateVisibleWhilePending",
    );
    assert.match(
      flickeringSession.outcome().visibleProjection().verification().speculativeVisibleProjectionDigest,
      /speculative-visible-projection/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-6 visible posture stays honest across kept-pending discard, redirect rejection, and final commit", async () => {
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
    const redirectPlan = routes.speculate("/users/u1", {
      visiblePosture: "allow-visible-flicker-before-commit",
    });
    assert.ok(redirectPlan);
    const redirectOutcome = await redirectPlan.evaluate({ signedIn: false });
    assert.equal(redirectOutcome.kind, "redirect");
    assert.equal(redirectOutcome.visibleProjection().state, "candidateNotVisible");

    const retainedPlan = routes.speculate("/users/u1", {
      discardPosture: "keep-branch-pending",
      visiblePosture: "allow-visible-flicker-before-commit",
    });
    assert.ok(retainedPlan);
    const retainedHistory = createHistoryStub();
    const retainedSession = await retainedPlan.open(retainedHistory);
    const retainedDiscard = await retainedSession.discard();
    assert.equal(retainedDiscard.outcome().kind, "pending");
    assert.equal(
      retainedDiscard.outcome().visibleProjection().state,
      "candidateVisibleWhilePending",
    );

    const resumed = await retainedDiscard.pendingBranch()?.resume(retainedHistory);
    assert.ok(resumed);
    const resumedExit = await resumed.dirtyExit(createSpecialistStub());
    const resumedPreview = await resumed.commitPreview();
    const committed = await resumed.commit(resumedPreview, resumedExit);
    assert.equal(
      committed.outcome().visibleProjection().state,
      "candidateVisibleAfterCommit",
    );
    assert.equal(committed.outcome().visibleProjection().href, "/users/u1");
  } finally {
    signals.free();
    await cleanup();
  }
});
