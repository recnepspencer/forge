import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));
export const defaultCrateDir = path.resolve(moduleDir, "..", "..", "..");

function normalizePath(filePath) {
  return filePath.replaceAll("\\", "/");
}

function sourceDeclarationPath(publishedTypesPath) {
  const relativePath = publishedTypesPath.replace(/^\.\//u, "");
  return relativePath.startsWith("react/") ? relativePath : `package/${relativePath}`;
}

function publishedEntrypointId(exportPath) {
  return exportPath === "." ? "root" : exportPath.replace(/^\.\//u, "");
}

function readEntrypointDefinitions(crateDir) {
  const publishedManifest = JSON.parse(readFileSync(path.join(crateDir, "pkg", "package.json"), "utf8"));
  return Object.entries(publishedManifest.exports).flatMap(([exportPath, definition]) => {
    // Asset-only exports (for example ./wasm and ./worker) are string paths with
    // no TypeScript declaration surface.
    if (typeof definition === "string") {
      return [];
    }
    if (!definition || typeof definition !== "object" || typeof definition.types !== "string") {
      throw new Error(`Published entrypoint ${exportPath} lacks a declaration path`);
    }
    return [{
      declarationPath: sourceDeclarationPath(definition.types),
      id: publishedEntrypointId(exportPath),
      publishedDeclarationPath: `pkg/${definition.types.replace(/^\.\//u, "")}`,
    }];
  });
}

function assertPublishedDeclarationsSynchronized(crateDir, entrypoints) {
  for (const entrypoint of entrypoints) {
    const sourceText = readFileSync(path.join(crateDir, entrypoint.declarationPath), "utf8");
    const publishedText = readFileSync(path.join(crateDir, entrypoint.publishedDeclarationPath), "utf8");
    if (sourceText !== publishedText) {
      throw new Error(
        `Published declaration ${entrypoint.publishedDeclarationPath} differs from ${entrypoint.declarationPath}`,
      );
    }
  }
}

function declarationKind(declaration) {
  if (ts.isClassDeclaration(declaration)) return "class";
  if (ts.isFunctionDeclaration(declaration)) return "function";
  if (ts.isInterfaceDeclaration(declaration)) return "interface";
  if (ts.isTypeAliasDeclaration(declaration)) return "type";
  if (ts.isVariableDeclaration(declaration)) return "value";
  return ts.SyntaxKind[declaration.kind];
}

function resolveExportSymbol(checker, symbol) {
  return symbol.flags & ts.SymbolFlags.Alias
    ? checker.getAliasedSymbol(symbol)
    : symbol;
}

function normalizedDeclarationText(printer, declaration) {
  const source = declaration.getSourceFile();
  return printer
    .printNode(ts.EmitHint.Unspecified, declaration, source)
    .replaceAll(/\s+/gu, " ")
    .trim();
}

function namedMembers(declaration) {
  if (!ts.isInterfaceDeclaration(declaration) && !ts.isClassDeclaration(declaration)) return [];
  return declaration.members
    .map((member) => member.name?.getText(declaration.getSourceFile()) ?? null)
    .filter((name) => name !== null)
    .sort();
}

function createProgram(crateDir, entrypoints) {
  return ts.createProgram(
    entrypoints.map((entrypoint) => path.join(crateDir, entrypoint.declarationPath)),
    {
      module: ts.ModuleKind.NodeNext,
      moduleResolution: ts.ModuleResolutionKind.NodeNext,
      skipLibCheck: true,
      target: ts.ScriptTarget.ESNext,
    },
  );
}

function collectDeclarationFileInventory({ crateDir, printer, program }) {
  return program.getSourceFiles()
    .map((source) => ({
      source: normalizePath(path.relative(crateDir, source.fileName)),
      signature: printer.printFile(source).replaceAll(/\s+/gu, " ").trim(),
    }))
    .filter(({ source }) =>
      (source.startsWith("package/") || source.startsWith("react/"))
      && source.endsWith(".d.ts"),
    )
    .sort((left, right) => left.source.localeCompare(right.source));
}

function collectEntrypointSurfaces({ checker, crateDir, entrypoint, printer, program }) {
  const sourcePath = path.join(crateDir, entrypoint.declarationPath);
  const source = program.getSourceFile(sourcePath);
  if (!source) throw new Error(`Missing declaration entrypoint: ${entrypoint.declarationPath}`);
  const moduleSymbol = checker.getSymbolAtLocation(source);
  if (!moduleSymbol) throw new Error(`Declaration entrypoint is not a module: ${entrypoint.declarationPath}`);

  return checker.getExportsOfModule(moduleSymbol).map((exportedSymbol) => {
    const resolvedSymbol = resolveExportSymbol(checker, exportedSymbol);
    const declaration = resolvedSymbol.declarations?.[0];
    if (!declaration) throw new Error(`Export ${exportedSymbol.name} has no declaration`);
    const source = normalizePath(path.relative(crateDir, declaration.getSourceFile().fileName));
    const signature = normalizedDeclarationText(printer, declaration);
    const members = namedMembers(declaration);
    return {
      entrypoint: entrypoint.id,
      exportName: exportedSymbol.name,
      kind: declarationKind(declaration),
      memberCount: members.length,
      members,
      signature,
      source,
    };
  });
}

export function collectPublicContractInventory(crateDir = defaultCrateDir) {
  const entrypointDefinitions = readEntrypointDefinitions(crateDir);
  assertPublishedDeclarationsSynchronized(crateDir, entrypointDefinitions);
  const program = createProgram(crateDir, entrypointDefinitions);
  const diagnostics = ts.getPreEmitDiagnostics(program);
  if (diagnostics.length > 0) {
    const message = ts.formatDiagnosticsWithColorAndContext(diagnostics, {
      getCanonicalFileName: (fileName) => fileName,
      getCurrentDirectory: () => crateDir,
      getNewLine: () => "\n",
    });
    throw new Error(`Public declarations do not typecheck:\n${message}`);
  }
  const checker = program.getTypeChecker();
  const printer = ts.createPrinter({ newLine: ts.NewLineKind.LineFeed });
  const surfaces = entrypointDefinitions
    .flatMap((entrypoint) => collectEntrypointSurfaces({ checker, crateDir, entrypoint, printer, program }))
    .sort((left, right) =>
      left.entrypoint.localeCompare(right.entrypoint)
      || left.exportName.localeCompare(right.exportName),
    );
  return {
    declarationFiles: collectDeclarationFileInventory({ crateDir, printer, program }),
    entrypoints: entrypointDefinitions.map((entrypoint) => entrypoint.id),
    surfaces,
  };
}

function matchesRule(surface, rule) {
  return rule.entrypoints.includes(surface.entrypoint)
    && rule.sourcePrefixes.some((prefix) => surface.source.startsWith(prefix));
}

export function assignSurfacesToCoverageGroups(surfaces, policy) {
  const assignments = new Map(policy.groups.map((group) => [group.id, []]));
  const unassigned = [];
  const ambiguous = [];
  for (const surface of surfaces) {
    const matchingGroups = policy.groups.filter((group) => matchesRule(surface, group));
    if (matchingGroups.length === 0) {
      unassigned.push(surface);
    } else if (matchingGroups.length > 1) {
      ambiguous.push({ groups: matchingGroups.map((group) => group.id), surface });
    } else {
      assignments.get(matchingGroups[0].id).push(surface);
    }
  }
  return { ambiguous, assignments, unassigned };
}

export function assignDeclarationFilesToCoverageGroups(declarationFiles, policy) {
  const assignments = new Map(policy.groups.map((group) => [group.id, []]));
  const unassigned = [];
  const ambiguous = [];
  for (const declarationFile of declarationFiles) {
    const matchingGroups = policy.groups.filter((group) =>
      group.filePrefixes.some((prefix) => declarationFile.source.startsWith(prefix)),
    );
    if (matchingGroups.length === 0) {
      unassigned.push(declarationFile);
    } else if (matchingGroups.length > 1) {
      ambiguous.push({ file: declarationFile, groups: matchingGroups.map((group) => group.id) });
    } else {
      assignments.get(matchingGroups[0].id).push(declarationFile);
    }
  }
  return { ambiguous, assignments, unassigned };
}

function stableSurfaceLine(surface) {
  return [
    surface.entrypoint,
    surface.exportName,
    surface.kind,
    surface.source,
    surface.signature,
  ].join("\u001f");
}

export function summarizeSurfaceCoverageGroup(surfaces) {
  const hash = createHash("sha256");
  for (const surface of surfaces) hash.update(`${stableSurfaceLine(surface)}\n`);
  return {
    declarationCount: surfaces.length,
    directMemberCount: surfaces.reduce((total, surface) => total + surface.memberCount, 0),
    signatureDigest: hash.digest("hex"),
  };
}

function summarizeDeclarationFiles(declarationFiles) {
  const hash = createHash("sha256");
  for (const declarationFile of declarationFiles) {
    hash.update(`${declarationFile.source}\u001f${declarationFile.signature}\n`);
  }
  return {
    declarationFileCount: declarationFiles.length,
    declarationFileDigest: hash.digest("hex"),
  };
}

export function summarizeCoveragePolicy(contractInventory, policy) {
  const surfaceCoverage = assignSurfacesToCoverageGroups(contractInventory.surfaces, policy);
  const fileCoverage = assignDeclarationFilesToCoverageGroups(contractInventory.declarationFiles, policy);
  const groups = policy.groups.map((group) => ({
    id: group.id,
    ...summarizeSurfaceCoverageGroup(surfaceCoverage.assignments.get(group.id)),
    ...summarizeDeclarationFiles(fileCoverage.assignments.get(group.id)),
  }));
  return { fileCoverage, groups, surfaceCoverage };
}

export function publicSurfaceName(surface) {
  return `${surface.entrypoint}:${surface.exportName}`;
}
