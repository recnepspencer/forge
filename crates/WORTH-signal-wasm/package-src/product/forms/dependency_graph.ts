import { FormDeclarationError } from "./form_errors.js";

export function requireDeclaredDependencies(declaredFieldIds, ownerId, dependencies) {
  if (!Array.isArray(dependencies)) {
    throw new FormDeclarationError("form dependency declarations must be arrays", {
      ownerId,
    });
  }
  const seen = new Set();
  for (const dependency of dependencies) {
    if (!declaredFieldIds.has(dependency)) {
      throw new FormDeclarationError("form declaration references an undeclared dependency field", {
        ownerId,
        dependency,
      });
    }
    if (seen.has(dependency)) {
      throw new FormDeclarationError("form declaration dependencies must be unique", {
        ownerId,
        dependency,
      });
    }
    seen.add(dependency);
  }
  return Object.freeze([...dependencies]);
}

export function denyDependencyCycles(declarations, ownerDescription) {
  const graph = new Map();
  const ownerIds = new Set(declarations.map((declaration) => declaration.ownerId));
  for (const declaration of declarations) {
    graph.set(
      declaration.ownerId,
      declaration.dependencies.filter((dependency) => (
        ownerIds.has(dependency) && dependency !== declaration.ownerId
      )),
    );
  }
  const visiting = new Set();
  const visited = new Set();
  for (const ownerId of ownerIds) {
    visitDependency(ownerId, graph, visiting, visited, ownerDescription);
  }
}

function visitDependency(ownerId, graph, visiting, visited, ownerDescription) {
  if (visited.has(ownerId)) {
    return;
  }
  if (visiting.has(ownerId)) {
    throw new FormDeclarationError(`${ownerDescription} dependency cycle denied`, {
      ownerId,
    });
  }
  visiting.add(ownerId);
  for (const dependency of graph.get(ownerId) ?? []) {
    visitDependency(dependency, graph, visiting, visited, ownerDescription);
  }
  visiting.delete(ownerId);
  visited.add(ownerId);
}
