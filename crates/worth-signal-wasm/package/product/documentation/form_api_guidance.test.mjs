import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..");
const docsDir = path.join(crateDir, "docs");
const formApiPath = path.join(crateDir, "docs", "api-reference", "forms.md");

test("the Form API reference recommends a construction instead of dumping the controller", async () => {
  const formApi = await readFile(formApiPath, "utf8");

  assert.match(formApi, /Most forms should be straightforward to construct/u);
  assert.match(formApi, /## The Construction We Recommend/u);
  assert.match(formApi, /profile\/profile\.validation\.ts/u);
  assert.match(formApi, /const profileRules =/u);
  assert.match(formApi, /signals\.form\.define/u);
  assert.match(formApi, /signals\.form\.source\.resourceLine\(profileLine/u);
  assert.match(formApi, /submit: submit\(\)/u);
  assert.match(formApi, /validation: \(\{ field \}\) =>/u);
  assert.match(formApi, /profileRules\.email/u);
  assert.match(formApi, /profileRules\.seats/u);
  assert.match(formApi, /## Where Client Validation Actually Runs/u);
  assert.match(formApi, /There is no separate `validate\(\)` call/u);
  assert.match(formApi, /field\.messages/u);
  assert.match(formApi, /editor\.submit\.disabled/u);
  assert.match(formApi, /form\.bindInput/u);
  assert.match(formApi, /form\.actionReadiness/u);
  assert.match(formApi, /## Use The Form Components You Already Have/u);
  assert.match(formApi, /Worth does\s+not prescribe their markup/u);
  assert.match(formApi, /## Put Form Behavior In A Hook/u);
  assert.match(formApi, /async function save/u);
  assert.match(formApi, /callback identity is not form state/u);
  assert.match(formApi, /await submit\.execute\(\)/u);
  assert.match(formApi, /## Then Components Stay Small/u);
  assert.match(formApi, /useSignalsForm\(profileForm\)/u);
  assert.match(formApi, /const editor = useProfileEditor\(\)/u);
  assert.match(formApi, /<TextInput/u);
  assert.match(formApi, /<NumberInput/u);
  assert.match(formApi, /<SubmitButton action=\{editor\.submit\}>/u);
  assert.match(formApi, /const submit = form\.action\("submit"\)/u);
  assert.match(formApi, /There is no `useState`/u);
  assert.match(formApi, /There is no `form\.submit\(\)` method/u);
  assert.match(formApi, /every added subsystem earn its place/u);
  assert.match(formApi, /Complete Form Export Catalog/u);

  assert.doesNotMatch(formApi, /## Core Controller Reads/u);
  assert.doesNotMatch(formApi, /Declaration keys:/u);
  assert.doesNotMatch(formApi, /fetch\("\/api\/profile"/u);
  assert.doesNotMatch(formApi, /form\.controller\.(fulfillAction|rejectAction)/u);
  assert.doesNotMatch(formApi, /satisfies Parameters<typeof signals\.form>/u);
  assert.doesNotMatch(formApi, /AppField/u);
  assert.doesNotMatch(formApi, /useCallback/u);
  assert.doesNotMatch(formApi, /function (TextInput|NumberInput|TextareaInput|CheckboxInput)/u);
});

test("public docs never teach a catch-all AppField abstraction", async () => {
  const relativePaths = await readdir(docsDir, { recursive: true });
  const markdownPaths = relativePaths.filter((entry) => entry.endsWith(".md"));
  const offenders = [];

  for (const relativePath of markdownPaths) {
    const content = await readFile(path.join(docsDir, relativePath), "utf8");
    if (/AppField/u.test(content)) {
      offenders.push(relativePath);
    }
  }

  assert.deepEqual(offenders, []);
});
