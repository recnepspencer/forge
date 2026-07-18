import { resourcePolicyProfiles } from "worth-signals-wasm";

export type PlantRole = "operator" | "supervisor" | "qa";

export type SopRevision = "B" | "C";

export interface SessionFacts {
  role: PlantRole;
  trainedRev: SopRevision;
  effectiveRev: SopRevision;
  underDeviation: boolean;
}

export interface RouterSectionModel {
  readonly routes: any;
  readonly initialTarget: string;
  readonly routeOptions: ReadonlyArray<{ path: string; label: string }>;
}

export const roleLabels: Record<PlantRole, string> = {
  operator: "Operator",
  supervisor: "Line supervisor",
  qa: "QA specialist",
};

/** Training on SOP-042 by role; the operator trained before the rev bump. */
export const ROLE_TRAINED_REV: Record<PlantRole, SopRevision> = {
  operator: "B",
  supervisor: "C",
  qa: "B",
};

export const ROUTE_OPTIONS = [
  { path: "/line/overview", label: "Line overview" },
  { path: "/batches/B-2214/record", label: "Batch record" },
  { path: "/batches/B-2214/steps/4", label: "Execute step 4" },
  { path: "/batches/B-2214/release", label: "Quality release" },
] as const;

export interface ReplayPersona {
  id: string;
  label: string;
  facts: Omit<SessionFacts, "underDeviation">;
}

export const REPLAY_PERSONAS: ReplayPersona[] = [
  {
    id: "operator-revB",
    label: "Operator · trained rev B",
    facts: { role: "operator", trainedRev: "B", effectiveRev: "C" },
  },
  {
    id: "operator-revC",
    label: "Operator · retrained rev C",
    facts: { role: "operator", trainedRev: "C", effectiveRev: "C" },
  },
  {
    id: "qa",
    label: "QA specialist",
    facts: { role: "qa", trainedRev: "B", effectiveRev: "C" },
  },
];

const pageData = {
  overview: {
    line: "Line L-03 · Infusion pump assembly",
    shift: "Day shift · 2 of 3 stations active",
    wip: [
      { batch: "B-2213", status: "Quality release pending" },
      { batch: "B-2214", status: "In process · step 4 of 7" },
    ],
  },
  batchRecord: {
    batch: "B-2214",
    product: "IP-400 infusion pump",
    status: "In process",
    steps: [
      { step: "1 · Line clearance", status: "complete" },
      { step: "2 · Sub-assembly install", status: "complete" },
      { step: "3 · Firmware load", status: "complete" },
      { step: "4 · Torque verification", status: "pending" },
      { step: "5 · Leak test", status: "not started" },
    ],
  },
  stepFour: {
    step: "Step 4 · Torque verification",
    sop: "SOP-042 · Torque verification procedure",
    spec: "0.9 – 1.1 N·m on fasteners F1–F4",
    instrument: "Calibrated driver TD-118 · cal due 2026-09-02",
  },
  release: {
    batch: "B-2214",
    checklist: [
      { item: "All steps executed", state: "blocked · step 4 pending" },
      { item: "Deviations reviewed", state: "0 open" },
      { item: "Device history record complete", state: "pending" },
    ],
  },
} as const;

export type OverviewPage = typeof pageData.overview;
export type BatchRecordPage = typeof pageData.batchRecord;
export type StepFourPage = typeof pageData.stepFour;
export type ReleasePage = typeof pageData.release;

function cloneJson<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

async function loadJson<T>(value: T): Promise<T> {
  await new Promise((resolve) => window.setTimeout(resolve, 260));
  return cloneJson(value);
}

async function policyDelay(): Promise<void> {
  await new Promise((resolve) => window.setTimeout(resolve, 220));
}

export function buildRouterSectionModel(signals: any): RouterSectionModel {
  const api = signals.api({
    baseUrl: "/mes-demo-api",
    effects: signals.resource.effects.branchNative(),
  });
  const defaultPolicy = resourcePolicyProfiles.stable();

  const overviewFamily = api.url("/line/overview").detail({
    policy: defaultPolicy,
    load: async () => loadJson(pageData.overview),
  });
  const batchRecordFamily = api.url("/batches/:batchId/record").detail({
    policy: defaultPolicy,
    load: async () => loadJson(pageData.batchRecord),
  });
  const stepFamily = api.url("/batches/:batchId/steps/:stepId").detail({
    policy: defaultPolicy,
    load: async () => loadJson(pageData.stepFour),
  });
  const releaseFamily = api.url("/batches/:batchId/release").detail({
    policy: defaultPolicy,
    load: async () => loadJson(pageData.release),
  });

  const stepExecution = signals.router.prerequisite(
    "stepExecution",
    async ({ facts, allow, forbidden }: any) => {
      await policyDelay();
      if (facts.role !== "operator" && facts.role !== "supervisor") {
        return forbidden({
          reason: "executionRequiresProductionRole",
          detail: "Batch step execution requires a production role.",
        });
      }
      if (facts.trainedRev === facts.effectiveRev) {
        return allow({ reason: "trainingCurrent" });
      }
      if (facts.underDeviation) {
        return allow({ reason: "deviationRecorded" });
      }
      return forbidden({
        reason: "trainingSupersededByRevision",
        detail: `Trained on SOP-042 rev ${facts.trainedRev}; effective revision is now ${facts.effectiveRev}.`,
      });
    },
  );

  const qaRelease = signals.router.prerequisite(
    "qaRelease",
    async ({ facts, allow, forbidden }: any) => {
      await policyDelay();
      return facts.role === "qa"
        ? allow({ reason: "qualityRolePresent" })
        : forbidden({
            reason: "releaseRequiresQuality",
            detail: "Quality release is restricted to the quality unit.",
          });
    },
  );

  const routes = signals.router.define({
    overview: signals.router.route("/line/overview", {
      resources: {
        page: signals.router.resourceLine(overviewFamily, {
          params: () => ({}),
          prefetch: "intent",
        }),
      },
    }),
    batchRecord: signals.router.route("/batches/:batchId/record", {
      resources: {
        page: signals.router.resourceLine(batchRecordFamily, {
          params: ({ params }: any) => ({ batchId: params.batchId }),
          prefetch: "intent",
        }),
      },
    }),
    stepExecute: signals.router.route("/batches/:batchId/steps/:stepId", {
      admission: [stepExecution],
      resources: {
        page: signals.router.resourceLine(stepFamily, {
          params: ({ params }: any) => ({ batchId: params.batchId, stepId: params.stepId }),
          prefetch: "intent",
        }),
      },
    }),
    release: signals.router.route("/batches/:batchId/release", {
      admission: [qaRelease],
      resources: {
        page: signals.router.resourceLine(releaseFamily, {
          params: ({ params }: any) => ({ batchId: params.batchId }),
          prefetch: "intent",
        }),
      },
    }),
  });

  return {
    routes,
    initialTarget: ROUTE_OPTIONS[2].path,
    routeOptions: ROUTE_OPTIONS,
  };
}

export interface OutcomeView {
  kind: string;
  tone: "admitted" | "deviation" | "redirected" | "denied";
  label: string;
  reason: string | null;
  detail: string | null;
}

export function describeOutcome(report: any, underDeviation: boolean): OutcomeView {
  const outcome = report?.outcome?.() ?? null;
  const kind = outcome?.kind ?? "unknown";
  const artifact = (() => {
    try {
      return outcome?.artifact?.() ?? null;
    } catch {
      return null;
    }
  })();
  const reason = artifact?.reason ?? null;
  const detail = artifact?.detail ?? null;

  if (kind === "admitted") {
    if (underDeviation && reason === null) {
      return { kind, tone: "deviation", label: "admitted · deviation", reason: "deviationRecorded", detail };
    }
    return {
      kind,
      tone: reason === "deviationRecorded" ? "deviation" : "admitted",
      label: reason === "deviationRecorded" ? "admitted · deviation" : "admitted",
      reason,
      detail,
    };
  }
  if (kind === "redirect") {
    return { kind, tone: "redirected", label: "redirected", reason, detail };
  }
  return { kind, tone: "denied", label: "denied", reason, detail };
}
