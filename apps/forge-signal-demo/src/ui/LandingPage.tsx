import React, { useEffect, useRef, useState } from "react";

interface RevealProps {
  children: React.ReactNode;
}

const RevealSection: React.FC<RevealProps> = ({ children }) => {
  const ref = useRef<HTMLDivElement>(null);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setVisible(true);
          observer.unobserve(entry.target);
        }
      },
      { threshold: 0.1, rootMargin: "0px 0px -50px 0px" }
    );

    if (ref.current) {
      observer.observe(ref.current);
    }
    return () => observer.disconnect();
  }, []);

  return (
    <div ref={ref} className={`reveal ${visible ? "is-visible" : ""}`}>
      {children}
    </div>
  );
};

interface LandingPageProps {
  onNavigate: (path: string) => void;
}

export const LandingPage: React.FC<LandingPageProps> = ({ onNavigate }) => {
  return (
    <div style={{ background: "var(--bg-primary)" }}>
      {/* Dynamic Scroll Progress Glowing Bar */}
      <div
        style={{
          position: "fixed",
          top: 0,
          left: 0,
          right: 0,
          height: "3px",
          background: "linear-gradient(90deg, var(--accent-signals), var(--accent-router), var(--accent-resources))",
          zIndex: 1000,
        }}
      />

      {/* Hero Section */}
      <section
        style={{
          minHeight: "85vh",
          display: "flex",
          alignItems: "center",
          position: "relative",
          overflow: "hidden",
          borderBottom: "1px solid var(--border-light)",
        }}
      >
        <div className="container" style={{ position: "relative", zIndex: 2, padding: "4rem 2rem" }}>
          <div style={{ maxWidth: "780px" }}>
            <span
              style={{
                fontSize: "0.9rem",
                fontWeight: 700,
                color: "var(--accent-signals)",
                letterSpacing: "2px",
                textTransform: "uppercase",
                background: "rgba(6, 182, 212, 0.06)",
                padding: "0.3rem 0.8rem",
                borderRadius: "6px",
                border: "1px solid rgba(6, 182, 212, 0.2)",
                display: "inline-block",
                marginBottom: "1.5rem",
              }}
            >
              WASM-POWERED RECTIVITY FOR THE MODERN BROWSER
            </span>
            <h1
              style={{
                fontSize: "clamp(2.5rem, 6vw, 4rem)",
                fontWeight: 700,
                lineHeight: "1.1",
                letterSpacing: "-1.5px",
                color: "var(--text-primary)",
                marginBottom: "1.5rem",
              }}
            >
              One Primitives Block. <br />
              <span
                style={{
                  background: "linear-gradient(135deg, var(--accent-signals), var(--accent-router))",
                  WebkitBackgroundClip: "text",
                  WebkitTextFillColor: "transparent",
                }}
              >
                Infinite Built-in Capabilities.
              </span>
            </h1>
            <p
              style={{
                fontSize: "1.25rem",
                color: "var(--text-secondary)",
                lineHeight: "1.6",
                marginBottom: "2.5rem",
              }}
            >
              Forge replaces stacks of ad-hoc libraries (routers, form validators, cache engines, branch managers) with compiler-verified WebAssembly primitives. Auth once, scale endlessly, compile clean.
            </p>
            <div style={{ display: "flex", gap: "1.25rem", flexWrap: "wrap" }}>
              <button
                className="btn btn-primary"
                onClick={() => onNavigate("#/demos")}
                style={{
                  fontSize: "1rem",
                  padding: "0.8rem 1.8rem",
                  background: "linear-gradient(135deg, var(--accent-signals) 0%, #0891b2 100%)",
                  color: "white",
                  border: "none",
                  boxShadow: "0 4px 20px rgba(6, 182, 212, 0.3)",
                }}
              >
                Run Interactive Demos
              </button>
              <button
                className="btn"
                onClick={() => onNavigate("#/docs")}
                style={{ fontSize: "1rem", padding: "0.8rem 1.8rem" }}
              >
                Read Documentation
              </button>
            </div>
          </div>
        </div>

        {/* Ambient Hero Graphic */}
        <div
          style={{
            position: "absolute",
            right: "-10%",
            top: "5%",
            width: "60%",
            height: "90%",
            background: "radial-gradient(circle, rgba(168,85,247,0.06) 0%, transparent 60%)",
            borderRadius: "50%",
            pointerEvents: "none",
            zIndex: 1,
          }}
        />
      </section>

      {/* Feature Section List */}
      <div className="container" style={{ padding: "6rem 2rem", display: "flex", flexDirection: "column", gap: "8rem" }}>
        
        {/* SECTION 1: SIGNALS */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-signals)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-signals)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              01 / Core Reactive Layer
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Local Reactive State</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Forget complex context boilerplate or cascading state re-renders. Forge signals maintain a compiler-tracked dependency graph that executes reactively inside WASM at native memory speed.
            </p>
          </div>

          <div className="comparison-container">
            {/* Forge code */}
            <div className="glass-panel comparison-card">
              <div className="comparison-header">
                <span>FORGE PRIMITIVE</span>
                <div style={{ display: "flex", gap: "0.5rem" }}>
                  <button className="btn" style={{ padding: "0.2rem 0.5rem", fontSize: "0.75rem", background: "rgba(6, 182, 212, 0.15)", color: "var(--accent-signals)", borderColor: "transparent" }}>Real API</button>
                </div>
              </div>
              <div className="code-wrapper">
                <pre><code>{`import { createSignals } from "forge-signal-wasm";
const signals = await createSignals();

// Define input values
const count = signals.input(0);

// Derive reactive values automatically
const doubled = signals.computed("double", () => count.read() * 2);
const status = signals.computed("status", () => 
  count.read() >= 10 ? "Optimal Load" : "Bootstrap"
);`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Forge Leverage</strong>: Declare computed nodes once. The dependency graph handles generational evaluation. Components only re-render if the final outputs change.
              </div>
            </div>

            {/* Alternative code */}
            <div className="glass-panel comparison-card" style={{ borderColor: "rgba(255,255,255,0.05)" }}>
              <div className="comparison-header">
                <span>CONVENTIONAL REACT ALTERNATIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`import { useState, useMemo } from "react";

// Ad-hoc local state
const [count, setCount] = useState(0);

// Manual memoization hooks
const doubled = useMemo(() => count * 2, [count]);
const status = useMemo(() => 
  count >= 10 ? "Optimal Load" : "Bootstrap"
, [count]);`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Alternative Cost</strong>: Hard-coded dependencies in array wrappers. Parent components force recalculation of children unless aggressively wrapped in React.memo.
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>WHAT YOU GET OUT OF THIS SINGLE CODE BLOCK:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-signals)" }}>✓</span> Compiler-linked reactive tracking</div>
              <div className="capability-item"><span style={{ color: "var(--accent-signals)" }}>✓</span> Direct derived computed evaluations</div>
              <div className="capability-item"><span style={{ color: "var(--accent-signals)" }}>✓</span> Automatic dependency cleanups</div>
              <div className="capability-item"><span style={{ color: "var(--accent-signals)" }}>✓</span> Generational change tracking</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/1")} style={{ borderColor: "var(--accent-signals)" }}>Run Live Counter Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/learn/feature-index")}>Read Signals Docs</button>
            </div>
          </div>
        </RevealSection>

        {/* SECTION 2: FORMS */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-forms)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-forms)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              02 / UI State Layer
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Structured Form Controllers</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Forms are not just groups of strings. Forge models forms as structured state objects with built-in dirty validation, original source tracking, and reactive draft snapshots.
            </p>
          </div>

          <div className="comparison-container">
            {/* Forge code */}
            <div className="glass-panel comparison-card">
              <div className="comparison-header">
                <span>FORGE PRIMITIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`const source = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field("title", { validate: t => !t ? "Required" : null }),
    done: field("done"),
  }),
});

form.fields.title.set("Ship docs today");
console.log(form.draft());      // User edits
console.log(form.effective());  // Final calculated values
console.log(form.readiness());  // Validation status`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Forge Leverage</strong>: Instant multi-layer model resolution. Tracks raw source inputs, dynamic editing layers (drafts), validations, and readiness states automatically.
              </div>
            </div>

            {/* Alternative code */}
            <div className="glass-panel comparison-card" style={{ borderColor: "rgba(255,255,255,0.05)" }}>
              <div className="comparison-header">
                <span>CONVENTIONAL REACT ALTERNATIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`// Requires combining state hooks and custom objects
const [original, setOriginal] = useState({ title: "Ship docs", done: false });
const [draft, setDraft] = useState({ title: "Ship docs", done: false });
const [errors, setErrors] = useState({});

const isDirty = original.title !== draft.title || original.done !== draft.done;
const isValid = draft.title.trim() !== "";
const effective = { ...original, ...draft };`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Alternative Cost</strong>: High quantities of glue state logic. Requires wiring custom validation frameworks, hand-coded object diffs, and syncing lifecycle hooks.
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>WHAT YOU GET OUT OF THIS SINGLE CODE BLOCK:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-forms)" }}>✓</span> Three-layer state snapshots (Source/Draft/Effective)</div>
              <div className="capability-item"><span style={{ color: "var(--accent-forms)" }}>✓</span> Automatic field dirty diffing</div>
              <div className="capability-item"><span style={{ color: "var(--accent-forms)" }}>✓</span> Validation status indicators</div>
              <div className="capability-item"><span style={{ color: "var(--accent-forms)" }}>✓</span> Submit readiness and action posture checks</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/2")} style={{ borderColor: "var(--accent-forms)" }}>Run Live Form Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/forms/index")}>Read Forms Docs</button>
            </div>
          </div>
        </RevealSection>

        {/* SECTION 3: ROUTER */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-router)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-router)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              03 / Application Authority Layer
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Typed Route Navigation</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Routing is not just about mounting screens. Forge route declarations create compiler-typed parameters, navigation boundaries, and automatic breadcrumb histories natively.
            </p>
          </div>

          <div className="comparison-container">
            {/* Forge code */}
            <div className="glass-panel comparison-card">
              <div className="comparison-header">
                <span>FORGE PRIMITIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`const routes = signals.router.define({
  home: signals.router.route("/"),
  detail: signals.router.route("/items/:itemId", {
    breadcrumb: signals.router.breadcrumb({
      label: ({ params }) => \`Item \${params.itemId}\`
    })
  }),
});

// Compile-safe typed parameters
const itemRef = routes.detail.to({ params: { itemId: "item-7" } });
console.log(itemRef.href); // "/items/item-7"

const report = await routes.admit(itemRef.href);
console.log(report.outcome().kind); // "admitted" | "rejected"`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Forge Leverage</strong>: Highly secure typed parameters. Automatic breadcrumb labels, projection checks, and admission gates built directly into the route registry.
              </div>
            </div>

            {/* Alternative code */}
            <div className="glass-panel comparison-card" style={{ borderColor: "rgba(255,255,255,0.05)" }}>
              <div className="comparison-header">
                <span>CONVENTIONAL REACT ALTERNATIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`// Hardcoded string templates inside navigation links
<Link to={\`/items/\${itemId}\`}>Item Details</Link>

// Separate libraries for breadcrumb arrays
const crumbs = [
  { path: "/", label: "Home" },
  { path: \`/items/\${itemId}\`, label: \`Item \${itemId}\` }
];`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Alternative Cost</strong>: Path template values are typed as raw strings. Missing/incorrect parameter names fail silenty at runtime. Manual synchronization is required for labels.
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>WHAT YOU GET OUT OF THIS SINGLE CODE BLOCK:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-router)" }}>✓</span> Compile-verified navigation references</div>
              <div className="capability-item"><span style={{ color: "var(--accent-router)" }}>✓</span> Dynamic breadcrumb trail builders</div>
              <div className="capability-item"><span style={{ color: "var(--accent-router)" }}>✓</span> Route admission gating</div>
              <div className="capability-item"><span style={{ color: "var(--accent-router)" }}>✓</span> Navigation projection inspector models</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/3")} style={{ borderColor: "var(--accent-router)" }}>Run Live Router Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/router/index")}>Read Router Docs</button>
            </div>
          </div>
        </RevealSection>

        {/* SECTION 4: RESOURCES */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-resources)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-resources)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              04 / Server Coordination Layer
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Resource Lines & Coordination</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Fetching server records should not result in manual synchronization hacks. Forge Resource Lines manage loading, caching, reconciliation, and local patches out-of-the-box.
            </p>
          </div>

          <div className="comparison-container">
            {/* Forge code */}
            <div className="glass-panel comparison-card">
              <div className="comparison-header">
                <span>FORGE PRIMITIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`const api = signals.api({ baseUrl: "/api" });

const taskDetail = api.url("/tasks/:taskId").detail({
  load: ({ taskId }) => fetchTaskFromDatabase(taskId),
});

// Materialize a single live resource line
const line = taskDetail.line({ taskId: "t-4" });

console.log(line.summary()); // status: "loading" | "settled"
console.log(line.value());   // loaded record data`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Forge Leverage</strong>: Declare coordination logic once. Automatic line-level data caching, reactive network updates, and unified server response reconciliation.
              </div>
            </div>

            {/* Alternative code */}
            <div className="glass-panel comparison-card" style={{ borderColor: "rgba(255,255,255,0.05)" }}>
              <div className="comparison-header">
                <span>CONVENTIONAL REACT ALTERNATIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`import { useQuery } from "@tanstack/react-query";

const { data, isLoading } = useQuery({
  queryKey: ["task", taskId],
  queryFn: () => fetchTaskFromDatabase(taskId),
});

// Requires manual context providers for caching parameters
// Requires custom useEffect hooks to resolve initial loads`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Alternative Cost</strong>: Relies on external caching infrastructures. Zero semantic understanding of query structures, requiring separate plugins for mutation syncing.
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>WHAT YOU GET OUT OF THIS SINGLE CODE BLOCK:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-resources)" }}>✓</span> Dynamic line-level cache bindings</div>
              <div className="capability-item"><span style={{ color: "var(--accent-resources)" }}>✓</span> Built-in settled/pending state wrappers</div>
              <div className="capability-item"><span style={{ color: "var(--accent-resources)" }}>✓</span> Mutation response auto-reconciliation</div>
              <div className="capability-item"><span style={{ color: "var(--accent-resources)" }}>✓</span> Speculative local patch capabilities</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/4")} style={{ borderColor: "var(--accent-resources)" }}>Run Live Resource Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/resources/index")}>Read Resources Docs</button>
            </div>
          </div>
        </RevealSection>

        {/* SECTION 5: HISTORY */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-history)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-history)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              05 / Time Travel & Branching Layer
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Built-in Replay & Branching</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Time should be a first-class state dimension. Forge history surfaces record application snapshots natively, allowing users to fork execution paths and revert state safely.
            </p>
          </div>

          <div className="comparison-container">
            {/* Forge code */}
            <div className="glass-panel comparison-card">
              <div className="comparison-header">
                <span>FORGE PRIMITIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`// Access built-in history models on the active runtime
const history = runtime.history();

// Create isolated "what-if" feature branch
const featBranch = history.createBranch("what-if");

// Swapping branches swaps the entire application state!
history.switchBranch(featBranch.id);
titleInput.set("Feature Draft");

// plan detailed merges mathematically with proof validation!
const mergePlan = history.planMergeBranchesDetailed(
  featBranch.id, 
  mainBranch.id
);`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Forge Leverage</strong>: Git-like branching capabilities built directly into the UI state engine. Swapping branches swaps all inputs, computed states, and forms.
              </div>
            </div>

            {/* Alternative code */}
            <div className="glass-panel comparison-card" style={{ borderColor: "rgba(255,255,255,0.05)" }}>
              <div className="comparison-header">
                <span>CONVENTIONAL REACT ALTERNATIVE</span>
              </div>
              <div className="code-wrapper">
                <pre><code>{`// Requires massive manual reducer trees
const [history, setHistory] = useState([initialState]);
const [currentIndex, setCurrentIndex] = useState(0);

// Branching requires duplicating massive object schemas
// and tracking complex parent-child references manually
const forkState = { ...currentState, parentSnapshotId: 4 };`}</code></pre>
              </div>
              <div className="comparison-analysis">
                <strong>Alternative Cost</strong>: Hand-rolled history chains are extremely memory intensive and prone to memory leaks. Zero capabilities for planning or dry-running complex state merges.
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>WHAT YOU GET OUT OF THIS SINGLE CODE BLOCK:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-history)" }}>✓</span> Multi-branch application execution states</div>
              <div className="capability-item"><span style={{ color: "var(--accent-history)" }}>✓</span> Time-travel state restore and undo/redo</div>
              <div className="capability-item"><span style={{ color: "var(--accent-history)" }}>✓</span> Full compiler-verified state merge plan exports</div>
              <div className="capability-item"><span style={{ color: "var(--accent-history)" }}>✓</span> Non-destructive time-series auditing</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/6")} style={{ borderColor: "var(--accent-history)" }}>Run Live History Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/resources/branch-native-effects")}>Read History Docs</button>
            </div>
          </div>
        </RevealSection>

        {/* SECTION 6: COMPOSED WORKFLOWS */}
        <RevealSection>
          <div style={{ borderLeft: "4px solid var(--accent-composed)", paddingLeft: "1.5rem" }}>
            <span style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--accent-composed)", letterSpacing: "1.5px", textTransform: "uppercase" }}>
              06 / Composition & Orchestration
            </span>
            <h2 style={{ fontSize: "2rem", fontWeight: 700, margin: "0.5rem 0 1rem 0" }}>Zero-Glue Composed Workflows</h2>
            <p style={{ color: "var(--text-secondary)", maxWidth: "720px", marginBottom: "2rem" }}>
              Primitives should not exist in isolation. Forge router, forms, and resources automatically connect together, creating fluid edit workflows without manual context bridging or sync wiring.
            </p>
          </div>

          <div
            className="glass-panel"
            style={{
              padding: "2.5rem",
              borderLeft: "4px solid var(--accent-composed)",
              margin: "2rem 0",
              background: "rgba(236, 72, 153, 0.02)",
            }}
          >
            <h3 style={{ fontSize: "1.2rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>
              Compose Multiple Layers of App Behavior Effortlessly
            </h3>
            <p style={{ color: "var(--text-secondary)", lineHeight: "1.6", marginBottom: "1.5rem" }}>
              Normally, to build a **Task Detail page → Edit Route → Resource-backed Draft Form → Dirty check leave confirmation** workflow, a developer must stitch together React Router navigation blockers, React Query mutation states, Formik state caches, and several ad-hoc `useEffect` checks.
            </p>
            <p style={{ color: "var(--text-secondary)", lineHeight: "1.6", marginBottom: "1.5rem" }}>
              With Forge, you simply configure **one route-coupled resource-backed form**. The router automatically warms up the resource line, the form binds to the line's values, and the route admission gate intercepts unsaved edits to protect drafts automatically.
            </p>
            
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "1fr",
                gap: "1.5rem",
                marginTop: "2rem",
              }}
            >
              <div
                style={{
                  background: "var(--bg-secondary)",
                  padding: "1.25rem 1.5rem",
                  borderRadius: "8px",
                  border: "1px solid var(--border-light)",
                }}
              >
                <div style={{ display: "flex", alignItems: "center", gap: "0.75rem", marginBottom: "0.5rem" }}>
                  <span className="badge badge-composed">Unified Execution</span>
                  <span style={{ fontWeight: 600, color: "var(--text-primary)" }}>Task Edit Workflow</span>
                </div>
                <pre style={{ fontSize: "0.8rem", color: "#f472b6" }}><code>{`// A form backed natively by a route-aware resource line
const taskForm = signals.form({
  source: taskResourceLine.toSource(), // Unified bindings!
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  })
});`}</code></pre>
              </div>
            </div>
          </div>

          {/* Capabilities */}
          <div style={{ marginTop: "2rem" }}>
            <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>THE COMPILED PAYOFF:</h3>
            <div className="capability-list">
              <div className="capability-item"><span style={{ color: "var(--accent-composed)" }}>✓</span> Zero ad-hoc synchronization bugs</div>
              <div className="capability-item"><span style={{ color: "var(--accent-composed)" }}>✓</span> Automatic viewport warmups</div>
              <div className="capability-item"><span style={{ color: "var(--accent-composed)" }}>✓</span> Draft preservation through back/forward actions</div>
              <div className="capability-item"><span style={{ color: "var(--accent-composed)" }}>✓</span> Multi-layer compiler protection out-of-the-box</div>
            </div>
            <div style={{ marginTop: "1.5rem", display: "flex", gap: "1rem" }}>
              <button className="btn" onClick={() => onNavigate("#/demos/5")} style={{ borderColor: "var(--accent-composed)" }}>Run Live Composed Demo</button>
              <button className="btn" onClick={() => onNavigate("#/docs/forms/route-coupling")}>Read Composition Docs</button>
            </div>
          </div>
        </RevealSection>

      </div>
    </div>
  );
};
