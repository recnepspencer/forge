#!/usr/bin/env node
/**
 * Forge icon downloader — equivalent to the web app's add-icon.js
 *
 * Downloads SVG icons from Lucide (https://lucide.dev) into the egui icon
 * folder and updates the FgIcon enum in forge-ui-components.
 *
 * Usage:
 *   node scripts/add-icon.js cube pencil-line chat-bubble trash-2
 *
 * What it does:
 *   1. Downloads each icon as an SVG from the Lucide CDN
 *   2. Saves to crates/forge-ui-components/icons/<name>.svg
 *   3. Appends new variants to the FgIcon enum in icons.rs
 *   4. Appends new match arms to the FgIcon::path() function
 */

const https = require("https");
const fs = require("fs");
const path = require("path");

const ICONS_DIR = path.join(
  __dirname,
  "..",
  "crates",
  "forge-ui-components",
  "icons",
);
const ICONS_RS = path.join(
  __dirname,
  "..",
  "crates",
  "forge-ui-components",
  "src",
  "icons.rs",
);
// unpkg serves exact package contents — most reliable for icon lookup
const CDN_BASE = "https://unpkg.com/lucide-static@latest/icons";

// ── Helpers ──────────────────────────────────────────────────────────────────

function toRustVariant(name) {
  // "trash-2" → "Trash2", "pencil-line" → "PencilLine"
  return name
    .split("-")
    .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
    .join("");
}

function download(url) {
  return new Promise((resolve, reject) => {
    const parsedUrl = new URL(url);
    const transport =
      parsedUrl.protocol === "https:" ? require("https") : require("http");
    transport
      .get(url, (res) => {
        if (
          res.statusCode === 301 ||
          res.statusCode === 302 ||
          res.statusCode === 307
        ) {
          const location = res.headers.location;
          // Resolve relative redirects against the original URL
          const next = location.startsWith("http")
            ? location
            : new URL(location, url).toString();
          return download(next).then(resolve).catch(reject);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
        }
        const chunks = [];
        res.on("data", (c) => chunks.push(c));
        res.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
        res.on("error", reject);
      })
      .on("error", reject);
  });
}

// ── Parse existing enum to find already-registered icons ─────────────────────

function readExistingIcons(content) {
  const existing = new Set();
  const re = /FgIcon::(\w+)\s*=>/g;
  let m;
  while ((m = re.exec(content)) !== null) {
    existing.add(m[1]);
  }
  return existing;
}

// ── Main ─────────────────────────────────────────────────────────────────────

async function main() {
  const args = process.argv.slice(2);
  if (args.length === 0) {
    console.log("Usage: node scripts/add-icon.js <icon-name> [icon-name2] ...");
    console.log("\nExample: node scripts/add-icon.js cube pencil-line trash-2");
    console.log("\nBrowse available icons at: https://lucide.dev/icons");
    process.exit(1);
  }

  fs.mkdirSync(ICONS_DIR, { recursive: true });

  let iconsContent = fs.readFileSync(ICONS_RS, "utf8");
  const existing = readExistingIcons(iconsContent);
  const newIcons = [];

  for (const name of args) {
    const variant = toRustVariant(name);

    // Download SVG
    const url = `${CDN_BASE}/${name}.svg`;
    process.stdout.write(`Downloading ${name}.svg … `);
    let svg;
    try {
      svg = await download(url);
    } catch (e) {
      console.error(`FAILED: ${e.message}`);
      process.exit(1);
    }
    const dest = path.join(ICONS_DIR, `${name}.svg`);
    fs.writeFileSync(dest, svg, "utf8");
    console.log("✓");

    if (existing.has(variant)) {
      console.log(
        `  → FgIcon::${variant} already registered, skipping enum update`,
      );
      continue;
    }
    newIcons.push({ name, variant });
  }

  if (newIcons.length === 0) {
    console.log("\nAll icons already registered. SVG files updated on disk.");
    return;
  }

  // ── Inject new enum variants ──────────────────────────────────────────────
  // Find the closing `}` of the enum block:
  //   pub enum FgIcon {
  //     ...
  //   }   ← insert before this
  const enumEnd = iconsContent.indexOf("\n}\n\nimpl FgIcon");
  if (enumEnd === -1) {
    console.error(
      "Could not locate end of FgIcon enum. Edit icons.rs manually.",
    );
    process.exit(1);
  }

  const variantLines = newIcons
    .map(({ variant }) => `    ${variant},`)
    .join("\n");
  iconsContent =
    iconsContent.slice(0, enumEnd) +
    "\n" +
    variantLines +
    iconsContent.slice(enumEnd);

  // ── Inject new svg_bytes() match arms ────────────────────────────────────
  // Anchor: the fallthrough `_ => None,` arm of svg_bytes()
  const pathMatchEnd = iconsContent.indexOf(
    "            _ => None,\n        }\n    }\n",
  );
  if (pathMatchEnd === -1) {
    console.error(
      "Could not locate svg_bytes() match in icons.rs. Edit manually.",
    );
    // Still write the enum changes
    fs.writeFileSync(ICONS_RS, iconsContent, "utf8");
    process.exit(1);
  }

  const armLines = newIcons
    .map(
      ({ name, variant }) =>
        `            FgIcon::${variant} => Some(include_bytes!("../icons/${name}.svg")),`,
    )
    .join("\n");

  iconsContent =
    iconsContent.slice(0, pathMatchEnd) +
    "\n" +
    armLines +
    "\n" +
    iconsContent.slice(pathMatchEnd);

  fs.writeFileSync(ICONS_RS, iconsContent, "utf8");

  console.log(
    `\n✅ Registered ${newIcons.length} new icon(s): ${newIcons.map((i) => `FgIcon::${i.variant}`).join(", ")}`,
  );
  console.log(`\nSVG files are in: crates/forge-ui-components/icons/`);
  console.log(`Enum updated in:  crates/forge-ui-components/src/icons.rs`);
  console.log(
    `\nNext: call icon_store.load() at app start, then draw with draw_icon(ui, FgIcon::${newIcons[0].variant}, 20.0)`,
  );
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
