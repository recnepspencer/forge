import type {
  CallableSignals,
} from "../index.js";

declare const signals: CallableSignals;

const routes = signals.router.define({
  detail: signals.router.route("/detail"),
});
const story = signals.router.browserHistory.story();
const hydration = signals.router.hydration.server("/detail", {
  serverRouteIdentity: "detail",
});
const hydrationReportPromise = routes.admitHydrationHandoff(hydration);

void hydrationReportPromise.then((report) => {
  const auditability = story.auditability(report);
  return auditability.summary().currentVisibleRouteSource;
});
