import React from "react";
import { DEMO_FOUR_CODE } from "../state/demoCodeSamples";
import { tokenizeCodeLine } from "./SignalsSectionCodeSample";
import "./signalsSection.css";

export const ROUTER_CODE_SAMPLE = DEMO_FOUR_CODE;

interface RouterCodeSampleProps {
  liveLine: string | null;
}

export function RouterSectionCodeSample({ liveLine }: RouterCodeSampleProps): React.ReactElement {
  const [copied, setCopied] = React.useState(false);
  const lines = ROUTER_CODE_SAMPLE.split("\n");

  const handleCopy = (): void => {
    void navigator.clipboard.writeText(ROUTER_CODE_SAMPLE);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  };

  return (
    <figure className="signals-code-card">
      <figcaption className="signals-code-head">
        <span className="signals-code-dots" aria-hidden="true">
          <i /><i /><i />
        </span>
        <span className="signals-code-filename">step-admission.ts</span>
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
