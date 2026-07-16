import React, { useState } from "react";
import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";

interface DemoFourProps {
  signals: any;
  demo: any;
  onNavigate: any;
}

export const DemoFour: React.FC<DemoFourProps> = ({ signals, demo, onNavigate }) => {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState({
    id: "t-4",
    title: "Complete worth compiler verification",
    status: "In Progress",
  });

  // Real signals resource setup
  const api = React.useMemo(() => signals.api({ baseUrl: "/api" }), [signals]);
  const taskDetail = React.useMemo(
    () =>
      api.url("/tasks/:taskId").detail({
        load: async ({ taskId: _taskId }: any) => {
          setLoading(true);
          await new Promise((r) => setTimeout(r, 1200));
          setLoading(false);
          return data;
        },
      }),
    [api, data],
  );

  const line = React.useMemo(() => taskDetail.line({ taskId: "t-4" }), [taskDetail]);
  const taskVal = useSignal<any>(signals, line.signal());
  const lineStatus = line.status().kind;

  const handleRefresh = () => {
    line.refresh();
  };

  const handleStatusChange = (newStatus: string) => {
    setData((d) => ({ ...d, status: newStatus }));
    line.refresh();
  };

  const displayVal = taskVal || data;

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      inspectorContent={
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <div><span style={{ color: "#10b981" }}>[resource:line]</span> id = "t-4"</div>
          <div><span style={{ color: "#38bdf8" }}>[resource:status]</span> status = "{lineStatus}"</div>
          <div><span style={{ color: "#f59e0b" }}>[resource:freshness]</span> freshness = "{line.freshness().kind}"</div>
          <div><span style={{ color: "#85e89d" }}>[resource:value]</span> payload = {JSON.stringify(displayVal)}</div>
        </div>
      }
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <div>
            <h4 style={{ fontWeight: 600, color: "var(--text-primary)" }}>{displayVal.title}</h4>
            <span style={{ fontSize: "0.8rem", color: "var(--text-muted)" }}>ID: {displayVal.id}</span>
          </div>
          <span className={`badge ${lineStatus === "pending" ? "badge-forms" : "badge-resources"}`}>
            {lineStatus === "pending" ? "PENDING" : "SETTLED"}
          </span>
        </div>

        <div style={{ display: "flex", gap: "0.5rem", flexWrap: "wrap" }}>
          <button className="btn" onClick={() => handleStatusChange("Completed")} disabled={loading}>Mark Completed</button>
          <button className="btn" onClick={() => handleStatusChange("In Progress")} disabled={loading}>In Progress</button>
          <button
            className="btn btn-primary"
            onClick={handleRefresh}
            disabled={loading}
            style={{ background: "var(--accent-resources)", color: "black", border: "none" }}
          >
            {loading ? "Re-fetching..." : "Refresh Cache"}
          </button>
        </div>
      </div>
    </DemoShell>
  );
};
