function createTreeTasks(runtime, url, overrides = {}) {
  return runtime.signals.api({}).url(url)
    .response(runtime.signals.resource.response.tree()({
      itemId: (task) => task.id,
      roots: (value) => value.roots,
      children: (node) => node.children ?? [],
      replaceChildren: overrides.replaceChildren ?? ((node, nextChildren) => ({
        ...node,
        children: nextChildren,
      })),
      replaceRoots: (value, nextRoots) => ({ ...value, roots: nextRoots }),
      nodeForItem: overrides.nodeForItem ?? ((itemId) => ["root", itemId]),
      replaceNode: (value, path, itemId, nextNode) => ({
        ...value,
        roots: replaceTreeNode(value.roots, path, itemId, nextNode),
      }),
    }))
    .list({
      load: () => ({
        roots: [{
          id: "root",
          title: "Root",
          children: [{ id: "task:1", title: "First", children: [] }],
        }],
      }),
    });
}

function replaceTreeNode(nodes, path, itemId, nextNode) {
  const [head, ...tail] = path;
  return nodes.map((node) => {
    if (node.id !== head) {
      return node;
    }
    if (tail.length === 0 && node.id === itemId) {
      return nextNode;
    }
    return {
      ...node,
      children: replaceTreeNode(node.children, tail, itemId, nextNode),
    };
  });
}

export { createTreeTasks };
