import { createSignals, localTruthSchema } from "worth-signals-wasm";
import type {
  LocalTruthMergePreviewOutcome,
  LocalTruthMergeReview,
  LocalTruthMutationRequest,
  LocalTruthOutcome,
} from "worth-signals-wasm";

import type {
  GearBranchRole,
  GearConflictChoice,
  GearHistorySelection,
  GearScenarioView,
} from "./gear_scenario_view.ts";
import { buildGearHistory } from "./gear_history.ts";
import {
  gearAspectMap,
  initialGearTruth,
  type GearDesignAspect,
  type GearTruth,
} from "./gear_truth.ts";

export interface GearConflictSelection {
  conflictId: string;
  choice: GearConflictChoice;
}

const gearAspectIds = Object.keys(gearAspectMap) as Array<keyof GearTruth>;

const schema = localTruthSchema<GearTruth>({
  id: "worth-signals-demo.gear",
  aspects: gearAspectIds.map((field) => ({
    id: field,
    field,
    valueType: ["material", "label"].includes(field) ? "string" : "number",
    equivalence: { kind: "exact" },
    costClass: "constant",
  })),
});

export async function createGearScenario() {
  const signals = await createSignals();
  const gearInput = signals.input(initialGearTruth, {
    debugName: "demo6.gear",
    producesAspects: Object.values(gearAspectMap),
  });
  const truth = signals.localTruth({
    authorityId: "worth-signals-demo.gear",
    schema,
    initialEntities: { gear: initialGearTruth },
    bindings: [{ entityId: "gear", input: gearInput, aspectMap: gearAspectMap }],
  });
  const authorityRoot = required(await truth.branch());
  const main = required(await truth.forkBranch({
    parentBranchId: authorityRoot.id,
    expectedParentBasis: authorityRoot.basis,
    name: "Main",
  }));
  let activeDesignBranchId: string | null = null;
  let comparisonDesignBranchId: string | null = null;
  let activeReview: LocalTruthMergeReview | null = null;
  let historySelection: GearHistorySelection | null = null;
  let phase: GearScenarioView["phase"] = "ready";
  let headline = "Fork Main. Both branches stay writable — no locks, no read-only copy.";
  let branchSequence = 0;
  let requestSequence = 0;
  let activeProjection = { branchName: "Main", truthBranchId: main.id };

  return Object.freeze({
    readView: buildScenarioView,
    async forkDesignBranch() {
      if (activeDesignBranchId) return buildScenarioView();
      if (historySelection) {
        throw new Error("Select a live history head before forking a new branch.");
      }
      const currentMain = required(await truth.branch(main.id));
      branchSequence += 1;
      const designBranch = required(await truth.forkBranch({
        parentBranchId: main.id,
        expectedParentBasis: currentMain.basis,
        name: `Design branch ${branchSequence}`,
      }));
      activeDesignBranchId = designBranch.id;
      comparisonDesignBranchId = designBranch.id;
      activeReview = null;
      historySelection = null;
      phase = "editing";
      headline = "Move anything on either side. Every commit names the exact aspects it changed.";
      activeProjection = { branchName: designBranch.name, truthBranchId: designBranch.id };
      return buildScenarioView();
    },
    async commitBranchPatch(
      role: GearBranchRole,
      patch: Partial<Pick<GearTruth, GearDesignAspect>>,
    ) {
      if (historySelection) {
        throw new Error("Select a live history head before committing new aspect changes.");
      }
      if (!activeDesignBranchId || phase !== "editing") {
        throw new Error("Fork two writable branches before committing aspect changes.");
      }
      const branchId = role === "main" ? main.id : activeDesignBranchId;
      const inspection = await truth.inspect();
      const committedGear = inspection.values[branchId].gear as GearTruth;
      const operations = designOperations(committedGear, patch);
      if (operations.length === 0) return buildScenarioView();
      requestSequence += 1;
      await commitOperations(
        branchId,
        `demo6-${role}-${requestSequence}`,
        operations,
      );
      historySelection = null;
      const aspectNames = operations.map(({ aspectId }) => aspectId).join(", ");
      headline = `${role === "main" ? "Main" : "Design"} committed ${aspectNames}.`;
      return buildScenarioView();
    },
    async mergeBranches() {
      if (historySelection) {
        throw new Error("Select a live history head before merging branches.");
      }
      if (!activeDesignBranchId || phase !== "editing") return buildScenarioView();
      historySelection = null;
      activeReview = await previewDesignBranchMerge(activeDesignBranchId);
      if (activeReview.conflicts.length > 0) {
        const count = activeReview.conflicts.length;
        phase = "review";
        headline = `Both branches moved ${count === 1 ? "the same aspect" : `${count} of the same aspects`}. Pick a winner — everything else merges itself.`;
        return buildScenarioView();
      }
      return commitActiveReview([]);
    },
    async resolveMerge(selections: readonly GearConflictSelection[]) {
      if (!activeReview || phase !== "review") {
        throw new Error("No aspect-level merge review is active.");
      }
      const runtimeSelections = activeReview.conflicts.map((conflict) => {
        const selection = selections.find(({ conflictId }) => conflictId === conflict.id);
        if (!selection) throw new Error(`Conflict ${conflict.id} has no selected branch.`);
        const alternativeChoice = selection.choice === "design" ? "source" : "target";
        const alternative = conflict.alternatives.find(({ choice }) => choice === alternativeChoice);
        if (!alternative) throw new Error(`${alternativeChoice} alternative is unavailable.`);
        return {
          reviewId: activeReview!.id,
          conflictId: conflict.id,
          alternativeId: alternative.id,
        };
      });
      return commitActiveReview(runtimeSelections);
    },
    async selectHistoryCommit(branchId: string, commitId: string) {
      const allowedBranchIds = new Set([main.id, comparisonDesignBranchId].filter(Boolean));
      if (!allowedBranchIds.has(branchId)) {
        throw new Error(`Branch ${branchId} is not part of the visible gear history.`);
      }
      const branch = required(await truth.branch(branchId));
      if (branch.headCommitId === commitId) {
        historySelection = null;
        return buildScenarioView();
      }
      const snapshot = required(await truth.historicalSnapshot({ branchId, commitId }));
      historySelection = {
        commitId,
        branchId,
        lane: branchId === comparisonDesignBranchId ? "design" : "main",
        snapshotId: snapshot.snapshotId,
        values: cloneGearTruth(snapshot.values.gear as GearTruth),
        visitedCommits: snapshot.counters.visitedCommits,
      };
      return buildScenarioView();
    },
    async terminate() {
      await truth.terminate();
      await signals.terminate();
    },
  });

  async function previewDesignBranchMerge(sourceBranchId: string) {
    const source = required(await truth.branch(sourceBranchId));
    const target = required(await truth.branch(main.id));
    return requiredReview(await truth.previewMerge({
      sourceBranchId,
      targetBranchId: main.id,
      expectedSourceBasis: source.basis,
      expectedTargetBasis: target.basis,
    }));
  }

  async function commitActiveReview(selections: LocalTruthMutationSelection[]) {
    if (!activeReview) throw new Error("No merge review is active.");
    requestSequence += 1;
    const merged = required(await truth.resolveMerge({
      requestId: `demo6-merge-${requestSequence}`,
      reviewId: activeReview.id,
      selections,
    }));
    activeDesignBranchId = null;
    activeReview = null;
    historySelection = null;
    phase = "merged";
    headline = mergedHeadline(merged.commit.operations.length, selections.length);
    activeProjection = { branchName: "Main", truthBranchId: main.id };
    return buildScenarioView();
  }

  async function commitOperations(
    branchId: string,
    requestId: string,
    operations: LocalTruthMutationRequest["operations"],
  ) {
    const branch = required(await truth.branch(branchId));
    return required(await truth.commit({
      requestId,
      branchId,
      expectedBasis: branch.basis,
      operations,
    }));
  }

  /** Commit projections settle asynchronously; wait for the current binding. */
  async function awaitProjectionBinding(truthBranchId: string) {
    for (let attempt = 0; attempt < 40; attempt += 1) {
      const receipt = await truth.derivation(truthBranchId);
      if (receipt && "artifactFamily" in receipt && receipt.binding) return receipt.binding;
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
    throw new Error("local truth Signal projection binding is unavailable");
  }

  async function readSignalProjection(): Promise<GearScenarioView["signalProjection"]> {
    try {
      const binding = await awaitProjectionBinding(activeProjection.truthBranchId);
      return {
        branchName: activeProjection.branchName,
        signalBranchId: Number(binding.signalBranchId),
        basisDigest: binding.signalBasisDigest,
      };
    } catch {
      return null;
    }
  }

  async function buildScenarioView(): Promise<GearScenarioView> {
    const inspection = await truth.inspect();
    const designBranchId = comparisonDesignBranchId ?? main.id;
    const mainBranch = requiredInspectionBranch(inspection.branches, main.id);
    const designBranch = comparisonDesignBranchId
      ? requiredInspectionBranch(inspection.branches, comparisonDesignBranchId)
      : null;
    const mainHistory = required(await truth.history(main.id));
    const designHistory = designBranch ? required(await truth.history(designBranch.id)) : null;
    const liveMainValues = inspection.values[main.id].gear as GearTruth;
    const liveDesignValues = inspection.values[designBranchId].gear as GearTruth;
    const displayedMain = historySelection?.lane === "main"
      ? historySelection.values
      : liveMainValues;
    const displayedDesign = historySelection?.lane === "design"
      ? historySelection.values
      : liveDesignValues;
    return {
      phase,
      headline: historySelection
        ? `Sealed snapshot ${historySelection.commitId.slice(0, 22)}. The live branches haven't moved.`
        : headline,
      main: cloneGearTruth(displayedMain),
      design: cloneGearTruth(displayedDesign),
      designBranchName: designBranch?.name ?? "Design branch",
      activeDesignBranchId,
      conflicts: activeReview?.conflicts.map((conflict) => ({
        id: conflict.id,
        aspectId: conflict.aspectId,
        mainValue: requiredConflictAlternative(conflict, "target").value,
        designValue: requiredConflictAlternative(conflict, "source").value,
      })) ?? [],
      history: buildGearHistory({ mainBranch, mainHistory, designBranch, designHistory }),
      historySelection,
      signalProjection: await readSignalProjection(),
    };
  }
}

function mergedHeadline(appliedCount: number, resolvedCount: number) {
  if (appliedCount === 0) return "Merged: nothing to apply — the branches never diverged.";
  const changes = `${appliedCount} aspect change${appliedCount === 1 ? "" : "s"} landed in Main in one commit`;
  if (resolvedCount === 0) {
    return `Merged: ${changes}. No conflicts — the branches never moved the same aspect.`;
  }
  return `Merged: ${changes} — ${resolvedCount} decided by you.`;
}

function requiredConflictAlternative(
  conflict: LocalTruthMergeReview["conflicts"][number],
  choice: "source" | "target",
) {
  const alternative = conflict.alternatives.find((candidate) => candidate.choice === choice);
  if (!alternative) throw new Error(`${choice} alternative is unavailable for ${conflict.id}.`);
  return alternative;
}

type LocalTruthMutationSelection = {
  reviewId: string;
  conflictId: string;
  alternativeId: string;
};

function designOperations(
  committedGear: GearTruth,
  patch: Partial<Pick<GearTruth, GearDesignAspect>>,
): LocalTruthMutationRequest["operations"] {
  return (Object.entries(patch) as Array<[GearDesignAspect, number]>)
    .filter(([, value]) => Number.isFinite(value))
    .map(([aspectId, value]) => ({
      entityId: "gear",
      aspectId,
      value: aspectId === "teeth" ? Math.round(value) : value,
    }))
    .filter(({ aspectId, value }) => committedGear[aspectId] !== value);
}

function cloneGearTruth(values: GearTruth): GearTruth {
  return { ...values };
}

function requiredInspectionBranch<T extends { id: string }>(branches: readonly T[], branchId: string): T {
  const branch = branches.find(({ id }) => id === branchId);
  if (!branch) throw new Error(`Branch ${branchId} is unavailable in Local Truth inspection.`);
  return branch;
}

function required<T>(outcome: LocalTruthOutcome<T>): T {
  if (outcome.posture !== "success" && outcome.posture !== "advisory") {
    throw new Error(`local truth operation failed: ${outcome.code}`);
  }
  return outcome.value;
}

function requiredReview(outcome: LocalTruthMergePreviewOutcome): LocalTruthMergeReview {
  if (outcome.posture === "reviewRequired") return outcome.review;
  return required(outcome);
}
