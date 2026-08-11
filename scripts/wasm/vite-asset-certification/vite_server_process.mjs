import { spawn } from "node:child_process";
import path from "node:path";
import process from "node:process";

export async function startViteServer(options) {
  const {
    cwd,
    script,
    port,
    readyPattern = /Local:\s+http:\/\/127\.0\.0\.1/u,
    timeoutMs = 120_000,
  } = options;

  const viteEntrypoint = path.join(cwd, "node_modules", "vite", "bin", "vite.js");
  const viteArgs = script === "preview"
    ? ["preview", "--host", "127.0.0.1", "--port", String(port)]
    : ["--host", "127.0.0.1", "--port", String(port)];
  const child = spawn(process.execPath, [viteEntrypoint, ...viteArgs], {
    cwd,
    env: { ...process.env, BROWSER: "none" },
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });

  let stdout = "";
  let stderr = "";
  let settled = false;

  const ready = new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      if (settled) {
        return;
      }
      settled = true;
      reject(
        new Error(
          `Vite ${script} on port ${port} did not become ready within ${timeoutMs}ms\n` +
            `stdout:\n${stdout}\nstderr:\n${stderr}`,
        ),
      );
    }, timeoutMs);

    const onChunk = (chunk, streamName) => {
      const text = chunk.toString("utf8");
      if (streamName === "stdout") {
        stdout += text;
      } else {
        stderr += text;
      }
      if (!settled && readyPattern.test(stdout + stderr)) {
        settled = true;
        clearTimeout(timer);
        resolve();
      }
    };

    child.stdout.on("data", (chunk) => onChunk(chunk, "stdout"));
    child.stderr.on("data", (chunk) => onChunk(chunk, "stderr"));
    child.on("exit", (code, signal) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      reject(
        new Error(
          `Vite ${script} exited before ready (code=${code}, signal=${signal})\n` +
            `stdout:\n${stdout}\nstderr:\n${stderr}`,
        ),
      );
    });
  });

  await ready;

  return {
    port,
    baseUrl: `http://127.0.0.1:${port}`,
    stdout: () => stdout,
    stderr: () => stderr,
    async stop() {
      if (child.exitCode !== null || child.killed) {
        return;
      }
      child.kill("SIGTERM");
      await new Promise((resolve) => {
        const timer = setTimeout(() => {
          child.kill("SIGKILL");
          resolve();
        }, 5_000);
        child.on("exit", () => {
          clearTimeout(timer);
          resolve();
        });
      });
    },
  };
}

export async function runViteBuild(cwd) {
  const { execFileAsync } = await import(
    "../verify-worth-signals-wasm-package-support.mjs"
  );
  const viteEntrypoint = path.join(cwd, "node_modules", "vite", "bin", "vite.js");
  await execFileAsync(process.execPath, [viteEntrypoint, "build"], { cwd });
}
