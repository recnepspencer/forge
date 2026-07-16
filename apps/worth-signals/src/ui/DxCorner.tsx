import React from "react";
import { tokenizeCodeLine } from "./SignalsSectionCodeSample";
import "./dxCorner.css";

export interface DxReceipt {
  claim: string;
  api: string;
}

interface DxCornerProps {
  code: string;
  filename: string;
  receipts: DxReceipt[];
  subtitle: string;
}

export function DxCorner({ code, filename, receipts, subtitle }: DxCornerProps): React.ReactElement {
  const [copied, setCopied] = React.useState(false);
  const lines = code.split("\n");

  const handleCopy = (): void => {
    void navigator.clipboard.writeText(code);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  };

  return (
    <section className="dx-corner" aria-label="Developer experience">
      <h2>You probably expected this to be harder</h2>
      <p className="dx-sub">{subtitle}</p>
      <ul className="dx-receipts">
        {receipts.slice(0, 3).map((receipt) => (
          <li key={receipt.api}>
            <p>{receipt.claim}</p>
            <code title={receipt.api}>{receipt.api}</code>
          </li>
        ))}
      </ul>
      <figure className="signals-code-card dx-code-card">
        <figcaption className="signals-code-head">
          <span className="signals-code-dots" aria-hidden="true">
            <i /><i /><i />
          </span>
          <span className="signals-code-filename">{filename}</span>
          <button onClick={handleCopy} type="button">{copied ? "Copied" : "Copy"}</button>
        </figcaption>
        <pre className="signals-code-block"><code>
          {lines.map((line, lineIndex) => (
            <span className="signals-code-line" key={lineIndex}>
              <span aria-hidden="true" className="signals-code-lineno">{lineIndex + 1}</span>
              <span className="signals-code-text">{tokenizeCodeLine(line, `l${lineIndex}`)}</span>
            </span>
          ))}
        </code></pre>
      </figure>
    </section>
  );
}
