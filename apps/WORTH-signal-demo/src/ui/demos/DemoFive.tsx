import React, { useState } from "react";
import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";

interface DemoFiveProps {
  signals: any;
  demo: any;
  onNavigate: any;
}

export const DemoFive: React.FC<DemoFiveProps> = ({ signals, demo, onNavigate }) => {
  const [activeRoute, setActiveRoute] = useState<"detail" | "edit">("detail");
  const [taskData, setTaskData] = useState({
    id: "task-12",
    title: "Parametric fillet mesh math correction",
    status: "In Review",
  });

  const api = React.useMemo(() => signals.api({ baseUrl: "/api" }), [signals]);
  const taskDetail = React.useMemo(
    () =>
      api.url("/tasks/:taskId").detail({
        load: async () => taskData,
      }),
    [api, taskData],
  );

  const line = React.useMemo(() => taskDetail.line({ taskId: "task-12" }), [taskDetail]);
  const taskVal = useSignal<any>(signals, line.signal());
  const displayVal = taskVal || taskData;

  const form = React.useMemo(
    () =>
      signals.form({
        source: displayVal,
        fields: ({ field }: any) => ({
          title: field("title"),
          status: field("status"),
        }),
        validation: ({ field }: any) => ({
          titleRequired: field("title", (title: string) => (
            title
              ? { kind: "valid", field: "title", digest: title }
              : {
                  kind: "invalid",
                  field: "title",
                  message: {
                    code: "title.required",
                    message: "Required",
                    severity: "error",
                    target: "title",
                    audience: "user",
                    visibility: "visible",
                  },
                }
          )),
        }),
      }),
    [signals, displayVal],
  );

  const [_tick, setTick] = useState(0);
  const triggerRender = () => setTick((t) => t + 1);

  const titleVal = form.fields.title.value();
  const statusVal = form.fields.status.value();
  const isDirty = form.dirty().isDirty;
  const isReady = form.readiness().canSubmit;

  const validation = form.validation();
  const titleError = validation.artifacts.find(
    (a: any) => a.field === "title" && a.kind === "invalid"
  )?.message?.message;

  const handleSave = () => {
    setTaskData({
      id: "task-12",
      title: titleVal,
      status: statusVal,
    });
    line.refresh();
    setActiveRoute("detail");
  };

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      inspectorContent={
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <div><span style={{ color: "#8b5cf6" }}>[router]</span> activeRoute = "{activeRoute}"</div>
          <div><span style={{ color: "#38bdf8" }}>[resource:line]</span> id = "task-12", value = {JSON.stringify(displayVal)}</div>
          <div><span style={{ color: "#f59e0b" }}>[form]</span> isDirty = {isDirty ? "true" : "false"}</div>
          <div><span style={{ color: "#10b981" }}>[form]</span> effective = {JSON.stringify(form.effective())}</div>
          <div><span style={{ color: isReady ? "#10b981" : "#ef4444" }}>[form:readiness]</span> canSubmit = {isReady ? "true" : "false"}</div>
        </div>
      }
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
        {activeRoute === "detail" ? (
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <div style={{ display: "flex", justifyContent: "space-between" }}>
              <div>
                <h4 style={{ fontWeight: 600, color: "var(--text-primary)" }}>{displayVal.title}</h4>
                <span style={{ fontSize: "0.8rem", color: "var(--text-muted)" }}>Loaded Resource Truth</span>
              </div>
              <span className="badge badge-composed">{displayVal.status}</span>
            </div>
            <button
              className="btn btn-primary"
              onClick={() => setActiveRoute("edit")}
              style={{ background: "var(--accent-composed)", color: "white", border: "none" }}
            >
              Edit Task details
            </button>
          </div>
        ) : (
          <div style={{ display: "flex", flexDirection: "column", gap: "1.25rem" }}>
            <div>
              <label style={{ fontSize: "0.8rem", color: "var(--text-muted)", display: "block", marginBottom: "0.5rem" }}>EDIT TITLE</label>
              <input
                className="form-input"
                value={titleVal || ""}
                onChange={(e) => {
                  form.fields.title.set(e.target.value);
                  triggerRender();
                }}
              />
              {titleError && (
                <span style={{ color: "var(--accent-history)", fontSize: "0.8rem", marginTop: "0.25rem", display: "block" }}>
                  {titleError}
                </span>
              )}
            </div>
            <div>
              <label style={{ fontSize: "0.8rem", color: "var(--text-muted)", display: "block", marginBottom: "0.5rem" }}>EDIT STATUS</label>
              <select
                className="form-select"
                value={statusVal || ""}
                onChange={(e) => {
                  form.fields.status.set(e.target.value);
                  triggerRender();
                }}
              >
                <option value="In Progress">In Progress</option>
                <option value="In Review">In Review</option>
                <option value="Completed">Completed</option>
              </select>
            </div>
            {isDirty && <div style={{ fontSize: "0.8rem", color: "var(--accent-forms)", background: "rgba(245,158,11,0.05)", padding: "0.5rem", borderRadius: "4px" }}>You have unsaved changes in your draft layer!</div>}
            <div style={{ display: "flex", gap: "0.5rem" }}>
              <button
                className="btn btn-primary"
                disabled={!isReady}
                onClick={handleSave}
                style={{
                  background: isReady ? "var(--accent-composed)" : "rgba(255,255,255,0.04)",
                  color: isReady ? "white" : "var(--text-muted)",
                  border: "none",
                  cursor: isReady ? "pointer" : "not-allowed",
                  opacity: isReady ? 1 : 0.6,
                  transition: "all 0.2s ease-in-out"
                }}
              >
                Save changes
              </button>
              <button className="btn" onClick={() => setActiveRoute("detail")}>Discard draft</button>
            </div>
          </div>
        )}
      </div>
    </DemoShell>
  );
};
