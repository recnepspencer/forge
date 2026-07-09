function buildRecursiveTreeInsertPatchValue({
  patchRecord,
  currentValue,
  patch,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "recursiveTree") {
    return null;
  }
  const treeLookup = requireRecursiveTreeInsertLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
  );
  const parentPath = treeLookup.path.slice(0, -1);
  const currentRoots = requireTreeNodeArray(
    topologyHelpers.roots(currentValue),
    patchRecord.familyKind,
    "roots(value)",
  );
  const nextValue = parentPath.length === 0
    ? topologyHelpers.replaceRoots(
      currentValue,
      insertTreeNodeIntoSiblings(currentRoots, patch),
    )
    : replaceRecursiveTreeChildrenAtPath(
      currentValue,
      currentRoots,
      parentPath,
      topologyHelpers,
      patch,
      patchRecord.familyKind,
      patchRecord.itemIdentity,
    );
  assertRecursiveTreeInsertPreservedLookup(
    patchRecord,
    nextValue,
    patch.itemId,
  );
  return nextValue;
}

function requireRecursiveTreeInsertLookup(
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
      `${patchRecord.familyKind} resource lines require readItem(...) proof for exact recursiveTree resourcePatch.insert(...)`,
    );
  }
  return patchRecord.reconcile.readItem(currentValue, itemId);
}

function replaceRecursiveTreeChildrenAtPath(
  currentValue,
  currentRoots,
  parentPath,
  topologyHelpers,
  patch,
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
    return replaceRecursiveTreeChildrenAtNode(
      node,
      tail,
      topologyHelpers,
      patch,
      familyKind,
      itemIdentity,
    );
  });
  if (!matchedParent) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.insert(...) tree parent path "${parentPath.join(" > ")}" to resolve an existing parent node`,
    );
  }
  return topologyHelpers.replaceRoots(currentValue, nextRoots);
}

function replaceRecursiveTreeChildrenAtNode(
  node,
  remainingPath,
  topologyHelpers,
  patch,
  familyKind,
  itemIdentity,
) {
  const currentChildren = requireTreeNodeArray(
    topologyHelpers.children(node),
    familyKind,
    `children("${readRecursiveTreeNodeId(node, familyKind, itemIdentity)}")`,
  );
  if (remainingPath.length === 0) {
    return topologyHelpers.replaceChildren(
      node,
      insertTreeNodeIntoSiblings(currentChildren, patch),
    );
  }
  const [head, ...tail] = remainingPath;
  let matchedParent = false;
  const nextChildren = currentChildren.map((child) => {
    if (head !== readRecursiveTreeNodeId(child, familyKind, itemIdentity)) {
      return child;
    }
    matchedParent = true;
    return replaceRecursiveTreeChildrenAtNode(
      child,
      tail,
      topologyHelpers,
      patch,
      familyKind,
      itemIdentity,
    );
  });
  if (!matchedParent) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.insert(...) tree parent path "${remainingPath.join(" > ")}" to resolve an existing parent node`,
    );
  }
  return topologyHelpers.replaceChildren(node, nextChildren);
}

function insertTreeNodeIntoSiblings(currentSiblings, patch) {
  return patch.placement === "prepend"
    ? [patch.nextItem, ...currentSiblings]
    : [...currentSiblings, patch.nextItem];
}

function assertRecursiveTreeInsertPreservedLookup(patchRecord, nextValue, itemId) {
  const insertedItem = patchRecord.reconcile.readItem(nextValue, itemId);
  if (insertedItem?.found !== true) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.insert(...) tree reconstruction to preserve inserted node "${itemId}" at its declared descendant path`,
    );
  }
}

function requireTreeNodeArray(nodes, familyKind, source) {
  if (!Array.isArray(nodes)) {
    throw new TypeError(
      `${familyKind} resource lines require recursive-tree ${source} to be an array during resourcePatch.insert(...)`,
    );
  }
  return nodes;
}

function readRecursiveTreeNodeId(node, familyKind, itemIdentity) {
  const nodeId = itemIdentity(node);
  if (typeof nodeId !== "string" || nodeId.length === 0) {
    throw new TypeError(
      `${familyKind} resource lines require recursive-tree itemIdentity(node) to return a non-empty string during resourcePatch.insert(...)`,
    );
  }
  return nodeId;
}

export { buildRecursiveTreeInsertPatchValue };
