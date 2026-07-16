import { demoRegistry } from "../state/demoData";

interface DemoPrefaceProps {
  demoId: number;
}

export function DemoPreface({ demoId }: DemoPrefaceProps) {
  const demo = demoRegistry.find((entry) => entry.id === demoId);

  if (!demo?.preface) {
    return null;
  }

  return (
    <aside className="xai-demo-preface">
      <span>What this is showing</span>
      <p>{demo.preface}</p>
    </aside>
  );
}
