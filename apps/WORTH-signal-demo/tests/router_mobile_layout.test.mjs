import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("Demo 4 keeps mobile controls, portal, and audit consequence connected", async () => {
  const section = await readFile(new URL("../src/ui/RouterSection.tsx", import.meta.url), "utf8");
  const mobileCss = await readFile(new URL("../src/ui/routerSectionMobile.css", import.meta.url), "utf8");

  const roleRow = section.indexOf('className="mfg-role-row"');
  const stage = section.indexOf('<section className="mfg-stage"');
  const browser = section.indexOf("<RouterSectionBrowserSurface");
  const audit = section.indexOf('className="mfg-log-panel"');

  assert.ok(roleRow >= 0);
  assert.ok(stage > roleRow);
  assert.ok(browser > stage);
  assert.ok(audit > browser);
  assert.match(section, /import "\.\/routerSectionMobile\.css";/u);
  assert.match(mobileCss, /\.mfg-role-row \{[^}]*margin-bottom: -1rem;/su);
  assert.match(mobileCss, /\.mfg-stage \{\s*gap: 0;/su);
  assert.match(mobileCss, /\.mfg-browser-nav \{[^}]*grid-template-columns: repeat\(2, minmax\(0, 1fr\)\);/su);
  assert.match(mobileCss, /\.mfg-browser-nav button \{[^}]*min-height: 2\.75rem;/su);
  assert.match(mobileCss, /\.mfg-browser-body \{[^}]*min-height: 0;/su);
  assert.match(mobileCss, /\.mfg-log-panel \{[^}]*border-top: 0;[^}]*border-radius: 0 0 14px 14px;/su);
  assert.match(mobileCss, /\.mfg-execute-button,\s*\.mfg-deviation-button \{[^}]*width: 100%;/su);
});
