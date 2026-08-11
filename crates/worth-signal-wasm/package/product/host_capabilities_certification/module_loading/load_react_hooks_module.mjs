import { mkdtemp, mkdir, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.join(moduleDir, "..", "..", "..");
const reactDir = path.join(packageDir, "..", "react");

export async function loadReactHooksModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "worth-signal-react-hooks-"));
  const sourceFiles = [
    ["model.ts", "model.js"],
    ["store.ts", "store.js"],
    ["context.tsx", "context.js"],
    ["hooks.ts", "hooks.js"],
  ];
  try {
    const reactPackageRoot = path.join(packageDir, "..");
    await writeFile(
      path.join(tempDir, "package.json"),
      JSON.stringify({ type: "module" }),
      "utf8",
    );
    await mkdir(path.join(tempDir, "node_modules"), { recursive: true });
    await symlink(
      path.join(reactPackageRoot, "node_modules", "react"),
      path.join(tempDir, "node_modules", "react"),
      "junction",
    );
    for (const [sourceName, outputName] of sourceFiles) {
      const sourcePath = path.join(reactDir, sourceName);
      const source = await readFile(sourcePath, "utf8");
      const transformed = stripTypeScriptTypes(source, { mode: "transform" });
      await writeFile(path.join(tempDir, outputName), transformed, "utf8");
    }
    const moduleUrl = pathToFileURL(path.join(tempDir, "hooks.js")).href;
    const contextUrl = pathToFileURL(path.join(tempDir, "context.js")).href;
    const storeUrl = pathToFileURL(path.join(tempDir, "store.js")).href;
    const [hooks, context, store] = await Promise.all([
      import(moduleUrl),
      import(contextUrl),
      import(storeUrl),
    ]);
    return {
      ...hooks,
      ...context,
      ...store,
      cleanup: () => rm(tempDir, { recursive: true, force: true }),
    };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}
