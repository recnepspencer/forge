import { NodeTree } from "./signal_graph_tree";

export function SignalGraphPanel({
  teethCount,
  tracedNode,
  conflictedNodes,
  resolvedNodes,
  onInspect,
}: {
  teethCount: number;
  tracedNode: string | null;
  conflictedNodes: Set<string>;
  resolvedNodes: Set<string>;
  onInspect: (id: string) => void;
}) {
  return (
    <section className="panel">
      <span className="panel__eyebrow">Signal Graph</span>
      <p className="panel__hint">Tap a node to explore its lineage.</p>
      <NodeTree
        teethCount={teethCount}
        tracedNode={tracedNode}
        conflictedNodes={conflictedNodes}
        resolvedNodes={resolvedNodes}
        onInspect={onInspect}
      />
    </section>
  );
}
