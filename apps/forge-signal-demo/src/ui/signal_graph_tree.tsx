import { useCallback, useMemo } from "react";

import {
  buildToothNodes,
  nodeChipClassName,
  NODES,
  STATIC_LAYERS,
} from "../state/node_view";

function LegendChip({ label, className }: { label: string; className: string }) {
  return <span className={`node-chip node-chip--legend ${className}`}>{label}</span>;
}

export function NodeTree({
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
  const toothNodes = useMemo(() => buildToothNodes(teethCount), [teethCount]);
  const getChildren = useCallback((id: string): Array<{ id: string; label: string }> => {
    if (id === "gearTeeth") {
      return toothNodes;
    }
    const node = NODES[id];
    if (!node) return [];
    return node.children.map((cid) => ({ id: cid, label: NODES[cid]?.label ?? cid }));
  }, [toothNodes]);

  return (
    <div className="node-tree">
      <div className="node-tree__legend">
        <LegendChip label="Topology" className="node-chip--topology" />
        <LegendChip label="Render" className="node-chip--render" />
        <LegendChip label="Output" className="node-chip--output" />
        <LegendChip label="Conflict" className="node-chip--conflict" />
        <LegendChip label="Resolved" className="node-chip--resolved" />
      </div>
      {STATIC_LAYERS.map((layer) => (
        <div key={layer.label} className="node-tree__layer">
          <span className="node-tree__layer-label">{layer.label}</span>
          <div className="node-tree__nodes">
            {layer.ids.map((id) => {
              const isSelected = tracedNode === id;
              const children = isSelected ? getChildren(id) : [];
              return (
                <div key={id} className="node-tree__node-group">
                  <button
                    type="button"
                    data-node-id={id}
                    className={nodeChipClassName(id, isSelected, conflictedNodes, resolvedNodes)}
                    onClick={() => onInspect(id)}
                  >
                    {NODES[id]?.label ?? id}
                    {id === "gearTeeth" && <span className="node-tree__count">{teethCount}</span>}
                  </button>
                  {children.length > 0 && (
                    <div className="node-tree__children">
                      {children.map((c) => (
                        <button
                          key={c.id}
                          type="button"
                          data-node-id={c.id}
                          className={`${nodeChipClassName(c.id, tracedNode === c.id, conflictedNodes, resolvedNodes)} node-chip--sm`}
                          onClick={() => onInspect(c.id)}
                        >
                          {c.label}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}
