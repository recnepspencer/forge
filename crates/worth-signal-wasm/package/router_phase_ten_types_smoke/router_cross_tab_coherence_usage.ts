import type {
  CallableSignals,
} from "../index.js";

declare const signals: CallableSignals;

const routes = signals.router.define({
  settings: signals.router.route("/settings"),
});

const coherence = signals.router.browserHistory.coherence.crossTab("workspace-main", {
  sourceTabId: "tab-b",
});

const ingress = signals.router.browserHistory.external("/settings", {
  routeIdentity: "settings",
  coherence,
});

const reportPromise = routes.admitBrowserHistoryIngress(ingress);

void coherence;
void reportPromise;
