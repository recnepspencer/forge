import React from "react";
import { tokenizeCodeLine } from "./SignalsSectionCodeSample";
import "./signalsSection.css";

export const RESOURCE_LINES_CODE_SAMPLE = `const api = signals.api({ baseUrl: "/api/storefront" });

const product = api.url("/products/:productId").detail({
  reconcile: signals.resource.detailFields({
    price: { read: (v) => v.price, write: (v, price) => ({ ...v, price }) },
  }),
  load: ({ productId }) => fetchProduct(productId),
});

const line = product.line({ productId: "p-204" });

// the server pushes new truth — no user action involved
line.deliver(product.delivery.field({
  packetId: "pkt-08", basisId: "srv-v1", nextBasisId: "srv-v2",
  field: "price", value: 188,
}));

// the line kept the receipt
line.diagnostics().lastEffect.provenance;`;

interface ResourceLinesCodeSampleProps {
  liveLine: string | null;
}

export function ResourceLinesSectionCodeSample({ liveLine }: ResourceLinesCodeSampleProps): React.ReactElement {
  const [copied, setCopied] = React.useState(false);
  const lines = RESOURCE_LINES_CODE_SAMPLE.split("\n");

  const handleCopy = (): void => {
    void navigator.clipboard.writeText(RESOURCE_LINES_CODE_SAMPLE);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1800);
  };

  return (
    <figure className="signals-code-card">
      <figcaption className="signals-code-head">
        <span className="signals-code-dots" aria-hidden="true">
          <i /><i /><i />
        </span>
        <span className="signals-code-filename">product-line.ts</span>
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
