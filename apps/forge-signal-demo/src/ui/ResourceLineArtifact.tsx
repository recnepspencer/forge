import "./resourceLineArtifact.css";

const rows = [
  { label: "products.line({})", state: "fulfilled", width: "86%" },
  { label: 'product.line({ productId: "p-204" })', state: "fresh", width: "68%" },
  { label: "diagnostics.lastEffect", state: "deliveryPatch", width: "74%" },
] as const;

export function ResourceLineArtifact() {
  return (
    <div className="xai-line-artifact" aria-hidden="true">
      <div className="xai-line-artifact-orb" />
      <div className="xai-line-artifact-stack">
        {rows.map((row, index) => {
          const style = { "--line-width": row.width, "--line-delay": `${index * 180}ms` } as any;
          return (
            <div className="xai-line-artifact-row" key={row.label} style={style}>
              <span className="xai-line-artifact-dot" />
              <strong>{row.label}</strong>
              <em>{row.state}</em>
              <i />
            </div>
          );
        })}
      </div>
      <div className="xai-line-artifact-inspector">
        <span>line.summary()</span>
        <b>status: fulfilled</b>
        <b>freshness: fresh</b>
        <b>history: 3 effects</b>
      </div>
    </div>
  );
}
