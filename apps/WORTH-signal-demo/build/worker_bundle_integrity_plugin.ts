import type { Plugin } from "vite";
import path from "node:path";

const WORKER_BUNDLE_NAME = "worker_runtime_bridge_worker";

type WorkerBundleOutput = {
  dynamicImports?: string[];
  fileName: string;
  imports?: string[];
  source?: string | Uint8Array;
  type: string;
};

type WorkerOutputBundle = Record<string, WorkerBundleOutput>;

export function workerBundleIntegrityPlugin(): Plugin {
  return {
    name: "worth-worker-bundle-integrity",
    generateBundle(_options, bundle) {
      assertWorkerBundleIntegrity(bundle);
    },
  };
}

export function assertWorkerBundleIntegrity(bundle: WorkerOutputBundle) {
  const workerOutputs = Object.values(bundle).filter(({ fileName }) =>
    fileName.includes(WORKER_BUNDLE_NAME),
  );

  if (workerOutputs.length !== 1) {
    throw new Error(
      `Worth production build must emit one worker runtime bundle; found ${workerOutputs.length}.`,
    );
  }

  const [workerOutput] = workerOutputs;
  for (const dependency of workerDependencies(workerOutput)) {
    if (!(dependency in bundle)) {
      throw new Error(
        `Worth worker runtime bundle references missing output ${dependency}.`,
      );
    }
  }
}

function workerDependencies(output: WorkerBundleOutput) {
  if (output.type === "chunk") {
    return [...(output.imports ?? []), ...(output.dynamicImports ?? [])];
  }

  if (output.source === undefined) {
    throw new Error("Worth worker runtime asset has no emitted source to verify.");
  }

  const source = typeof output.source === "string"
    ? output.source
    : new TextDecoder().decode(output.source);
  const specifiers = [
    ...source.matchAll(/\bimport(?:[^"']*?\bfrom\s*)?["']([^"']+)["']/gu),
    ...source.matchAll(/\bimport\(\s*["']([^"']+)["']\s*\)/gu),
  ].map((match) => match[1]).filter((specifier) => specifier.startsWith("."));

  return specifiers.map((specifier) =>
    path.posix.normalize(path.posix.join(path.posix.dirname(output.fileName), specifier)),
  );
}
