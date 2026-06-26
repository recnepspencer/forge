import { createSignals } from "@aust-group/forge-signal-wasm";

const root = document.querySelector("#root");

try {
  const workerResult = await withTimeout(
    createSignals().then((workerSignals) => {
      const workerCount = workerSignals.input(1);
      return `worker:${workerCount()}`;
    }),
    10000,
  );
  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityCount = compatibilitySignals.input(1);

  if (root) {
    root.textContent = [
      workerResult,
      `compatibility:${compatibilityCount()}`,
    ].join(" ");
  }
} catch (error) {
  if (root) {
    root.textContent = error instanceof Error ? error.message : String(error);
  }
  throw error;
}

function withTimeout<T>(
  promise: Promise<T>,
  timeoutMs: number,
): Promise<T> {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      reject(new Error(`worker-first smoke timed out after ${timeoutMs}ms`));
    }, timeoutMs);
    promise.then(
      (value) => {
        clearTimeout(timeout);
        resolve(value);
      },
      (error) => {
        clearTimeout(timeout);
        reject(error);
      },
    );
  });
}
