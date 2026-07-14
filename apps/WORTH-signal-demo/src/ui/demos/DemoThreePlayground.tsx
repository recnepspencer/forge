import React from "react";
import type { DemoRoleProfile, NavLinkDef, ReplayMode } from "./demoThreeRouterModel";

interface DemoThreePlaygroundProps {
  role: DemoRoleProfile;
  navLinks: NavLinkDef[];
  sequence: NavLinkDef[];
  setSequence: React.Dispatch<React.SetStateAction<NavLinkDef[]>>;
  appendLink: (linkId: string) => void;
  simulate: () => Promise<void>;
  replayMode: ReplayMode;
  setReplayMode: (mode: ReplayMode) => void;
  replay: () => void;
  outcomes: any[];
  replayRows: string[];
  playgroundStatus: string;
}

export const DemoThreePlayground: React.FC<DemoThreePlaygroundProps> = ({
  role,
  navLinks,
  sequence,
  setSequence,
  appendLink,
  simulate,
  replayMode,
  setReplayMode,
  replay,
  outcomes,
  replayRows,
  playgroundStatus,
}) => {
  return (
    <div className="glass-panel" style={{ padding: "1rem", display: "grid", gap: "1rem" }}>
      <div>
        <div style={{ color: "var(--accent-router)", fontSize: "0.76rem", fontWeight: 700, letterSpacing: "1px" }}>
          ROUTER PLAYGROUND
        </div>
        <div style={{ color: "var(--text-secondary)", fontSize: "0.92rem", marginTop: "0.35rem", lineHeight: "1.6" }}>
          Build a session as <strong style={{ color: "var(--text-primary)" }}>{role.label}</strong>, simulate the route chain, then replay how the same URLs land differently when the role changes.
        </div>
      </div>

      <div className="demo-playground-grid">
        <div className="glass-panel" style={{ padding: "1rem", display: "grid", gap: "0.85rem" }}>
          <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Session Builder</div>
          <div style={{ display: "flex", gap: "0.6rem", flexWrap: "wrap" }}>
            {navLinks.map((link) => (
              <button key={link.id} className="btn" onClick={() => appendLink(link.id)}>
                Queue {link.label}
              </button>
            ))}
          </div>
          <div style={{ display: "grid", gap: "0.7rem" }}>
            {sequence.map((link, index) => (
              <div key={`${link.id}-${index}`} style={{ display: "flex", justifyContent: "space-between", gap: "0.8rem", alignItems: "center", padding: "0.8rem", borderRadius: "10px", border: "1px solid var(--border-light)", background: "rgba(255,255,255,0.03)" }}>
                <div>
                  <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Step {index + 1}: {link.label}</div>
                  <div style={{ color: "var(--text-muted)", fontSize: "0.78rem", marginTop: "0.2rem" }}>{link.href}</div>
                </div>
                <button className="btn" onClick={() => setSequence((prev) => prev.filter((_, itemIndex) => itemIndex !== index))}>
                  Remove
                </button>
              </div>
            ))}
          </div>
        </div>

        <div className="glass-panel" style={{ padding: "1rem", display: "grid", gap: "0.85rem" }}>
          <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Replay Controls</div>
          <button className="btn btn-primary" onClick={() => void simulate()} style={{ background: "var(--accent-router)", color: "white", border: "none" }}>
            Simulate Session
          </button>
          <select
            value={replayMode}
            onChange={(event) => setReplayMode(event.target.value as ReplayMode)}
            style={{ background: "var(--bg-secondary)", color: "var(--text-primary)", border: "1px solid var(--border-light)", borderRadius: "8px", padding: "0.7rem 0.9rem" }}
          >
            <option value="boundary">Replay Route Admissions</option>
            <option value="breadcrumbs">Replay Breadcrumb Changes</option>
            <option value="history">Replay Session Growth</option>
          </select>
          <button className="btn" onClick={replay} disabled={outcomes.length === 0}>
            Replay Outcome
          </button>
          <div style={{ color: "var(--text-muted)", fontSize: "0.82rem", lineHeight: "1.55" }}>{playgroundStatus}</div>
        </div>
      </div>

      <div className="demo-playground-summary-grid">
        <div className="glass-panel" style={{ padding: "1rem", display: "flex", flexDirection: "column", gap: "0.65rem" }}>
          <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Session Outcomes</div>
          {outcomes.length === 0 && (
            <div style={{ color: "var(--text-muted)" }}>Simulate a route chain to see which pages are admitted and which redirect to permission required.</div>
          )}
          {outcomes.map((outcome: any) => (
            <div key={`${outcome.step}-${outcome.href}`} style={{ padding: "0.75rem", borderRadius: "10px", border: "1px solid var(--border-light)", background: "rgba(255,255,255,0.02)" }}>
              <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Step {outcome.step}: {outcome.label}</div>
              <div style={{ color: "var(--text-secondary)", fontSize: "0.8rem", marginTop: "0.3rem", lineHeight: "1.55" }}>
                {outcome.access.allowed ? "Allowed" : "Redirected"} • admitted route={outcome.routeId ?? "none"}
              </div>
            </div>
          ))}
        </div>

        <div className="glass-panel" style={{ padding: "1rem", display: "flex", flexDirection: "column", gap: "0.65rem" }}>
          <div style={{ color: "var(--text-primary)", fontWeight: 600 }}>Replay Output</div>
          {replayRows.length === 0 && (
            <div style={{ color: "var(--text-muted)" }}>Choose a replay mode to inspect the retained session truth.</div>
          )}
          {replayRows.map((row, index) => (
            <div key={`${row}-${index}`} style={{ padding: "0.75rem", borderRadius: "10px", border: "1px solid var(--border-light)", background: "rgba(255,255,255,0.02)", color: "var(--text-secondary)", fontSize: "0.82rem", lineHeight: "1.55" }}>
              {row}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
