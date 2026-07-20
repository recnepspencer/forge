import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("demo routes use a compact, connected mobile pager", async () => {
  const css = await readFile(
    new URL("../src/ui/landingDemoRoute.css", import.meta.url),
    "utf8",
  );
  const component = await readFile(new URL("../src/ui/Demos.tsx", import.meta.url), "utf8");

  assert.match(css, /@media \(max-width: 900px\)/u);
  assert.match(
    css,
    /\.xai-demo-route-copy \.xai-eyebrow \{[^}]*align-self: start;[^}]*line-height: 1;/su,
  );
  assert.match(
    css,
    /\.xai-demo-route-nav-row \{[^}]*grid-column: 2;[^}]*grid-row: 1;[^}]*grid-template-columns: 2\.75rem minmax\(0, 1fr\) 2\.75rem;/su,
  );
  assert.match(
    css,
    /\.xai-demo-pager-button > span:not\(\[aria-hidden="true"\]\) \{[^}]*clip-path: inset\(50%\);/su,
  );
  assert.match(css, /\.xai-demo-route-count \{[^}]*grid-column: 2;[^}]*justify-content: space-between;/su);
  assert.match(css, /\.xai-demo-eyebrow-index,[^{]*\.xai-demo-count-wide \{[^}]*display: none;/su);
  assert.match(css, /\.xai-demo-count-compact \{[^}]*display: inline;/su);
  assert.match(component, /Demo<span className="xai-demo-eyebrow-index">/u);
  assert.match(component, /aria-current=\{id === demoId \? "page" : undefined\}/u);
});
