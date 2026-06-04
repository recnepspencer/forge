import React from "react";
import { DemoPreface } from "./DemoPreface";
import "./signalsSection.css";

interface SignalsSectionProps {
  onNavigate: (path: string) => void;
}

function CodeLine({ children }: { children?: React.ReactNode }): React.ReactElement {
  return <div className="signals-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: React.ReactNode;
}): React.ReactElement {
  return <span className={`tok tok-${kind}`}>{children}</span>;
}

export function SignalsSection({ onNavigate }: SignalsSectionProps): React.ReactElement {
  const [title, setTitle] = React.useState("New launch plan");

  const slug = React.useMemo(
    () => title.toLowerCase().trim().replace(/\s+/g, "-"),
    [title],
  );
  const publishDisabled = title.trim().length === 0;
  const summary = publishDisabled ? "Needs title" : "Ready to publish";

  return (
    <div className="xai-section-band accent-signals">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">01 / Signals</span>
        <h2>A single change flows through one graph.</h2>
        <p>
          Forge starts with retained reactive truth. Inputs and derived values live
          in one runtime graph, so one mutation produces one coherent propagation
          path instead of a chain of hand-managed recalculations.
        </p>
      </div>

      <DemoPreface demoId={1} />

      <div className="signals-teach-grid">
        <article className="signals-code-card">
          <div className="signals-card-topline">
            <span>Signals authoring</span>
          </div>
          <h3>Define one input and derive everything else.</h3>
          <p>
            One input drives three derived values, and the runtime keeps the
            propagation path coherent.
          </p>
          <div className="signals-code-block" role="presentation">
            <CodeLine>
              <Tok kind="kw">const</Tok> <Tok kind="var">title</Tok> <Tok kind="op">=</Tok>{" "}
              <Tok kind="plain">signals</Tok>.<Tok kind="fn">input</Tok>(
              <Tok kind="str">"New launch plan"</Tok>);
            </CodeLine>
            <CodeLine />
            <CodeLine>
              <Tok kind="kw">const</Tok> <Tok kind="var">slug</Tok> <Tok kind="op">=</Tok>{" "}
              <Tok kind="plain">signals</Tok>.<Tok kind="fn">computed</Tok>(
              <Tok kind="str">"slug"</Tok>, <Tok kind="op">() =&gt;</Tok>
            </CodeLine>
            <CodeLine>
              {"  "}
              <Tok kind="var">title</Tok>.<Tok kind="fn">read</Tok>().<Tok kind="fn">toLowerCase</Tok>()
              .<Tok kind="fn">replace</Tok>(<Tok kind="str">/\\s+/g</Tok>, <Tok kind="str">"-"</Tok>),
            </CodeLine>
            <CodeLine>
              <Tok kind="plain">);</Tok>
            </CodeLine>
            <CodeLine />
            <CodeLine>
              <Tok kind="kw">const</Tok> <Tok kind="var">publishDisabled</Tok> <Tok kind="op">=</Tok>{" "}
              <Tok kind="plain">signals</Tok>.<Tok kind="fn">computed</Tok>(
              <Tok kind="str">"publishDisabled"</Tok>, <Tok kind="op">() =&gt;</Tok>
            </CodeLine>
            <CodeLine>
              {"  "}
              <Tok kind="var">title</Tok>.<Tok kind="fn">read</Tok>().<Tok kind="fn">trim</Tok>()
              .<Tok kind="prop">length</Tok> <Tok kind="op">===</Tok> <Tok kind="plain">0</Tok>,
            </CodeLine>
            <CodeLine>
              <Tok kind="plain">);</Tok>
            </CodeLine>
            <CodeLine />
            <CodeLine>
              <Tok kind="kw">const</Tok> <Tok kind="var">summary</Tok> <Tok kind="op">=</Tok>{" "}
              <Tok kind="plain">signals</Tok>.<Tok kind="fn">computed</Tok>(
              <Tok kind="str">"summary"</Tok>, <Tok kind="op">() =&gt;</Tok>
            </CodeLine>
            <CodeLine>
              {"  "}
              <Tok kind="var">publishDisabled</Tok>.<Tok kind="fn">read</Tok>() <Tok kind="op">?</Tok>{" "}
              <Tok kind="str">"Needs title"</Tok> <Tok kind="op">:</Tok>{" "}
              <Tok kind="str">"Ready to publish"</Tok>,
            </CodeLine>
            <CodeLine>
              <Tok kind="plain">);</Tok>
            </CodeLine>
          </div>
        </article>

        <article className="signals-demo-card">
          <div className="signals-card-topline">
            <span>Mini demo</span>
          </div>
          <form className="signals-inline-form" onSubmit={(event) => event.preventDefault()}>
            <label className="signals-input-card">
              <span>Title</span>
              <input
                className="signals-input"
                onChange={(event) => setTitle(event.target.value)}
                placeholder="New launch plan"
                type="text"
                value={title}
              />

              <div className="signals-submit-row">
                <button className="signals-publish-button" disabled={publishDisabled} type="submit">
                  Publish
                </button>
              </div>
            </label>
          </form>

          <div className="signals-derived-grid">
            <div className="signals-derived-card">
              <strong>Derived</strong>
              <span>slug</span>
              <code>{slug || "(empty)"}</code>
            </div>
            <div className="signals-derived-card">
              <strong>Derived</strong>
              <span>publishDisabled</span>
              <code>{String(publishDisabled)}</code>
            </div>
            <div className="signals-derived-card">
              <strong>Derived</strong>
              <span>summary</span>
              <code>{summary}</code>
            </div>
          </div>
        </article>
      </div>

      <div className="signals-compare-strip">
        <article className="xai-compare-card xai-compare-card-typical">
          <span>Without Forge</span>
          <h4>Manual propagation discipline</h4>
          <ul>
            <li>Update local state</li>
            <li>Remember dependent recalculations</li>
            <li>Protect against stale derived values</li>
            <li>Rely on memo and effect discipline</li>
          </ul>
        </article>

        <article className="xai-compare-card xai-compare-card-forge">
          <span>With Forge</span>
          <h4>One runtime-owned propagation path</h4>
          <ul>
            <li>Update one input</li>
            <li>The runtime resolves dependent values</li>
            <li>Subscribers receive final stable truth</li>
            <li>No manual propagation choreography</li>
          </ul>
        </article>
      </div>

      <div className="signals-cta-row">
        <div className="signals-cta-copy">See one mutation propagate live through the runtime.</div>
        <div className="xai-section-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/1")} type="button">
            Open signals demo
          </button>
          <button
            className="xai-button xai-button-secondary"
            onClick={() => onNavigate("#/docs/learn/feature-index")}
            type="button"
          >
            Read signals docs
          </button>
        </div>
      </div>
    </div>
  );
}
