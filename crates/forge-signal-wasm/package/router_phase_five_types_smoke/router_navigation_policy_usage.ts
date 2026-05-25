import type {
  CallableSignals,
  NavigationCommitPolicy,
  NavigationIntentKind,
  NavigationRedirectPolicy,
  RouteNavigationExecutionContract,
  RouteNavigationFreshnessDiagnostics,
  RouteNavigationProjectionPolicy,
  RouteNavigationTransitionPolicy,
} from "../index.js";

declare const signals: CallableSignals;

const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/users/:userId", {
    search: {
      tab: signals.router.search.optional.string(),
    },
  }),
});

const restorePlan = routes.detail
  .intent(
    {
      params: { userId: "u1" },
      search: { tab: "activity" },
    },
    {
      kind: "restoreBack",
      policy: {
        continuity: "preserve-visible-until-explicit-refresh",
        projectionRefresh: "explicit",
        artifactPolicy: "diagnostics",
        commit: "speculativeBranch",
        redirect: "surfaceRedirect",
        deployment: "workerFirst",
      },
    },
  )
  .compile();

const restoreKind: NavigationIntentKind = restorePlan.kind;
const restoreCommit: NavigationCommitPolicy = restorePlan.policy().commit;
const restoreRedirect: NavigationRedirectPolicy = restorePlan.policy().redirect;
const restoreTransitionPolicy: RouteNavigationTransitionPolicy = restorePlan.policy();
const restoreExecutionContract: RouteNavigationExecutionContract =
  restorePlan.execution();
const restoreFreshness: RouteNavigationFreshnessDiagnostics = restorePlan.freshness();
const restoreProjectionPolicy: RouteNavigationProjectionPolicy =
  restorePlan.projectionPolicy();
const restoreExplainCommit: NavigationCommitPolicy =
  restorePlan.explain().transitionPolicy.commit;
const restoreHistoryEffect: "pushstate" | "replacestate" | "none" =
  restorePlan.policy().historyEffect;
const restoreNavigationFamily:
  | "direct-route"
  | "canonicalization"
  | "soft-refresh"
  | "same-route-mutation"
  | "restore-navigation" = restorePlan.policy().navigationFamily;
const restoreRouteTruthEffect:
  | "advance-admitted-route-truth"
  | "canonicalize-admitted-route-truth"
  | "re-admit-current-route-truth"
  | "re-admit-current-route-with-mutation"
  | "restore-admitted-route-truth" = restorePlan.execution().routeTruthEffect;
const restoreVisibleProjectionEffect:
  | "refresh-visible-projection-immediately"
  | "refresh-visible-projection-after-admission"
  | "preserve-visible-projection-until-explicit-refresh" =
  restorePlan.execution().visibleProjectionEffect;
const restoreRefreshAttribution:
  | "refreshes-visible-projection-immediately"
  | "refreshes-visible-projection-after-admission"
  | "requires-explicit-visible-refresh" =
  restorePlan.freshness().refreshAttribution;
const restoreContinuityAttribution:
  | "no-visible-continuity-preservation"
  | "preserve-visible-while-pending"
  | "preserve-visible-until-explicit-refresh" =
  restorePlan.freshness().continuityAttribution;
const restoreStaleReason:
  | null
  | "waiting-for-admission-refresh"
  | "waiting-for-explicit-refresh" = restorePlan.freshness().staleVisibilityReason;

const breadcrumbPlan = routes.home.to().plan({
  commit: "directCommit",
  redirect: "followRedirect",
  projectionRefresh: "after-admission",
  continuity: "preserve-visible-while-pending",
});

const breadcrumbKind: NavigationIntentKind = breadcrumbPlan.kind;
const breadcrumbTransitionPolicy: RouteNavigationTransitionPolicy = breadcrumbPlan.policy();
const breadcrumbFreshness: RouteNavigationFreshnessDiagnostics =
  breadcrumbPlan.freshness();
const breadcrumbProjectionPolicy: RouteNavigationProjectionPolicy =
  breadcrumbPlan.projectionPolicy();
const pushIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "push" }).compile().kind;
const replaceIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "replace" }).compile().kind;
const canonicalizeIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "canonicalize" }).compile().kind;
const softRefreshIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "softRefresh" }).compile().kind;
const sameRouteMutationIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "sameRouteMutation" }).compile().kind;
const breadcrumbReturnIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "breadcrumbReturn" }).compile().kind;
const restoreBackIntentKind: NavigationIntentKind =
  routes.home.intent(undefined, { kind: "restoreBack" }).compile().kind;

void restoreKind;
void restoreCommit;
void restoreRedirect;
void restoreTransitionPolicy;
void restoreExecutionContract;
void restoreFreshness;
void restoreProjectionPolicy;
void restoreExplainCommit;
void restoreHistoryEffect;
void restoreNavigationFamily;
void restoreRouteTruthEffect;
void restoreVisibleProjectionEffect;
void restoreRefreshAttribution;
void restoreContinuityAttribution;
void restoreStaleReason;
void breadcrumbKind;
void breadcrumbTransitionPolicy;
void breadcrumbFreshness;
void breadcrumbProjectionPolicy;
void pushIntentKind;
void replaceIntentKind;
void canonicalizeIntentKind;
void softRefreshIntentKind;
void sameRouteMutationIntentKind;
void breadcrumbReturnIntentKind;
void restoreBackIntentKind;
