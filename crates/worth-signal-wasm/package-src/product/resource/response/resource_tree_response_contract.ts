import {
  createCollectionResponse,
} from "./resource_collection_response_factory.js";

function tree() {
  return function defineTreeResponse(options) {
    requireTreeResponseOptions(options);
    return createCollectionResponse(
      "resource.response.tree<T>()(...)",
      createTreeAdapter(options),
      { topology: "recursiveTree", itemField: null },
    );
  };
}

function requireTreeResponseOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "resource.response.tree<T>()(...) requires an options object",
    );
  }
  for (const field of [
    "roots",
    "children",
    "replaceChildren",
    "replaceRoots",
    "nodeForItem",
    "replaceNode",
  ]) {
    if (typeof options[field] !== "function") {
      throw new TypeError(
        `resource.response.tree<T>()(...) requires ${field}(...)`,
      );
    }
  }
}

function createTreeAdapter(options) {
  return {
    ...options,
    topologyHelpers: Object.freeze({
      kind: "recursiveTree",
      roots: options.roots,
      children: options.children,
      replaceChildren: options.replaceChildren,
      replaceRoots: options.replaceRoots,
      nodeForItem: options.nodeForItem,
    }),
    items(value) {
      return readTreeItems(
        options.roots(value),
        options.itemId,
        options.children,
        "roots(value)",
      );
    },
    replaceItems(value, nextItems) {
      requireTreeIdentity(options.roots(value), options.itemId, options.children, "roots(value)");
      return options.replaceRoots(value, [...nextItems]);
    },
    readItem(value, itemIdValue) {
      return readTreeItem(value, itemIdValue, options, "nodeForItem");
    },
    replaceItem(value, itemIdValue, nextItem) {
      return replaceSingleTreeItem(value, itemIdValue, nextItem, options);
    },
  };
}

function replaceSingleTreeItem(value, itemIdValue, nextItem, options) {
  const currentItem = readTreeItem(value, itemIdValue, options, "nodeForItem");
  if (!currentItem.found) {
    throw new RangeError(
      `resource.response.tree<T>()(...) could not find tree node id "${itemIdValue}"`,
    );
  }
  requireTreeNodeIdentity(itemIdValue, nextItem, options.itemId);
  const nextValue = options.replaceNode(
    value,
    currentItem.path,
    itemIdValue,
    nextItem,
  );
  const replacedItem = readTreeItem(
    nextValue,
    itemIdValue,
    options,
    "replaceNode(value, path, itemId, nextNode)",
  );
  if (!replacedItem.found) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires replaceNode(value, path, itemId, nextNode) to preserve tree node "${itemIdValue}"`,
    );
  }
  return nextValue;
}

function readTreeItems(rawRoots, itemId, children, source) {
  return flattenTreeNodes(requireTreeIdentity(rawRoots, itemId, children, source), children);
}

function requireTreeIdentity(rawRoots, itemId, children, source) {
  const roots = requireTreeNodeArray(rawRoots, source);
  const seen = new Set();
  for (const root of roots) {
    visitTreeNode(root, itemId, children, source, seen);
  }
  return roots;
}

function visitTreeNode(node, itemId, children, source, seen) {
  const nodeId = itemId(node);
  if (typeof nodeId !== "string" || nodeId.length === 0) {
    throw new TypeError(
      "resource.response.tree<T>()(...) requires itemId(node) to return a non-empty string",
    );
  }
  if (seen.has(nodeId)) {
    throw new TypeError(
      `resource.response.tree<T>()(...) cannot expose duplicated tree node id "${nodeId}"`,
    );
  }
  seen.add(nodeId);
  for (const child of requireTreeNodeArray(children(node), `${source} children("${nodeId}")`)) {
    visitTreeNode(child, itemId, children, source, seen);
  }
}

function flattenTreeNodes(roots, children) {
  return roots.flatMap((node) => [
    node,
    ...flattenTreeNodes(children(node), children),
  ]);
}

function readTreeItem(value, itemIdValue, options, source) {
  const path = requireTreeNodePath(options.nodeForItem(itemIdValue), itemIdValue, source);
  const node = readTreeNodeAtPath(
    options.roots(value),
    path,
    options,
    "roots(value)",
  );
  if (node === null) {
    return Object.freeze({ found: false, path, item: null });
  }
  const nodeId = options.itemId(node);
  if (nodeId !== itemIdValue) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires ${source}(itemId) path to end at requested node "${itemIdValue}", not "${nodeId}"`,
    );
  }
  return Object.freeze({ found: true, path, item: node });
}

function readTreeNodeAtPath(rawRoots, path, options, source) {
  let siblings = requireTreeNodeArray(rawRoots, source);
  let currentNode = null;
  for (const segment of path) {
    currentNode = readUniqueTreePathSegment(
      siblings,
      segment,
      options.itemId,
      source,
    );
    if (currentNode === null) {
      return null;
    }
    siblings = requireTreeNodeArray(
      options.children(currentNode),
      `${source} children("${segment}")`,
    );
  }
  return currentNode;
}

function readUniqueTreePathSegment(siblings, segment, itemId, source) {
  let matchingNode = null;
  for (const node of siblings) {
    const nodeId = itemId(node);
    if (typeof nodeId !== "string" || nodeId.length === 0) {
      throw new TypeError(
        "resource.response.tree<T>()(...) requires itemId(node) to return a non-empty string",
      );
    }
    if (nodeId !== segment) {
      continue;
    }
    if (matchingNode !== null) {
      throw new TypeError(
        `resource.response.tree<T>()(...) cannot expose duplicated tree node id "${segment}" inside ${source}`,
      );
    }
    matchingNode = node;
  }
  return matchingNode;
}

function requireTreeNodePath(rawPath, itemIdValue, source) {
  if (!Array.isArray(rawPath) || rawPath.length === 0) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires ${source}(itemId) to return a non-empty descendant path`,
    );
  }
  for (const segment of rawPath) {
    if (typeof segment !== "string" || segment.length === 0) {
      throw new TypeError(
        `resource.response.tree<T>()(...) requires ${source}(itemId) descendant path segments to be non-empty strings`,
      );
    }
  }
  if (rawPath.at(-1) !== itemIdValue) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires ${source}(itemId) descendant path to end with item id "${itemIdValue}"`,
    );
  }
  return Object.freeze([...rawPath]);
}

function requireTreeNodeArray(nodes, source) {
  if (!Array.isArray(nodes)) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires ${source} to be an array of tree nodes`,
    );
  }
  return nodes;
}

function requireTreeNodeIdentity(itemIdValue, node, itemId) {
  const nextItemId = itemId(node);
  if (nextItemId !== itemIdValue) {
    throw new TypeError(
      `resource.response.tree<T>()(...) requires replaceNode(value, path, itemId, nextNode) to preserve node id "${itemIdValue}"`,
    );
  }
}

export { tree };
