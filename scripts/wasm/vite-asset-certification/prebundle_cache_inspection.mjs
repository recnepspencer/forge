import { access, readdir, readFile } from "node:fs/promises";
import path from "node:path";

/**
 * Independent evidence that Vite actually prebundled the package into
 * node_modules/.vite/deps rather than only serving source files.
 */
export async function inspectVitePrebundleCache(worldRoot, packageName) {
  const depsDir = path.join(worldRoot, "node_modules", ".vite", "deps");
  try {
    await access(depsDir);
  } catch {
    return {
      depsDir,
      present: false,
      packageRelatedEntries: [],
      metadataMentionsPackage: false,
    };
  }

  const entries = await readdir(depsDir);
  const packageRelatedEntries = entries.filter((entry) =>
    entry.toLowerCase().includes(packageName.toLowerCase()) ||
    entry.toLowerCase().includes("worth-signals-wasm") ||
    entry.toLowerCase().includes("worth_signal_wasm")
  );

  let metadataMentionsPackage = false;
  const metadataPath = path.join(depsDir, "_metadata.json");
  try {
    const metadata = await readFile(metadataPath, "utf8");
    metadataMentionsPackage =
      metadata.includes(packageName) || metadata.includes("worth-signals-wasm");
  } catch {
    metadataMentionsPackage = false;
  }

  return {
    depsDir,
    present: true,
    entryCount: entries.length,
    packageRelatedEntries,
    metadataMentionsPackage,
  };
}
