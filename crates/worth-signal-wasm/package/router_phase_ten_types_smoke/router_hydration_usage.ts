import type {
  CallableSignals,
  RouterHydrationHandoff,
} from "../index.js";

declare const signals: CallableSignals;

const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/detail"),
});

const hydration: RouterHydrationHandoff = signals.router.hydration.server("/detail", {
  serverRouteIdentity: "detail",
  serverHref: "/detail",
});

const hydrationReportPromise = routes.admitHydrationHandoff(hydration);

void hydration;
void hydrationReportPromise;
