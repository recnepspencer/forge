import React from "react";
import { DEMO_ONE_CODE } from "../state/demoCodeSamples";

export const SIGNALS_CODE_SAMPLE = DEMO_ONE_CODE;

const KEYWORDS = new Set(["await", "const", "from", "import", "new", "return"]);

const TOKEN_PATTERN =
  /(\/\/[^\n]*)|("(?:[^"\\]|\\.)*")|(\b\d[\d_]*(?:\.\d+)?\b)|([A-Za-z_$][\w$]*)|(=>|[^\sA-Za-z_$]+)|(\s+)/g;

function classifyWord(word: string, nextChar: string): string {
  if (KEYWORDS.has(word)) return "kw";
  if (nextChar === "(") return "fn";
  return "var";
}

export function tokenizeCodeLine(line: string, lineKey: string): React.ReactNode[] {
  const nodes: React.ReactNode[] = [];
  let match: RegExpExecArray | null;
  let index = 0;
  const pattern = new RegExp(TOKEN_PATTERN.source, "g");

  while ((match = pattern.exec(line)) !== null) {
    const [text, comment, str, num, word, punct, space] = match;
    const key = `${lineKey}-${index}`;
    index += 1;

    if (comment !== undefined) {
      nodes.push(<span className="signals-tok signals-tok-comment" key={key}>{text}</span>);
    } else if (str !== undefined) {
      nodes.push(<span className="signals-tok signals-tok-str" key={key}>{text}</span>);
    } else if (num !== undefined) {
      nodes.push(<span className="signals-tok signals-tok-num" key={key}>{text}</span>);
    } else if (word !== undefined) {
      const rest = line.slice(pattern.lastIndex).match(/^\s*(.)/);
      const kind = classifyWord(text, rest?.[1] ?? "");
      nodes.push(<span className={`signals-tok signals-tok-${kind}`} key={key}>{text}</span>);
    } else if (punct !== undefined) {
      nodes.push(<span className="signals-tok signals-tok-op" key={key}>{text}</span>);
    } else if (space !== undefined) {
      nodes.push(<span key={key}>{text}</span>);
    }
  }

  return nodes;
}

interface SignalsCodeSampleProps {
  liveWhyLine: string | null;
}

export function SignalsCodeSample({ liveWhyLine }: SignalsCodeSampleProps): React.ReactElement {
  const [copied, setCopied] = React.useState(false);
  const lines = SIGNALS_CODE_SAMPLE.split("\n");

  const handleCopy = (): void => {
    void navigator.clipboard.writeText(SIGNALS_CODE_SAMPLE);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  };

  return (
    <figure className="signals-code-card">
      <figcaption className="signals-code-head">
        <span className="signals-code-dots" aria-hidden="true">
          <i /><i /><i />
        </span>
        <span className="signals-code-filename">transfer-decision.ts</span>
        <button onClick={handleCopy} type="button">{copied ? "Copied" : "Copy"}</button>
      </figcaption>
      <pre className="signals-code-block"><code>
        {lines.map((line, lineIndex) => (
          <span className="signals-code-line" key={lineIndex}>
            <span aria-hidden="true" className="signals-code-lineno">{lineIndex + 1}</span>
            <span className="signals-code-text">{tokenizeCodeLine(line, `l${lineIndex}`)}</span>
          </span>
        ))}
        {liveWhyLine ? (
          <span className="signals-code-line signals-code-line-live">
            <span aria-hidden="true" className="signals-code-lineno">→</span>
            <span className="signals-code-text">
              <span className="signals-tok signals-tok-live">{liveWhyLine}</span>
              <span className="signals-code-live-tag">live from this page</span>
            </span>
          </span>
        ) : null}
      </code></pre>
    </figure>
  );
}
