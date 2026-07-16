import React from "react";
import { tokenizeCodeLine } from "./SignalsSectionCodeSample";
import "./signalsSection.css";

export const RESOURCES_CODE_SAMPLE = `const po = signals.api({
  baseUrl: "/api/procurement",
  effects: signals.resource.effects.branchNative(),
});

const poLines = po.url("/orders/:orderId/lines")
  .response(signals.resource.response.array({ itemId: (line) => line.id }))
  .list({ load: ({ orderId }) => client.fetchLines(orderId) });

const saveLine = po.url("/orders/:orderId/lines/:lineId")
  .response(signals.resource.response.detail<PoLine>()({ label: "label", sync: "sync" }))
  .update({
    // a confirmation replaces one item — it cannot clobber the rest of the screen
    reconciles: [{ family: poLines, params: ({ orderId }) => ({ orderId }),
      collection: { kind: "item" }, fallback: "partialReconciliation" }],
    load: ({ orderId, lineId, body }) => client.saveLine(orderId, lineId, body),
  });

// every optimistic patch is an admitted effect, not a cache overwrite
line.patch(poLines.patch.insert({ itemId, placement: "append", nextItem }));
line.diagnostics().lastEffect;  // provenance, confirmation, rollback posture
line.history().lifecycle;       // the record of what the screen showed, and when`;

interface ResourcesCodeSampleProps {
  liveLine: string | null;
}

export function ResourcesSectionCodeSample({ liveLine }: ResourcesCodeSampleProps): React.ReactElement {
  const [copied, setCopied] = React.useState(false);
  const lines = RESOURCES_CODE_SAMPLE.split("\n");

  const handleCopy = (): void => {
    void navigator.clipboard.writeText(RESOURCES_CODE_SAMPLE);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  };

  return (
    <figure className="signals-code-card">
      <figcaption className="signals-code-head">
        <span className="signals-code-dots" aria-hidden="true">
          <i /><i /><i />
        </span>
        <span className="signals-code-filename">po-lines.ts</span>
        <button onClick={handleCopy} type="button">{copied ? "Copied" : "Copy"}</button>
      </figcaption>
      <pre className="signals-code-block"><code>
        {lines.map((line, lineIndex) => (
          <span className="signals-code-line" key={lineIndex}>
            <span aria-hidden="true" className="signals-code-lineno">{lineIndex + 1}</span>
            <span className="signals-code-text">{tokenizeCodeLine(line, `l${lineIndex}`)}</span>
          </span>
        ))}
        {liveLine ? (
          <span className="signals-code-line signals-code-line-live">
            <span aria-hidden="true" className="signals-code-lineno">→</span>
            <span className="signals-code-text">
              <span className="signals-tok signals-tok-live">{liveLine}</span>
              <span className="signals-code-live-tag">live from this page</span>
            </span>
          </span>
        ) : null}
      </code></pre>
    </figure>
  );
}
