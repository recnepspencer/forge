import type {
  CallableSignals,
  RouteHistoryReplayResult,
  RouteHistoryRestoreResult,
  RouteReplayHistoryFacade,
  RouteRestoredBreadcrumbs,
  RouteRestoreBoundary,
  RouteRestoreHistoryFacade,
  RuntimeSnapshotEnvelopeArtifact,
} from "../index.js";

declare const signals: CallableSignals;
declare const snapshot: RuntimeSnapshotEnvelopeArtifact;

const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/detail", {
    breadcrumb: signals.router.breadcrumb({
      id: "detail",
      label: "Detail",
      parent: signals.router.breadcrumbParent({
        carry: true,
        fallback: signals.router.breadcrumbEntry({
          id: "home-fallback",
          label: "Home",
          target: "/",
        }),
      }),
    }),
  }),
});

const restoreBoundary: RouteRestoreBoundary = signals.router.restoreBoundary(snapshot);
const historyFacade: RouteRestoreHistoryFacade = signals.history();
const replayHistoryFacade: RouteReplayHistoryFacade = signals.history();
const story = signals.router.browserHistory.story();

const ingress = signals.router.browserHistory.load("/", {
  routeIdentity: "homeRoute",
  restoreBoundary,
});
const writeback = signals.router.browserHistory.writeback.push("/detail", {
  routeIdentity: "detailRoute",
  restoreBoundary,
});
const restoredBreadcrumbs: RouteRestoredBreadcrumbs = signals.router.restoreBreadcrumbs(
  story.breadcrumbTrail().entries,
);

ingress.restoreBoundary;
writeback.restoreBoundary;
restoredBreadcrumbs.entries;
restoredBreadcrumbs.entries[0].provenance();

const currentEntry = story.current();
const currentBoundary: RouteRestoreBoundary | null =
  currentEntry?.restoreBoundary() ?? null;
const currentRestoreResult:
  | RouteHistoryRestoreResult
  | Promise<RouteHistoryRestoreResult>
  | null = currentEntry ? currentEntry.restore(historyFacade) : null;
const backRestoreResult:
  | RouteHistoryRestoreResult
  | Promise<RouteHistoryRestoreResult> = story.backProvenance().restore(historyFacade);
const breadcrumbRestoreResult:
  | RouteHistoryRestoreResult
  | Promise<RouteHistoryRestoreResult> = story.breadcrumbTrail().entries[0].restore(
  historyFacade,
);
const currentReplayResult:
  | RouteHistoryReplayResult
  | Promise<RouteHistoryReplayResult>
  | null = currentEntry ? currentEntry.replay(replayHistoryFacade) : null;
const backReplayResult:
  | RouteHistoryReplayResult
  | Promise<RouteHistoryReplayResult> = story.backProvenance().replay(replayHistoryFacade);
const breadcrumbReplayResult:
  | RouteHistoryReplayResult
  | Promise<RouteHistoryReplayResult> = story.breadcrumbTrail().entries[0].replay(
  replayHistoryFacade,
);
const breadcrumbProvenance = story.breadcrumbTrail().entries[0].provenance();
const inspection = story.inspection();
const inspectionSummary = inspection.summary();
const inspectionBreadcrumbProvenance = inspection.breadcrumbProvenance();
const currentOutletComposition = currentEntry?.outletComposition() ?? null;
const inspectionCurrentOutletComposition = inspection.currentOutletComposition();
const inspectionBackOutletComposition = inspection.backOutletComposition();

void currentBoundary;
void currentRestoreResult;
void backRestoreResult;
void breadcrumbRestoreResult;
void currentReplayResult;
void backReplayResult;
void breadcrumbReplayResult;
void breadcrumbProvenance;
void inspection;
void inspectionSummary;
void inspectionBreadcrumbProvenance;
void currentOutletComposition;
void inspectionCurrentOutletComposition;
void inspectionBackOutletComposition;
