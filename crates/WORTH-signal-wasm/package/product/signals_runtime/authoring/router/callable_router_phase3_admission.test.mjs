import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-3 route admission resolves declared prerequisites into admitted route truth", async () => {
  await withPhaseThreeRouterFixture(async ({ routes }) => {
    const projectedCandidate = routes.project("/users/user-1?tab=activity");
    assert.ok(projectedCandidate);
    assert.equal("formsAuthority" in projectedCandidate.route(), false);
    assert.equal(projectedCandidate.route().controller("detail").kind, "projectedControllerCapability");
    assert.equal(projectedCandidate.route().graph("detailGraph").kind, "projectedGraphCapability");

    const admissionPlan = projectedCandidate.admission({
      workspaceState: "active",
      auth: "signedIn",
      workspaceReady: true,
      tenantCapability: "granted",
    });
    assert.deepEqual(admissionPlan.prerequisiteNames(), [
      "workspace-active",
      "auth-required",
    ]);
    assert.equal(admissionPlan.provenance().attemptedRouteId, "app.users.detail");
    assert.deepEqual(admissionPlan.provenance().factsKeys, [
      "auth",
      "tenantCapability",
      "workspaceReady",
      "workspaceState",
    ]);
    assert.deepEqual(
      admissionPlan.provenance().consumedSources.map((source) => `${source.family}:${source.name}`),
      [
        "hostCapability:auth",
        "resourceTruth:workspaceReady",
        "graphTruth:tenantCapability",
      ],
    );

    const admittedOutcome = await admissionPlan.resolve();
    assert.equal(admittedOutcome.kind, "admitted");
    assert.equal(admittedOutcome.routeId, "app.users.detail");
    assert.equal(admittedOutcome.href, "/users/user-1?tab=activity");
    assert.equal(admittedOutcome.route().kind, "admittedRouteCapability");
    assert.equal(admittedOutcome.route().controller("detail").kind, "admittedControllerCapability");
    assert.deepEqual(admittedOutcome.route().controller("detail").outputNames(), ["routeLabel"]);
    assert.equal(admittedOutcome.route().graph("detailGraph").kind, "admittedGraphCapability");
    assert.equal(admittedOutcome.route().graph("detailGraph").summary().id, "routeDetailGraph");
    assert.deepEqual(admittedOutcome.route().params, { userId: "user-1" });
    assert.deepEqual(admittedOutcome.route().search, { tab: "activity" });
    assert.equal(admittedOutcome.outlet().descriptor().occupantRouteId, "app.users.detail");
    assert.equal(admittedOutcome.route().formsAuthority()?.surfaceId, "user-detail-form");
    assert.equal(admittedOutcome.diagnostics().formsAuthority?.continuity, "preserve");
    assert.match(admittedOutcome.verification().formsAuthorityDigest, /route-forms-authority/);
    assert.deepEqual(
      admittedOutcome.diagnostics().prerequisiteDecisions.map((decision) => ({
        prerequisite: decision.prerequisite,
        kind: decision.kind,
        reason: decision.reason,
      })),
      [
        {
          prerequisite: "workspace-active",
          kind: "allow",
          reason: "workspaceActive",
        },
        {
          prerequisite: "auth-required",
          kind: "allow",
          reason: "authenticated",
        },
      ],
    );
    assert.deepEqual(
      admittedOutcome.provenance().prerequisiteDecisions[1]?.consumedSources.map((source) => source.name),
      ["auth", "workspaceReady", "tenantCapability"],
    );
    assert.equal(admissionPlan.verification().routeId, "app.users.detail");
    assert.match(admittedOutcome.verification().routeOutcomeDigest, /route-outcome/);
    assert.equal(admittedOutcome.provenance().terminalSource, "admittedWithoutRecovery");
    assert.equal(admittedOutcome.provenance().resolvedRouteId, "app.users.detail");
  });
});

test("phase-3 admission returns explicit redirect, forbidden, unavailable, denied, and not-found outcomes", async () => {
  await withPhaseThreeRouterFixture(async ({ routes }) => {
    const redirectOutcome = await routes.admit("/users/user-1?tab=activity", {
      workspaceState: "active",
      auth: "anonymous",
      workspaceReady: true,
      tenantCapability: "granted",
    });
    assert.equal(redirectOutcome.kind, "redirect");
    assert.equal(redirectOutcome.artifact().href, "/login");
    assert.equal(redirectOutcome.artifact().prerequisite, "auth-required");
    assert.equal(redirectOutcome.provenance().terminalSource, "prerequisiteArtifact");
    assert.equal(redirectOutcome.provenance().terminalArtifact?.kind, "redirect");
    assert.equal(redirectOutcome.provenance().prerequisiteDecisions.at(-1)?.prerequisite, "auth-required");

    const forbiddenOutcome = await routes.admit("/admin", {
      workspaceState: "active",
    });
    assert.equal(forbiddenOutcome.kind, "forbidden");
    assert.equal(forbiddenOutcome.artifact().prerequisite, "admin-role");

    const unavailableOutcome = await routes.admit("/maintenance", {
      workspaceState: "active",
      maintenance: true,
    });
    assert.equal(unavailableOutcome.kind, "unavailable");
    assert.equal(unavailableOutcome.artifact().prerequisite, "maintenance-window");

    const deniedOutcome = await routes.admit("/licensed", {
      workspaceState: "active",
    });
    assert.equal(deniedOutcome.kind, "denied");
    assert.equal(deniedOutcome.artifact().prerequisite, "license-check");

    const noCandidateOutcome = await routes.admit("/missing");
    assert.equal(noCandidateOutcome.kind, "notFound");
    assert.equal(noCandidateOutcome.routeId, null);
    assert.equal(noCandidateOutcome.artifact().prerequisite, null);
    assert.equal(noCandidateOutcome.provenance().terminalSource, "noProjectedCandidate");
    assert.equal(noCandidateOutcome.provenance().attemptedHref, "/missing");
  });
});

test("phase-3 admission short-circuits after the first non-admitted prerequisite artifact", async () => {
  await withPhaseThreeRouterFixture(async ({ routes, visitedPrerequisites }) => {
    visitedPrerequisites.length = 0;
    const outcome = await routes.admit("/redirect-chain", {
      workspaceState: "active",
      auth: "anonymous",
    });
    assert.equal(outcome.kind, "redirect");
    assert.deepEqual(visitedPrerequisites, ["redirect-first"]);
  });
});

test("phase-3 admission recovers stale links through declared nearest-valid fallback", async () => {
  await withPhaseThreeRouterFixture(async ({ routes }) => {
    const outcome = await routes.admit("/projects/project-1", {
      workspaceState: "active",
      projectState: "deleted",
    });
    assert.equal(outcome.kind, "admitted");
    assert.equal(outcome.routeId, "app.projects.index");
    assert.equal(outcome.href, "/projects");
    assert.equal(outcome.recovery()?.recovery, "stale-project-recovery");
    assert.equal(outcome.recovery()?.href, "/projects");
    assert.equal(outcome.diagnostics().recovery?.fromArtifactKind, "notFound");
    assert.equal(outcome.diagnostics().recovery?.fromRouteId, "app.projects.detail");
    assert.equal(outcome.diagnostics().recovery?.fromHref, "/projects/project-1");
    assert.equal(outcome.provenance().terminalSource, "recoveredOutcome");
    assert.equal(outcome.provenance().attemptedRouteId, "app.projects.detail");
    assert.equal(outcome.provenance().resolvedRouteId, "app.projects.index");
    assert.equal(outcome.provenance().recoveryTrail[0]?.toRouteId, "app.projects.index");
    assert.equal(outcome.verification().routeId, "app.projects.index");
    assert.notEqual(
      outcome.verification().admissionPlanDigest,
      routes.project("/projects")?.admission({ workspaceState: "active" }).verification().admissionPlanDigest,
    );
    assert.match(outcome.verification().admissionPlanDigest, /route-admission-plan/);
  });
});

test("phase-3 redirect outcomes dominate recovery and invalid recovery targets fail closed", async () => {
  await withPhaseThreeRouterFixture(async ({ routes, visitedRecoveries }) => {
    visitedRecoveries.length = 0;
    const redirectOutcome = await routes.admit("/users/user-1?tab=activity", {
      workspaceState: "active",
      auth: "anonymous",
      workspaceReady: true,
      tenantCapability: "granted",
    });
    assert.equal(redirectOutcome.kind, "redirect");
    assert.equal(redirectOutcome.recovery(), null);
    assert.deepEqual(visitedRecoveries, []);

    await assert.rejects(
      () => routes.admit("/projects/broken-project", {
        workspaceState: "active",
        projectState: "deleted",
        invalidRecoveryTarget: true,
      }),
      /does not project a declared route candidate/,
    );
  });
});

test("phase-3 declared prerequisite sources fail closed on missing or mistyped admission facts", async () => {
  await withPhaseThreeRouterFixture(async ({ routes }) => {
    await assert.rejects(
      () => routes.admit("/users/user-1?tab=activity", {
        workspaceState: "active",
        workspaceReady: true,
        tenantCapability: "granted",
      }),
      /requires declared source "auth"/,
    );

    await assert.rejects(
      () => routes.admit("/users/user-1?tab=activity", {
        workspaceState: "active",
        auth: "signedIn",
        workspaceReady: "yes",
        tenantCapability: "granted",
      }),
      /source "workspaceReady" must be a boolean/,
    );

    const hiddenBypassOutcome = await routes.admit("/users/user-1?tab=activity", {
      workspaceState: "active",
      auth: "signedIn",
      workspaceReady: true,
      tenantCapability: "granted",
      hiddenBypass: "should-not-be-visible",
    });
    assert.equal(hiddenBypassOutcome.kind, "admitted");
    assert.equal(hiddenBypassOutcome.routeId, "app.users.detail");
    assert.deepEqual(
      hiddenBypassOutcome.provenance().prerequisiteDecisions[1]?.consumedSources.map((source) => source.name),
      ["auth", "workspaceReady", "tenantCapability"],
    );
  });
});

async function withPhaseThreeRouterFixture(run) {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const visitedPrerequisites = [];
    const visitedRecoveries = [];
    const routeLabel = signals.output(() => "detail");
    const detailController = signals.controller({
      outputs: {
        routeLabel,
      },
    });
    const detailGraph = signals.graph("routeDetailGraph", {
      outputs: {
        routeLabel,
      },
    });
    const authSource = signals.router.host.string("auth");
    const workspaceReadySource = signals.router.resource.boolean("workspaceReady");
    const tenantCapabilitySource = signals.router.graph.string("tenantCapability");
    const workspaceActive = signals.router.prerequisite("workspace-active", ({ facts, allow, unavailable }) => (
      facts.workspaceState === "active"
        ? allow({ reason: "workspaceActive" })
        : unavailable({ reason: "workspaceInactive", detail: "Workspace is not active." })
    ));
    const authRequired = signals.router.prerequisite("auth-required", {
      consumes: [authSource, workspaceReadySource, tenantCapabilitySource],
      evaluate: ({ consume, allow, redirect, denied }) => {
        if (consume(workspaceReadySource) !== true || consume(tenantCapabilitySource) !== "granted") {
          return denied({
            reason: "capabilityUnavailable",
            detail: "Route admission sources did not grant capability truth.",
          });
        }
        return consume(authSource) === "signedIn"
          ? allow({ reason: "authenticated" })
          : redirect({ href: "/login", reason: "authRequired", detail: "Sign in is required." });
      },
    });
    const adminRole = signals.router.prerequisite("admin-role", ({ forbidden }) => (
      forbidden({ reason: "missingAdminRole", detail: "Admin privileges are required." })
    ));
    const maintenanceWindow = signals.router.prerequisite("maintenance-window", ({ facts, allow, unavailable }) => (
      facts.maintenance === true
        ? unavailable({ reason: "maintenanceWindow", detail: "Route is under maintenance." })
        : allow({ reason: "available" })
    ));
    const licenseCheck = signals.router.prerequisite("license-check", ({ denied }) => (
      denied({ reason: "licenseRequired", detail: "A product license is required." })
    ));
    const redirectFirst = signals.router.prerequisite("redirect-first", ({ redirect }) => {
      visitedPrerequisites.push("redirect-first");
      return redirect({ href: "/login", reason: "authRequired" });
    });
    const redirectSecond = signals.router.prerequisite("redirect-second", ({ denied }) => {
      visitedPrerequisites.push("redirect-second");
      return denied({ reason: "shouldNotRun" });
    });
    const projectAvailable = signals.router.prerequisite("project-available", ({ facts, allow, notFound }) => (
      facts.projectState === "deleted"
        ? notFound({ reason: "projectMissing", detail: "Project no longer exists." })
        : allow({ reason: "projectAvailable" })
    ));
    const staleProjectRecovery = signals.router.recovery(
      "stale-project-recovery",
      ({ facts, terminalArtifact, fallback }) => {
        visitedRecoveries.push("stale-project-recovery");
        if (terminalArtifact.kind !== "notFound") {
          return null;
        }
        return fallback({
          href: facts.invalidRecoveryTarget === true ? "/missing-recovery-target" : "/projects",
          reason: "staleProject",
          detail: "Recover to the projects index when the requested project is gone.",
        });
      },
    );

    const routes = signals.router.define({
      app: signals.router.layout(
        signals.router.route("/", { admission: [workspaceActive] }),
        { outlet: "shell" },
        {
          home: signals.router.route("/"),
          projects: signals.router.layout("/projects", { outlet: "detail" }, {
            index: signals.router.route("/projects"),
            detail: signals.router.route("/projects/:projectId", {
              admission: [projectAvailable],
              recovery: [staleProjectRecovery],
            }),
          }),
          users: signals.router.layout("/users", { outlet: "detail" }, {
      detail: signals.router.route("/users/:userId", {
        search: {
          tab: signals.router.search.optional.string(),
        },
        controllers: {
          detail: detailController,
        },
        graphs: {
          detailGraph,
        },
        admission: [authRequired],
        forms: signals.router.forms("user-detail-form", {
          continuity: "preserve",
        }),
      }),
          }),
          admin: signals.router.route("/admin", {
            admission: [adminRole],
          }),
          maintenance: signals.router.route("/maintenance", {
            admission: [maintenanceWindow],
          }),
          licensed: signals.router.route("/licensed", {
            admission: [licenseCheck],
          }),
          redirectChain: signals.router.route("/redirect-chain", {
            admission: [redirectFirst, redirectSecond],
          }),
        },
      ),
    });
    try {
      await run({ routes, visitedPrerequisites, visitedRecoveries });
    } finally {
      signals.free();
    }
  } finally {
    await cleanup();
  }
}
