import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

async function withPhaseSevenRouterFixture(run) {
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const pendingLoads = new Map();
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const { resourceParamIdentity, resourceParams } = await importProductModule("resource/facade.js");
    const authSource = signals.router.host.string("auth");
    const authRequired = signals.router.prerequisite("auth-required", {
      consumes: [authSource],
      evaluate: ({ consume, allow, redirect }) => (
        consume(authSource) === "signedIn"
          ? allow({ reason: "authenticated" })
          : redirect({ href: "/login", reason: "signInRequired" })
      ),
    });
    const createDetailFamily = (kind) => signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ userId }) => resourceParamIdentity({ userId }, userId),
      load: ({ userId }) => new Promise((resolve) => {
        pendingLoads.set(`${kind}:${userId}`, resolve);
      }),
    });
    const detailFamily = createDetailFamily("detail");
    const hoverFamily = createDetailFamily("hoverCard");
    const focusFamily = createDetailFamily("focusPanel");
    const viewportFamily = createDetailFamily("viewportStats");
    const routes = signals.router.define({
      home: signals.router.route("/"),
      about: signals.router.route("/about"),
      login: signals.router.route("/login"),
      private: signals.router.route("/private", {
        admission: [authRequired],
      }),
      users: {
        detail: signals.router.route("/users/:userId", {
          resources: {
            detail: signals.router.resourceLine(detailFamily, {
              params: ({ params }) => ({ userId: params.userId }),
              prefetch: "hover",
            }),
          },
        }),
      },
      warm: signals.router.route("/warm/:userId", {
        resources: {
          hoverCard: signals.router.resourceLine(hoverFamily, {
            params: ({ params }) => ({ userId: params.userId }),
            prefetch: "hover",
          }),
          focusPanel: signals.router.resourceLine(focusFamily, {
            params: ({ params }) => ({ userId: params.userId }),
            prefetch: "focus",
          }),
          viewportStats: signals.router.resourceLine(viewportFamily, {
            params: ({ params }) => ({ userId: params.userId }),
            prefetch: "viewport",
          }),
        },
      }),
      viewportOnly: signals.router.route("/viewport-only/:userId", {
        resources: {
          viewportStats: signals.router.resourceLine(viewportFamily, {
            params: ({ params }) => ({ userId: params.userId }),
            prefetch: "viewport",
          }),
        },
      }),
    });
    await run({
      signals,
      routes,
      settleLoad(kind, userId, value) {
        if (arguments.length === 2) {
          value = userId;
          userId = kind;
          kind = "detail";
        }
        const resolve = pendingLoads.get(`${kind}:${userId}`);
        if (resolve === undefined) {
          throw new Error(`missing pending transition resource load for "${kind}:${userId}"`);
        }
        pendingLoads.delete(`${kind}:${userId}`);
        resolve(value);
      },
      hasPendingLoad(kind, userId) {
        return pendingLoads.has(`${kind}:${userId}`);
      },
    });
  } finally {
    await cleanup();
  }
}

function flushTasks() {
  return new Promise((resolve) => {
    setImmediate(resolve);
  });
}

export {
  flushTasks,
  withPhaseSevenRouterFixture,
};
