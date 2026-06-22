use std::collections::BTreeMap;

use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

use crate::runtime::surface::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
    ForgeQueryDerivedMaterializationReceipt, ForgeQueryDerivedMaterializationResult,
    ForgeQueryDerivedMaterializationTarget, ForgeQueryRetainedFieldPath,
    ForgeQueryRetainedMaterializedRow,
};
use crate::runtime::ForgeQueryDerivedViewHandle;

fn retained_row(
    fields: impl IntoIterator<Item = (&'static str, AspectValue)>,
) -> ForgeQueryRetainedMaterializedRow {
    let fields = fields
        .into_iter()
        .map(|(path, value)| {
            (
                retained_field_path(path).expect("retained field path admits"),
                value,
            )
        })
        .collect::<BTreeMap<_, _>>();
    ForgeQueryRetainedMaterializedRow::from_scalar_values(fields)
        .expect("retained row should build")
}

fn binding() -> ForgeQueryDerivedArtifactBinding {
    let snapshot_identity = crate::memory_workspace::admit_external_snapshot_label("snapshot:test");
    let materialization = ForgeQueryDerivedMaterializationResult::from_retained_rows(
        vec![retained_row([
            ("authority_snapshot_id", AspectValue::Int64(7)),
            (
                "nested.truth_basis_digest_hex",
                AspectValue::String(InternedString::Raw("basis:test".to_string())),
            ),
        ])],
        ForgeQueryDerivedMaterializationReceipt::test_only(
            "surface:test",
            snapshot_identity.clone(),
            "result:test",
        ),
    );
    let bundle = ForgeQueryDerivedMaterializationBundle::new(
        snapshot_identity,
        BTreeMap::from([("surface:test".to_string(), materialization)]),
    );
    ForgeQueryDerivedArtifactBinding::bind(
        bundle,
        "artifact:test",
        [ForgeQueryDerivedMaterializationTarget::new("surface:test")],
    )
    .expect("binding should build")
}

fn view_handle() -> ForgeQueryDerivedViewHandle<crate::runtime::ForgeQueryNativeRow> {
    ForgeQueryDerivedViewHandle::new("surface:test")
}

#[test]
fn retained_scalar_fact_set_reads_nested_fields() {
    let view = view_handle();
    let truth_basis_digest =
        retained_field_path("nested.truth_basis_digest_hex").expect("field path should parse");
    let authority_snapshot_id =
        retained_field_path("authority_snapshot_id").expect("field path should parse");
    let facts = binding()
        .consume_scalar_fields(
            &view,
            [truth_basis_digest.clone(), authority_snapshot_id.clone()],
        )
        .expect("scalar facts should extract");

    assert_eq!(facts.artifact_name(), "artifact:test");
    assert_eq!(facts.target().view_name(), "surface:test");
    assert_eq!(facts.field_count(), 2);
    assert_eq!(
        facts.field_value_at(&authority_snapshot_id),
        Some(&AspectValue::Int64(7))
    );
    assert_eq!(
        facts.field_value_at(&truth_basis_digest),
        Some(&AspectValue::String(InternedString::Raw(
            "basis:test".to_string()
        )))
    );
    assert!(!facts.fact_set_digest().is_empty());
}

#[test]
fn retained_scalar_fact_set_rejects_missing_field() {
    let view = view_handle();
    let missing_field = retained_field_path("missing.field").expect("field path should parse");
    let error = binding()
        .consume_scalar_fields(&view, [missing_field])
        .expect_err("missing field should fail");

    assert!(matches!(
        error,
        crate::runtime::ForgeQueryRuntimeError::RetainedRowDecode { .. }
    ));
}

fn retained_field_path(path: &str) -> Result<ForgeQueryRetainedFieldPath, String> {
    let fields = path
        .split('.')
        .map(|segment| {
            FieldKey::new(segment.to_string())
                .ok_or_else(|| format!("`{path}` is not a retained scalar field path"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(fields)
        .ok_or_else(|| format!("`{path}` is not a retained scalar field path"))?;
    Ok(ForgeQueryRetainedFieldPath::from_canonical_field_path(path))
}
