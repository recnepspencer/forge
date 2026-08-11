/**
 * Track 1: HTML-as-WASM must fail with the package diagnostic, not only a
 * bare WebAssembly.CompileError / opaque instantiate failure.
 */
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { access } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const pkgEntrypoint = path.join(
  repoRoot,
  "crates/worth-signal-wasm/pkg/worth_signal_wasm.js",
);

async function runIsolatedHtmlInitProbe() {
  await access(pkgEntrypoint);
  const entryUrl = pathToFileURL(pkgEntrypoint).href;
  const childSource = `
import init from ${JSON.stringify(entryUrl)};

const htmlBody = "<!doctype html><html><body>not-wasm</body></html>";
const response = new Response(htmlBody, {
  status: 200,
  headers: { "Content-Type": "text/html; charset=utf-8" },
});

async function expectHtmlDiagnostic(label) {
  try {
    await init(response.clone());
    return { label, threw: false };
  } catch (error) {
    return {
      label,
      threw: true,
      name: error instanceof Error ? error.name : typeof error,
      message: error instanceof Error ? error.message : String(error),
    };
  }
}

const first = await expectHtmlDiagnostic("first");
const second = await expectHtmlDiagnostic("retry");
process.stdout.write(JSON.stringify({ first, second }));
`;

  return await new Promise((resolve, reject) => {
    const child = spawn(process.execPath, ["--input-type=module"], {
      cwd: repoRoot,
      stdio: ["pipe", "pipe", "pipe"],
    });
    let stdout = "";
    let stderr = "";
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", reject);
    child.on("close", (code) => {
      if (code !== 0) {
        reject(new Error(`probe exited ${code}: ${stderr || stdout}`));
        return;
      }
      try {
        resolve(JSON.parse(stdout));
      } catch (error) {
        reject(
          new Error(
            `probe returned non-JSON (stderr=${stderr}): ${error}`,
          ),
        );
      }
    });
    child.stdin.end(childSource);
  });
}

function assertPackageHtmlDiagnostic(result) {
  assert.equal(result.threw, true, `${result.label} must reject HTML bytes`);
  assert.match(
    result.message,
    /worth-signals-wasm: expected WASM bytes/u,
    `${result.label} must use the package diagnostic prefix`,
  );
  assert.match(result.message, /received HTML/u);
  assert.match(result.message, /3c 21 64 6f/u);
  assert.match(
    result.message,
    /createSignals\(\{\s*assets:/u,
  );
  assert.notEqual(
    result.name,
    "CompileError",
    `${result.label} must not surface only a bare CompileError`,
  );
}

test("prepared entry rejects HTML Response with remediation diagnostic", async () => {
  const probe = await runIsolatedHtmlInitProbe();
  assertPackageHtmlDiagnostic(probe.first);
  assertPackageHtmlDiagnostic(probe.second);
});
