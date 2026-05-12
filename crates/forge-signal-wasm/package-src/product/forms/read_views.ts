export function createFormReadView(form) {
  return Object.freeze({
    source: form.source,
    draft: form.draft,
    effective: form.effective,
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
