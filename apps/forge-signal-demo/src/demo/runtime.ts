import {
  createSignalRuntime,
  define,
  expr,
  policy,
  type MergePlanReport,
  type MergeResultReport,
  type ReplayFrameSummary,
  type RunSummary,
  type SignalRuntime,
  type WhySummary,
} from "@forge/signal";

export type { MergePlanReport, MergeResultReport } from "@forge/signal";

export type BranchId = number;

export type GearParams = {
  teeth: number;
  outerRadius: number;
  innerRadius: number;
  rotation: number;
};

export type GearSummary = GearParams & {
  rimThickness: number;
  toothSpan: number;
};

export type BranchView = {
  id: BranchId;
  name: string;
  params: GearParams;
  summary: GearSummary;
  replay: ReplayFrameSummary[];
  why: WhySummary;
};

type RuntimeBundle = {
  runtime: SignalRuntime;
};

const INITIAL_PARAMS: GearParams = {
  teeth: 14,
  outerRadius: 114,
  innerRadius: 90,
  rotation: 0,
};

export async function createDemoRuntime(): Promise<RuntimeBundle> {
  const runtime = await createSignalRuntime();

  runtime.setRuntimePolicy(policy.preset("webDevelopment"));

  const teeth = runtime.defineSource(
    define.source<number>("teeth").initial(INITIAL_PARAMS.teeth),
  );
  const outerRadius = runtime.defineSource(
    define.source<number>("outerRadius").initial(INITIAL_PARAMS.outerRadius),
  );
  const innerRadius = runtime.defineSource(
    define.source<number>("innerRadius").initial(INITIAL_PARAMS.innerRadius),
  );
  const rotation = runtime.defineSource(
    define.source<number>("rotation").initial(INITIAL_PARAMS.rotation),
  );

  runtime.defineRecipe(
    define
      .recipe<number>("rimThickness")
      .reads(outerRadius, innerRadius)
      .expr(
        expr.subtract(
          expr.read<number>("outerRadius"),
          expr.read<number>("innerRadius"),
        ),
      ),
  );

  runtime.defineRecipe(
    define
      .recipe<number>("toothSpan")
      .reads(outerRadius, teeth)
      .expr(
        expr.divide(
          expr.read<number>("outerRadius"),
          expr.read<number>("teeth"),
        ),
      ),
  );

  runtime.defineRecipe(
    define
      .recipe<GearSummary>("gearSummary")
      .reads(teeth, outerRadius, innerRadius, rotation, "rimThickness", "toothSpan")
      .expr(
        expr.object<GearSummary>({
          teeth: expr.read<number>("teeth"),
          outerRadius: expr.read<number>("outerRadius"),
          innerRadius: expr.read<number>("innerRadius"),
          rotation: expr.read<number>("rotation"),
          rimThickness: expr.read<number>("rimThickness"),
          toothSpan: expr.read<number>("toothSpan"),
        }),
      )
      .identityExact(),
  );

  return { runtime };
}

export function displayBranchId(branchId: BranchId): string {
  return String(branchId);
}

export function branchCount(runtime: SignalRuntime): number {
  return runtime.history().branches().length;
}

export function totalGraphNodes(runtime: SignalRuntime): number {
  const definitions = runtime.adapters().exportDefinitions();
  return (
    definitions.sources.length +
    definitions.recipes.length +
    definitions.sourceFamilies.length +
    definitions.recipeFamilies.length
  );
}

export function readCurrentParams(runtime: SignalRuntime): GearParams {
  return {
    teeth: runtime.read<number>("teeth"),
    outerRadius: runtime.read<number>("outerRadius"),
    innerRadius: runtime.read<number>("innerRadius"),
    rotation: runtime.read<number>("rotation"),
  };
}

export function readBranchView(
  runtime: SignalRuntime,
  branchId: BranchId,
): BranchView {
  const history = runtime.history();
  const before = history.currentBranch();
  history.switchBranch(branchId);

  const branch = history.currentBranch();
  const params = readCurrentParams(runtime);
  const summary = runtime.read<GearSummary>("gearSummary");
  const replay = history.replayFor("gearSummary").frames.slice(-8);
  const why = runtime.diagnostics().why("gearSummary");

  history.switchBranch(before.id);

  return {
    id: branch.id,
    name: branch.name,
    params,
    summary,
    replay,
    why,
  };
}

export function updateBranchParam(
  runtime: SignalRuntime,
  branchId: BranchId,
  key: keyof GearParams,
  value: number,
): RunSummary {
  const history = runtime.history();
  const before = history.currentBranch();
  history.switchBranch(branchId);

  const result = runtime.transaction([
    {
      kind: "set",
      id: key,
      value,
    },
  ]);

  history.switchBranch(before.id);
  return result;
}

export function ensureFeatureBranch(runtime: SignalRuntime): BranchId {
  const history = runtime.history();
  const branches = history.branches();
  const existing = branches.find((branch) => branch.name === "what-if");
  if (existing) {
    return existing.id;
  }
  return history.createBranch("what-if").id;
}

export function planMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): MergePlanReport {
  return runtime
    .history()
    .planMergeBranches(sourceBranchId, targetBranchId);
}

export function executeMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): MergeResultReport {
  return runtime.history().mergeBranches(sourceBranchId, targetBranchId);
}
