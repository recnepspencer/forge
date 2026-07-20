import manifestSource from "../../../../crates/worth-signal-wasm/docs/metadata/public-documentation.json?raw";

export type PublicDocKind =
  | "concept"
  | "guide"
  | "landing"
  | "reference"
  | "troubleshooting"
  | "tutorial";

export interface PublicDocManifestItem {
  kind: PublicDocKind;
  path: string;
  title: string;
}

export interface PublicDocManifestSection {
  id: string;
  items: PublicDocManifestItem[];
  title: string;
}

export interface PublicDocRedirect {
  from: string;
  to: string;
}

export interface PublicDocsManifest {
  product: string;
  redirects: PublicDocRedirect[];
  schemaVersion: number;
  sections: PublicDocManifestSection[];
}

const publicDocKinds = new Set<PublicDocKind>([
  "concept",
  "guide",
  "landing",
  "reference",
  "troubleshooting",
  "tutorial",
]);

function requireRecord(value: unknown, context: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${context} must be an object`);
  }
  return value as Record<string, unknown>;
}

function requireString(value: unknown, context: string): string {
  if (typeof value !== "string" || value.trim() === "") {
    throw new TypeError(`${context} must be a non-empty string`);
  }
  return value;
}

function parseItem(value: unknown, sectionId: string, index: number): PublicDocManifestItem {
  const item = requireRecord(value, `documentation item ${sectionId}[${index}]`);
  const kind = requireString(item.kind, `documentation item ${sectionId}[${index}].kind`);
  if (!publicDocKinds.has(kind as PublicDocKind)) {
    throw new TypeError(`documentation item ${sectionId}[${index}] has unsupported kind ${kind}`);
  }
  return {
    kind: kind as PublicDocKind,
    path: requireString(item.path, `documentation item ${sectionId}[${index}].path`),
    title: requireString(item.title, `documentation item ${sectionId}[${index}].title`),
  };
}

function parseSection(value: unknown, index: number): PublicDocManifestSection {
  const section = requireRecord(value, `documentation section ${index}`);
  const id = requireString(section.id, `documentation section ${index}.id`);
  if (!Array.isArray(section.items) || section.items.length === 0) {
    throw new TypeError(`documentation section ${id} must contain at least one item`);
  }
  return {
    id,
    items: section.items.map((item, itemIndex) => parseItem(item, id, itemIndex)),
    title: requireString(section.title, `documentation section ${index}.title`),
  };
}

function parseRedirect(value: unknown, index: number): PublicDocRedirect {
  const redirect = requireRecord(value, `documentation redirect ${index}`);
  const from = requireString(redirect.from, `documentation redirect ${index}.from`);
  const to = requireString(redirect.to, `documentation redirect ${index}.to`);
  if (from === to) {
    throw new TypeError(`documentation redirect ${from} cannot target itself`);
  }
  return { from, to };
}

function parsePublicDocsManifest(source: string): PublicDocsManifest {
  const manifest = requireRecord(JSON.parse(source) as unknown, "public documentation manifest");
  if (manifest.schemaVersion !== 1) {
    throw new TypeError("public documentation manifest schemaVersion must be 1");
  }
  if (!Array.isArray(manifest.sections) || manifest.sections.length === 0) {
    throw new TypeError("public documentation manifest must contain sections");
  }

  const sections = manifest.sections.map(parseSection);
  const paths = sections.flatMap((section) => section.items.map((item) => item.path));
  if (new Set(paths).size !== paths.length) {
    throw new TypeError("public documentation manifest paths must be unique");
  }
  const redirects = Array.isArray(manifest.redirects)
    ? manifest.redirects.map(parseRedirect)
    : [];
  const redirectSources = redirects.map((redirect) => redirect.from);
  if (new Set(redirectSources).size !== redirectSources.length) {
    throw new TypeError("public documentation redirect sources must be unique");
  }
  if (redirectSources.some((source) => paths.includes(source))) {
    throw new TypeError("a public documentation path cannot also be a redirect source");
  }

  return {
    product: requireString(manifest.product, "public documentation manifest product"),
    redirects,
    schemaVersion: 1,
    sections,
  };
}

export const publicDocsManifest = parsePublicDocsManifest(manifestSource);
