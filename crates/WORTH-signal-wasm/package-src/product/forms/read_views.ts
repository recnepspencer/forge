export function createFormReadView(form, snapshots = {}) {
  return Object.freeze({
    source: snapshots.source === undefined ? form.source : () => snapshots.source,
    draft: snapshots.draft === undefined ? form.draft : () => snapshots.draft,
    effective: snapshots.effective === undefined ? form.effective : () => snapshots.effective,
    host: snapshots.host === undefined ? form.host : () => snapshots.host,
    field(fieldId) {
      return createFieldReadView(form.field(fieldId));
    },
  });
}

function createFieldReadView(field) {
  return Object.freeze({
    id: field.id,
    path: field.path,
    locus: field.locus,
    sourceValue: field.sourceValue,
    draftValue: field.draftValue,
    effectiveValue: field.effectiveValue,
    value: field.value,
    dirty: field.dirty,
  });
}
