import { stableValueDigest } from "../values/value_paths.js";

export function readLayoutReport(fieldDeclarations, form) {
  const accessibility = form.accessibility();
  const steps = form.steps();
  const stepByField = new Map();
  for (const step of steps.artifacts) {
    for (const fieldId of step.fields) {
      stepByField.set(fieldId, step);
    }
  }
  const fieldHints = fieldDeclarations.map((declaration) =>
    fieldLayoutHint(declaration, accessibility, stepByField.get(declaration.id) ?? null),
  );
  const rowHints = rowLayoutHints(fieldHints);
  const sectionHints = sectionLayoutHints(steps.artifacts, fieldHints, rowHints);
  const summary = Object.freeze({
    sections: sectionHints.length,
    rows: rowHints.length,
    fields: fieldHints.length,
    unavailableFields: fieldHints.filter((field) => field.capabilityPosture.posture === "unavailable").length,
    responsiveFields: fieldHints.filter((field) => field.responsive.length > 0).length,
    messageTrackFields: fieldHints.filter((field) => field.tracks.message === "declared").length,
  });
  const counters = Object.freeze({
    costBasis: "derivedLayoutHintScan",
    incrementalStatus: "notIncremental",
    sections: sectionHints.length,
    rows: rowHints.length,
    fields: fieldHints.length,
    responsiveTokens: fieldHints.reduce((total, field) => total + field.responsive.length, 0) +
      sectionHints.reduce((total, section) => total + section.responsive.length, 0),
    minHeightHints: fieldHints.filter((field) => field.minHeight !== null).length,
    growFields: fieldHints.filter((field) => field.grow).length,
    wrapRows: rowHints.filter((row) => row.wrap).length,
    unavailableFields: summary.unavailableFields,
  });
  return Object.freeze({
    sections: Object.freeze(sectionHints),
    rows: Object.freeze(rowHints),
    fields: Object.freeze(fieldHints),
    summary,
    counters,
    digest: stableValueDigest({
      sections: sectionHints,
      rows: rowHints,
      fields: fieldHints,
      summary,
      counters,
    }),
  });
}

function fieldLayoutHint(declaration, accessibility, step) {
  const accessibilityField = accessibility.fields.find((entry) => entry.field === declaration.id);
  const unavailableCapabilities = [];
  if (!declaration.inputAdapter.capabilities.supportsLabelTrack) {
    unavailableCapabilities.push("labelTrack");
  }
  if (declaration.layout.minHeight !== null && !declaration.inputAdapter.capabilities.supportsMinHeightSync) {
    unavailableCapabilities.push("minHeightSync");
  }
  if (declaration.layout.responsive.length > 0 && !declaration.inputAdapter.capabilities.supportsResponsiveTokens) {
    unavailableCapabilities.push("responsiveTokens");
  }
  if (
    accessibilityField?.messageIds.length > 0 &&
    !declaration.inputAdapter.capabilities.supportsMessageTrack
  ) {
    unavailableCapabilities.push("messageTrack");
  }
  return Object.freeze({
    field: declaration.id,
    path: declaration.path,
    section: step?.id ?? null,
    row: declaration.layout.row,
    column: declaration.layout.column,
    tracks: Object.freeze({
      label: "declared",
      control: "declared",
      help: declaration.accessibility.description ? "declared" : "omitted",
      message: accessibilityField?.messageIds.length ? "declared" : "omitted",
    }),
    density: declaration.layout.density,
    alignment: declaration.layout.alignment,
    minHeight: declaration.layout.minHeight,
    grow: declaration.layout.grow,
    wrap: declaration.layout.wrap,
    responsive: declaration.layout.responsive,
    capabilityPosture: unavailableCapabilities.length === 0
      ? Object.freeze({
          posture: "supported",
          unavailableCapabilities: Object.freeze([]),
          reason: null,
        })
      : Object.freeze({
          posture: "unavailable",
          unavailableCapabilities: Object.freeze(unavailableCapabilities),
          reason: `${declaration.id} adapter cannot honor ${unavailableCapabilities.join(", ")}`,
        }),
  });
}

function rowLayoutHints(fieldHints) {
  const rows = new Map();
  for (const field of fieldHints) {
    const key = `${field.section ?? "form"}::${field.row}`;
    const bucket = rows.get(key) ?? {
      id: field.row,
      section: field.section,
      fields: [],
      columns: new Set(),
      minHeights: [],
      growFields: [],
      wrap: false,
    };
    bucket.fields.push(field.field);
    bucket.columns.add(field.column);
    if (field.minHeight !== null) {
      bucket.minHeights.push(field.minHeight);
    }
    if (field.grow) {
      bucket.growFields.push(field.field);
    }
    bucket.wrap = bucket.wrap || field.wrap;
    rows.set(key, bucket);
  }
  return [...rows.values()].map((row) => Object.freeze({
    id: row.id,
    section: row.section,
    fields: Object.freeze(row.fields),
    columns: Object.freeze([...row.columns]),
    maxMinHeight: row.minHeights.length ? Math.max(...row.minHeights) : null,
    growFields: Object.freeze(row.growFields),
    wrap: row.wrap,
  }));
}

function sectionLayoutHints(stepArtifacts, fieldHints, rowHints) {
  return stepArtifacts.map((step) => Object.freeze({
    id: step.id,
    group: step.group,
    order: step.order,
    density: step.layout.density,
    alignment: step.layout.alignment,
    responsive: step.layout.responsive,
    fields: Object.freeze(fieldHints.filter((field) => field.section === step.id).map((field) => field.field)),
    rows: Object.freeze(rowHints.filter((row) => row.section === step.id).map((row) => row.id)),
  }));
}
