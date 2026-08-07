/**
 * Anti-theatre: each probe inverts one production fix in a child process and
 * requires the matching attack test to FAIL for the named reason.
 */
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const here = path.dirname(fileURLToPath(import.meta.url));
const crateRoot = path.resolve(here, "../../../../../");
const forgeRoot = path.resolve(crateRoot, "../..");
const adversarial = path.join(here, "worker_first_react_adversarial.test.mjs");
const nasty = path.join(here, "worker_first_react_nasty.test.mjs");
const collabStack = path.join(here, "worker_first_react_collab_stack_attack.test.mjs");

function runNamedTest(testFile, namePattern, timeoutMs = 20_000) {
  const env = { ...process.env };
  delete env.NODE_TEST_CONTEXT;
  delete env.NODE_OPTIONS;
  const result = spawnSync(
    process.execPath,
    [
      "--experimental-wasm-modules",
      "--test",
      "--test-force-exit",
      `--test-timeout=${timeoutMs}`,
      `--test-name-pattern=${namePattern}`,
      testFile,
    ],
    { cwd: forgeRoot, encoding: "utf8", env },
  );
  return {
    status: result.status,
    out: `${result.stdout ?? ""}${result.stderr ?? ""}`,
  };
}

function withPatchedFile(relativePath, mutate, fn) {
  const absolute = path.join(crateRoot, relativePath);
  const original = readFileSync(absolute, "utf8");
  const patched = mutate(original);
  assert.notEqual(patched, original, `patch must change ${relativePath}`);
  writeFileSync(absolute, patched, "utf8");
  try {
    return fn();
  } finally {
    writeFileSync(absolute, original, "utf8");
  }
}

test("MUTATION: drop React first-subscribe refresh => stale ATTACK fails", () => {
  withPatchedFile("react/store.ts", (source) => source.replace(
    `    // Mutations may have landed before the first React diagnostics subscriber
    // attached; sync immediately so getDiagnosticsSnapshot is not permanently stale.
    refreshDiagnosticsSnapshot();
`,
    "",
  ), () => {
    const result = runNamedTest(
      adversarial,
      "React diagnostics stay stale if mutations happen before first subscribeDiagnostics",
    );
    assert.notEqual(result.status, 0, result.out);
    assert.match(result.out, /must refresh from the live runtime|notStrictEqual|ERR_ASSERTION/u);
  });
});

test("MUTATION: drop live notify isolation => throwing listener ATTACK fails", () => {
  withPatchedFile(
    "package-src/product/entrypoint/worker_first_root_live_diagnostics.ts",
    (source) => source.replace(
      `  function notify() {
    for (const listener of [...listeners]) {
      try {
        listener();
      } catch {
        // One faulty subscriber must not silence sibling diagnostics delivery
        // or abort terminate/reset notification.
      }
    }
  }`,
      `  function notify() {
    for (const listener of [...listeners]) {
      listener();
    }
  }`,
    ),
    () => {
      const result = runNamedTest(
        adversarial,
        "one throwing diagnostics listener must not silence the others",
      );
      assert.notEqual(result.status, 0, result.out);
      assert.match(result.out, /listener-a-boom|sibling listener must still run|ERR_ASSERTION/u);
    },
  );
});

test("MUTATION: drop latestObservation requireActive => terminate ATTACK fails", () => {
  withPatchedFile(
    "package-src/product/entrypoint/worker_first_root_cached_facades.ts",
    (source) => source.replace(
      `    latestObservation() {
      rootSession.requireActiveDiagnostics("diagnostics.latestObservation");
      return diagnosticsContextOrLive(rootSession).latestObservation;
    },`,
      `    latestObservation() {
      return diagnosticsContextOrLive(rootSession).latestObservation;
    },`,
    ),
    () => {
      const result = runNamedTest(
        adversarial,
        "post-terminate diagnostics surface must fail closed",
      );
      assert.notEqual(result.status, 0, result.out);
      assert.match(result.out, /Missing expected exception|ERR_ASSERTION/u);
    },
  );
});

test("MUTATION: swallow React diagnostics schedule => dual-store ATTACK fails", () => {
  withPatchedFile("react/store.ts", (source) => source.replace(
    `  function scheduleDiagnosticsRefresh(): void {
    if (diagnosticsRefreshQueued) {
      return;
    }
    diagnosticsRefreshQueued = true;
    enqueueMicrotask(() => {
      diagnosticsRefreshQueued = false;
      refreshDiagnosticsSnapshot();
    });
  }`,
    `  function scheduleDiagnosticsRefresh(): void {}`,
  ), () => {
    const result = runNamedTest(
      nasty,
      "two React stores on one runtime must both observe mutations",
    );
    assert.notEqual(result.status, 0, result.out);
  });
});

test("MUTATION: allow forged collab branchId => stacked collab ATTACK fails", () => {
  withPatchedFile(
    "package-src/product/forms/collaboration/collaboration_resource_proof.ts",
    (source) => {
      const patched = source.replace(
        /export function pinCollaborationBranchId\([\s\S]*?\n\}\r?\n\r?\nexport function readCollaborationBranchId/,
        `export function pinCollaborationBranchId(resourceProof, reportedBranchId) {
  return reportedBranchId ?? resourceProof.branchId;
}

export function readCollaborationBranchId`,
      );
      return patched;
    },
    () => {
      const result = runNamedTest(
        collabStack,
        "collab form \\+ open optimistic line \\+ router freeze fails closed on importGraph",
        60_000,
      );
      assert.notEqual(result.status, 0, result.out);
      assert.match(
        result.out,
        /must match admitted resource branch proof|Missing expected exception|ERR_ASSERTION|strictEqual|deepStrictEqual/u,
      );
    },
  );
});

test("MUTATION: sticky React cache skips root read => stacked collab ATTACK fails", () => {
  withPatchedFile("react/store.ts", (source) => source.replace(
    `    if (entry.listeners.size === 0 || entry.snapshotVersion !== entry.version) {
      entry.snapshot = readSignalValue(signals, entry.target);
      entry.snapshotVersion = entry.version;
    } else {
      signals.read(entry.target);
    }
    return entry.snapshot;`,
    `    if (entry.listeners.size === 0 || entry.snapshotVersion !== entry.version) {
      entry.snapshot = readSignalValue(signals, entry.target);
      entry.snapshotVersion = entry.version;
    }
    return entry.snapshot;`,
  ), () => {
    const result = runNamedTest(
      collabStack,
      "collab form \\+ open optimistic line \\+ router freeze fails closed on importGraph",
      60_000,
    );
    assert.notEqual(result.status, 0, result.out);
    assert.match(result.out, /fail closed|Missing expected exception|ERR_ASSERTION|Zombie|Optimistic-2/u);
  });
});
