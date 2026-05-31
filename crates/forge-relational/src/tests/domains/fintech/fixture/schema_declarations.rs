use crate::facade::schema::{DeclaredAspectContractBinding, RelationalSchemaRegistry};
use crate::tests::support::{
    aspect_key, entity_bool_field_aspect, entity_field_aspect, entity_u64_field_aspect, field_key,
    lifecycle_aspect, relation_field_aspect, relation_source_aspect, relation_target_aspect,
    AspectSchemaFixture,
};

pub(super) fn fintech_schema_registry() -> RelationalSchemaRegistry {
    AspectSchemaFixture {
        entity_aspects: fintech_entity_aspects(),
        relation_aspects: vec![
            relation_field_aspect(aspect_key("role"), field_key("role")),
            lifecycle_aspect(),
            relation_source_aspect(),
            relation_target_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

fn fintech_entity_aspects() -> Vec<DeclaredAspectContractBinding> {
    string_entity_field_aspects()
        .into_iter()
        .chain(unsigned_entity_field_aspects())
        .chain(bool_entity_field_aspects())
        .chain(std::iter::once(lifecycle_aspect()))
        .collect()
}

fn string_entity_field_aspects() -> Vec<DeclaredAspectContractBinding> {
    vec![
        entity_field_aspect(aspect_key("name"), field_key("name")),
        entity_field_aspect(aspect_key("entity_type"), field_key("entity_type")),
        entity_field_aspect(aspect_key("desk_name"), field_key("desk_name")),
        entity_field_aspect(aspect_key("book_name"), field_key("book_name")),
        entity_field_aspect(aspect_key("case"), field_key("case")),
        entity_field_aspect(aspect_key("rating"), field_key("rating")),
        entity_field_aspect(aspect_key("desk"), field_key("desk")),
        entity_field_aspect(aspect_key("book"), field_key("book")),
        entity_field_aspect(aspect_key("ccy"), field_key("ccy")),
        entity_field_aspect(aspect_key("status"), field_key("status")),
        entity_field_aspect(aspect_key("kind"), field_key("kind")),
        entity_field_aspect(aspect_key("event"), field_key("event")),
        entity_field_aspect(aspect_key("symbol"), field_key("symbol")),
        entity_field_aspect(aspect_key("asset_class"), field_key("asset_class")),
        entity_field_aspect(aspect_key("scenario"), field_key("scenario")),
        entity_field_aspect(aspect_key("breach_state"), field_key("breach_state")),
        entity_field_aspect(aspect_key("stress_regime"), field_key("stress_regime")),
        entity_field_aspect(aspect_key("limit_status"), field_key("limit_status")),
        entity_field_aspect(aspect_key("severity"), field_key("severity")),
        entity_field_aspect(aspect_key("recorded_by"), field_key("recorded_by")),
    ]
}

fn unsigned_entity_field_aspects() -> Vec<DeclaredAspectContractBinding> {
    vec![
        entity_u64_field_aspect(aspect_key("desk_index"), field_key("desk_index")),
        entity_u64_field_aspect(aspect_key("book_index"), field_key("book_index")),
        entity_u64_field_aspect(aspect_key("balance_cents"), field_key("balance_cents")),
        entity_u64_field_aspect(aspect_key("notional"), field_key("notional")),
        entity_u64_field_aspect(aspect_key("curve_bucket"), field_key("curve_bucket")),
        entity_u64_field_aspect(aspect_key("mid"), field_key("mid")),
        entity_u64_field_aspect(aspect_key("threshold_bps"), field_key("threshold_bps")),
        entity_u64_field_aspect(aspect_key("trade_index"), field_key("trade_index")),
    ]
}

fn bool_entity_field_aspects() -> Vec<DeclaredAspectContractBinding> {
    vec![
        entity_bool_field_aspect(
            aspect_key("correction_candidate"),
            field_key("correction_candidate"),
        ),
        entity_bool_field_aspect(aspect_key("corrected"), field_key("corrected")),
        entity_bool_field_aspect(aspect_key("diverged"), field_key("diverged")),
        entity_bool_field_aspect(aspect_key("transient"), field_key("transient")),
        entity_bool_field_aspect(
            aspect_key("savepoint_applied"),
            field_key("savepoint_applied"),
        ),
        entity_bool_field_aspect(aspect_key("refreshed"), field_key("refreshed")),
        entity_bool_field_aspect(
            aspect_key("repair_completed"),
            field_key("repair_completed"),
        ),
    ]
}
