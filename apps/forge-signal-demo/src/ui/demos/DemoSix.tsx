import React, { useState, useEffect } from "react";
import { DemoShell } from "../DemoShell";
import { useSignal } from "../Demos";

interface DemoSixProps {
  signals: any;
  demo: any;
  onNavigate: any;
}

export const DemoSix: React.FC<DemoSixProps> = ({ signals, demo, onNavigate }) => {
  const history = React.useMemo(() => signals.history(), [signals]);
  const [branches, setBranches] = useState<any[]>([]);
  const [activeBranch, setActiveBranch] = useState<any>(null);

  // Inputs on active runtime
  const taskTitleInput = React.useMemo(() => signals.input("Initial production task"), [signals]);
  const taskTitle = useSignal<string>(signals, taskTitleInput);

  const refreshBranchesList = () => {
    const list = history.branches();
    setBranches(list);
    setActiveBranch(history.current_branch());
  };

  useEffect(() => {
    refreshBranchesList();
  }, [history]);

  const handleCreateBranch = () => {
    const name = `what-if-${branches.length}`;
    history.create_branch(name);
    refreshBranchesList();
  };

  const handleSwitchBranch = (branchId: number) => {
    history.switch_branch(branchId);
    refreshBranchesList();
  };

  return (
    <DemoShell
      demo={demo}
      onNavigate={onNavigate}
      inspectorContent={
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <div><span style={{ color: "#ef4444" }}>[history:branch]</span> current = {`{"id": ${activeBranch?.id}, "name": "${activeBranch?.name}"}`}</div>
          <div><span style={{ color: "#ef4444" }}>[history:registry]</span> branches = {JSON.stringify(branches.map((b) => ({ id: b.id, name: b.name })))}</div>
          <div><span style={{ color: "#10b981" }}>[reactivity:input]</span> taskTitle = "{taskTitle}"</div>
        </div>
      }
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
        {/* Title Input */}
        <div>
          <label style={{ fontSize: "0.8rem", color: "var(--text-muted)", display: "block", marginBottom: "0.5rem" }}>
            TASK TITLE ON ACTIVE BRANCH ({activeBranch?.name.toUpperCase()})
          </label>
          <input
            className="form-input"
            value={taskTitle || ""}
            onChange={(e) => taskTitleInput.set(e.target.value)}
          />
        </div>

        {/* Branch Management */}
        <div style={{ borderTop: "1px solid var(--border-light)", paddingTop: "1rem" }}>
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "1rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 600, color: "var(--text-secondary)" }}>ACTIVE WORKSPACE BRANCHES</span>
            <button
              className="btn"
              onClick={handleCreateBranch}
              style={{ fontSize: "0.8rem", borderColor: "var(--accent-history)", color: "var(--accent-history)" }}
            >
              + Create branch
            </button>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
            {branches.map((branch) => (
              <div
                key={branch.id}
                style={{
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                  padding: "0.75rem 1rem",
                  borderRadius: "6px",
                  background: activeBranch?.id === branch.id ? "rgba(239, 68, 68, 0.06)" : "rgba(255,255,255,0.01)",
                  border: `1px solid ${activeBranch?.id === branch.id ? "var(--accent-history)" : "var(--border-light)"}`,
                }}
              >
                <div>
                  <strong style={{ fontSize: "0.9rem", color: "var(--text-primary)" }}>{branch.name}</strong>
                  <div style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>Branch id: {branch.id}</div>
                </div>
                <button
                  className="btn"
                  disabled={activeBranch?.id === branch.id}
                  onClick={() => handleSwitchBranch(branch.id)}
                  style={{ fontSize: "0.8rem", padding: "0.4rem 0.8rem" }}
                >
                  {activeBranch?.id === branch.id ? "Active" : "Checkout"}
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>
    </DemoShell>
  );
};
