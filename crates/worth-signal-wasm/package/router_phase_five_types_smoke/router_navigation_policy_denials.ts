import type { CallableSignals } from "../index.js";

declare const signals: CallableSignals;

const routes = signals.router.define({
  home: signals.router.route("/"),
});

// @ts-expect-error invalid navigation intent kind must stay denied
routes.home.intent(undefined, { kind: "teleportBack" });

// @ts-expect-error invalid commit posture must stay denied
routes.home.to().plan({ commit: "branchish" });

// @ts-expect-error invalid redirect posture must stay denied
routes.home.to().plan({ redirect: "guessRedirect" });

// @ts-expect-error invalid projection freshness posture must stay denied
routes.home.to().plan({ projectionRefresh: "later" });

// @ts-expect-error invalid continuity posture must stay denied
routes.home.to().plan({ continuity: "keep-it-vibes" });
