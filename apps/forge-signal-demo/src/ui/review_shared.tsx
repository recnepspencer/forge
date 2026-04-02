export function ProofStat({ label, value }: { label: string; value: string }) {
  return (
    <div className="proof-stat">
      <div className="proof-stat__label">{label}</div>
      <div className="proof-stat__value">{value}</div>
    </div>
  );
}
