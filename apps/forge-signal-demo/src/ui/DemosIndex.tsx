import React from "react";
import { demoRegistry } from "../state/demoData";

interface DemosIndexProps {
  onNavigate: (path: string) => void;
}

export const DemosIndex: React.FC<DemosIndexProps> = ({ onNavigate }) => {
  const demos = React.useMemo(() => demoRegistry.filter((demo) => demo.id !== 6), []);

  return (
    <div style={{ background: "var(--bg-primary)", minHeight: "calc(100vh - 72px)", padding: "4rem 2rem" }}>
      <div className="container" style={{ maxWidth: "960px" }}>
        {/* Header */}
        <div style={{ textAlign: "center", marginBottom: "4rem" }}>
          <span
            style={{
              fontSize: "0.85rem",
              fontWeight: 700,
              color: "var(--accent-router)",
              letterSpacing: "1.5px",
              textTransform: "uppercase",
              background: "rgba(168, 85, 247, 0.06)",
              padding: "0.3rem 0.8rem",
              borderRadius: "6px",
              border: "1px solid rgba(168, 85, 247, 0.2)",
              display: "inline-block",
              marginBottom: "1rem",
            }}
          >
            INTERACTIVE PROGRESSION LADDER
          </span>
          <h1
            style={{
              fontSize: "2.5rem",
              fontWeight: 700,
              color: "var(--text-primary)",
              letterSpacing: "-1px",
              marginBottom: "1rem",
            }}
          >
            Prove Forge Capabilities Step-by-Step
          </h1>
          <p
            style={{
              fontSize: "1.1rem",
              color: "var(--text-secondary)",
              maxWidth: "640px",
              margin: "0 auto",
              lineHeight: "1.6",
            }}
          >
            Forge features compile into a single structured dependency graph. Walk up the ladder from simple reactivity to composed forms, route projection, and time-travel branch merges.
          </p>
        </div>

        {/* Demo Ladder List */}
        <div style={{ display: "flex", flexDirection: "column", gap: "2rem", position: "relative" }}>
          {/* Vertical connecting line */}
          <div
            style={{
              position: "absolute",
              left: "40px",
              top: "30px",
              bottom: "30px",
              width: "2px",
              background: "linear-gradient(180deg, var(--accent-signals), var(--accent-forms), var(--accent-router), var(--accent-resources), var(--accent-composed), var(--accent-history))",
              zIndex: 1,
              opacity: 0.3,
            }}
          />

          {demos.map((demo) => {
            const accentColor = 
              demo.id === 1 ? "var(--accent-signals)" :
              demo.id === 2 ? "var(--accent-forms)" :
              demo.id === 3 ? "var(--accent-router)" :
              demo.id === 4 ? "var(--accent-resources)" :
              demo.id === 5 ? "var(--accent-composed)" :
              "var(--accent-history)";

            const difficultyBadge =
              demo.difficulty === "Beginner" ? "badge-signals" :
              demo.difficulty === "Intermediate" ? "badge-router" :
              "badge-history";

            return (
              <div
                key={demo.id}
                className="glass-panel"
                style={{
                  display: "flex",
                  gap: "2rem",
                  padding: "2rem",
                  position: "relative",
                  zIndex: 2,
                  background: "rgba(22, 28, 45, 0.65)",
                }}
              >
                {/* Visual Step Marker */}
                <div
                  style={{
                    width: "44px",
                    height: "44px",
                    borderRadius: "50%",
                    background: "var(--bg-darker)",
                    border: `2px solid ${accentColor}`,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontWeight: 700,
                    fontSize: "1.1rem",
                    color: accentColor,
                    boxShadow: `0 0 15px ${accentColor}33`,
                    flexShrink: 0,
                  }}
                >
                  {demo.id}
                </div>

                {/* Content */}
                <div style={{ flex: 1, display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                  <div style={{ display: "flex", alignItems: "center", gap: "0.75rem", flexWrap: "wrap" }}>
                    <h3 style={{ fontSize: "1.3rem", fontWeight: 600, color: "var(--text-primary)" }}>
                      {demo.title}
                    </h3>
                    <span className={`badge ${difficultyBadge}`}>{demo.difficulty}</span>
                  </div>

                  <p style={{ color: "var(--text-secondary)", fontSize: "0.95rem", lineHeight: "1.6" }}>
                    {demo.purpose}
                  </p>

                  <p style={{ color: accentColor, fontSize: "0.85rem", fontWeight: 600, marginTop: "0.25rem" }}>
                    → Key payoff: {demo.primaryMessage}
                  </p>

                  <div style={{ marginTop: "1rem", display: "flex", gap: "1rem" }}>
                    <button
                      className="btn"
                      onClick={() => onNavigate(`#/demos/${demo.id}`)}
                      style={{
                        background: `${accentColor}10`,
                        color: accentColor,
                        borderColor: `${accentColor}33`,
                        fontSize: "0.85rem",
                        padding: "0.5rem 1rem",
                      }}
                    >
                      Open Demo
                    </button>
                    <button
                      className="btn"
                      onClick={() => onNavigate(`#/docs/${demo.relatedDocsPath}`)}
                      style={{ fontSize: "0.85rem", padding: "0.5rem 1rem" }}
                    >
                      Read Related Docs
                    </button>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
