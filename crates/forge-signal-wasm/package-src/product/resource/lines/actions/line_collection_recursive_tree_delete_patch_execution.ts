function buildRecursiveTreeDeletePatchValue({
  patchRecord,
  currentValue,
  patch,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "recursiveTree") {
    return null;
  }
  const actualTarget = resolveRecursiveTreeDeleteTarget(
    patchRecord,
    currentValue,
    patch.itemId,
    topologyHelpers,
  );
  if (actualTarget === null) {
    return null;
  }
  const treeLookup = requireRecursiveTreeDeleteLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
  );
  if (!areTreePathsEqual(treeLookup.path, actualTarget.path)) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.delete(...) tree lookup path "${treeLookup.path.join(" > ")}" to match actual node path "${actualTarget.path.join(" > ")}" for itemId "${patch.itemId}"`,
    );
  }
  const currentRoots = requireTreeNodeArray(
    topologyHelpers.roots(currentValue),
    patchRecord.familyKind,
    "roots(value)",
  );
  if (actualTarget.path.length === 1) {
    const nextRoots = currentRoots.filter(
      (node) => readRecursiveTreeNodeId(node, patchRecord.familyKind, patchRecord.itemIdentity)
        !== patch.itemId,
    );
    return topologyHelpers.replaceRoots(currentValue, nextRoots);
  }
  return deleteRecursiveTreeNodeAtPath(
    currentValue,
    currentRoots,
    actualTarget.path.slice(0, -1),
    patch.itemId,
    topologyHelpers,
    patchRecord.familyKind,
    patchRecord.itemIdentity,
  );
}

function resolveRecursiveTreeDeleteTarget(
  patchRecord,
  currentValue,
  itemId,
  topologyHelpers,
) {
  const matches = [];
  const roots = requireTreeNodeArray(
    topologyHelpers.roots(currentValue),
    patchRecord.familyKind,
    "roots(value)",
  );
  collectRecursiveTreeDeleteTargets(
    roots,
    [],
    matches,
    itemId,
    topologyHelpers,
    patchRecord.familyKind,
    patchRecord.itemIdentity,
  );
  if (matches.length === 0) {
    return null;
  }
  if (matches.length > 1) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines cannot admit narrow patch(...) for duplicated visible itemId "${itemId}"; use resourcePatch.replace(...) when item identity is ambiguous`,
    );
  }
  return matches[0];
}

function collectRecursiveTreeDeleteTargets(
  siblings,
  parentPath,
  matches,
  itemId,
  topologyHelpers,
  familyKind,
  itemIdentity,
) {
  for (const node of siblings) {
    const nodeId = readRecursiveTreeNodeId(node, familyKind, itemIdentity);
    const nextPath = [...parentPath, nodeId];
    if (nodeId === itemId) {
      matches.push(Object.freeze({ path: Object.freeze(nextPath), item: node }));
    }
    collectRecursiveTreeDeleteTargets(
      requireTreeNodeArray(
        topologyHelpers.children(node),
        familyKind,
        `children("${nodeId}")`,
      ),
      nextPath,
      matches,
      itemId,
      topologyHelpers,
      familyKind,
      itemIdentity,
    );
  }
}

function requireRecursiveTreeDeleteLookup(
  patchRecord,
  currentValue,
  itemId,
  locatedItem,
) {
  if (locatedItem !== undefined) {
    return locatedItem;
  }
  if (typeof patchRecord.reconcile.readItem !== "function") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require readItem(...) proof for exact recursiveTree resourcePatch.delete(...)`,
    );
  }
  return patchRecord.reconcile.readItem(currentValue, itemId);
}

function deleteRecursiveTreeNodeAtPath(
  currentValue,
  currentRoots,
  parentPath,
  itemId,
  topologyHelpers,
  familyKind,
  itemIdentity,
) {
  const [head, ...tail] = parentPath;
  let matchedParent = false;
  const nextRoots = currentRoots.map((node) => {
    if (head !== readRecursiveTreeNodeId(node, familyKind, itemIdentity)) {
      return node;
    }
    matchedParent = true;
    return deleteRecursiveTreeNodeWithinParent(
      node,
      tail,
      itemId,
      topologyHelpers,
      familyKind,
      itemIdentity,
    );
  });
  if (!matchedParent) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.delete(...) tree parent path "${parentPath.join(" > ")}" to resolve an existing parent node`,
    );
  }
  return topologyHelpers.replaceRoots(currentValue, nextRoots);
}

function deleteRecursiveTreeNodeWithinParent(
  node,
  remainingPath,
  itemId,
  topologyHelpers,
  familyKind,
  itemIdentity,
) {
  const currentChildren = requireTreeNodeArray(
    topologyHelpers.children(node),
    familyKind,
    `children("${readRecursiveTreeNodeId(node, familyKind, itemIdentity)}")`,
  );
  if (remainingPath.length === 0) {
    const nextChildren = currentChildren.filter(
      (child) => readRecursiveTreeNodeId(child, familyKind, itemIdentity) !== itemId,
    );
    return topologyHelpers.replaceChildren(node, nextChildren);
  }
  const [head, ...tail] = remainingPath;
  let matchedParent = false;
  const nextChildren = currentChildren.map((child) => {
    if (head !== readRecursiveTreeNodeId(child, familyKind, itemIdentity)) {
      return child;
    }
    matchedParent = true;
    return deleteRecursiveTreeNodeWithinParent(
      child,
      tail,
      itemId,
      topologyHelpers,
      familyKind,
      itemIdentity,
    );
  });
  if (!matchedParent) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.delete(...) tree parent path "${remainingPath.join(" > ")}" to resolve an existing parent node`,
    );
  }
  return topologyHelpers.replaceChildren(node, nextChildren);
}

function areTreePathsEqual(left, right) {
  if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
    return false;
  }
  for (let index = 0; index < left.length; index += 1) {
    if (left[index] !== right[index]) {
      return false;
    }
  }
  return true;
}

function requireTreeNodeArray(nodes, familyKind, source) {
  if (!Array.isArray(nodes)) {
    throw new TypeError(
      `${familyKind} resource lines require recursive-tree ${source} to be an array during resourcePatch.delete(...)`,
    );
  }
  return nodes;
}

function readRecursiveTreeNodeId(node, familyKind, itemIdentity) {
  const nodeId = itemIdentity(node);
  if (typeof nodeId !== "string" || nodeId.length === 0) {
    throw new TypeError(
      `${familyKind} resource lines require recursive-tree itemIdentity(node) to return a non-empty string during resourcePatch.delete(...)`,
    );
  }
  return nodeId;
}

export { buildRecursiveTreeDeletePatchValue };
