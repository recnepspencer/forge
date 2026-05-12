use super::super::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalFloatWidth, CanonicalIntegerWidth,
};
use super::algorithm::{
    CanonicalDigestAlgorithmMetadata, CanonicalDigestInputDomain, CanonicalDigestInputShape,
};
use super::evidence::{
    CanonicalDigestBasisBundle, CanonicalDigestBasisSequence, CanonicalDigestDerivationInput,
    CanonicalDigestInputEvidence,
};
use crate::values::InternedString;

pub(super) fn canonical_digest_material(input: &CanonicalDigestDerivationInput) -> String {
    let mut material = String::new();
    append_algorithm_material(&mut material, input.algorithm());
    match input.evidence() {
        CanonicalDigestInputEvidence::SingleSequence(sequence) => {
            append_token(&mut material, "input", "single");
            append_sequence_material(&mut material, sequence);
        }
        CanonicalDigestInputEvidence::DomainBundle(bundle) => {
            append_token(&mut material, "input", "domain-bundle");
            append_bundle_material(&mut material, bundle);
        }
        CanonicalDigestInputEvidence::ExportBundle(bundle) => {
            append_token(&mut material, "input", "export-bundle");
            append_bundle_material(&mut material, bundle);
        }
    }
    material
}

fn append_algorithm_material(material: &mut String, algorithm: &CanonicalDigestAlgorithmMetadata) {
    append_token(material, "algorithm", algorithm.id().as_str());
    append_token(material, "version", algorithm.rule_version().as_str());
    append_token(
        material,
        "shape",
        input_shape_token(algorithm.input_shape()),
    );
    append_token(
        material,
        "domain",
        &input_domain_token(algorithm.input_domain()),
    );
}

fn append_bundle_material(material: &mut String, bundle: &CanonicalDigestBasisBundle) {
    append_token(material, "bundle-version", bundle.version().as_str());
    for sequence in bundle.sequences() {
        append_sequence_material(material, sequence);
    }
}

fn append_sequence_material(material: &mut String, sequence: &CanonicalDigestBasisSequence) {
    append_token(material, "sequence-version", sequence.version().as_str());
    append_token(
        material,
        "sequence-domain",
        domain_material_token(sequence.domain()),
    );
    append_u32(material, "cost.entry-count", sequence.cost().entry_count());
    append_u32(
        material,
        "cost.ordering-comparisons",
        sequence.cost().ordering_comparisons(),
    );
    append_u32(
        material,
        "cost.nested-sequence-count",
        sequence.cost().nested_sequence_count(),
    );
    append_u32(
        material,
        "cost.compatibility-lowering-count",
        sequence.cost().compatibility_lowering_count(),
    );
    for entry in sequence.entries() {
        append_entry_material(material, entry);
    }
}

fn append_entry_material(material: &mut String, entry: &CanonicalBasisEntry) {
    append_token(
        material,
        "entry.domain",
        domain_material_token(entry.domain()),
    );
    append_locus_material(material, entry.locus());
    append_token(material, "entry.kind", entry_kind_token(entry.kind()));
    append_value_material(material, entry.value());
}

fn append_locus_material(material: &mut String, locus: &CanonicalBasisLocus) {
    match locus {
        CanonicalBasisLocus::Root => append_token(material, "locus", "root"),
        CanonicalBasisLocus::EntryOrdinal(ordinal) => {
            append_u32(material, "locus.ordinal", *ordinal);
        }
        CanonicalBasisLocus::Aspect(aspect) => {
            append_token(material, "locus.aspect", aspect.as_str());
        }
        CanonicalBasisLocus::AspectField { aspect, path } => {
            append_token(material, "locus.aspect-field.aspect", aspect.as_str());
            append_u32(
                material,
                "locus.aspect-field.len",
                path.fields().len() as u32,
            );
            for field in path.fields() {
                append_token(material, "locus.aspect-field.field", field.as_str());
            }
        }
        CanonicalBasisLocus::Named(value) => {
            append_token(material, "locus.named.kind", "named");
            append_interned_string(material, "locus.named", value);
        }
    }
}

fn append_value_material(material: &mut String, value: &CanonicalBasisValue) {
    match value {
        CanonicalBasisValue::Null => append_token(material, "value.kind", "null"),
        CanonicalBasisValue::Bool(value) => {
            append_token(material, "value.kind", "bool");
            append_token(
                material,
                "value.bool",
                if *value { "true" } else { "false" },
            );
        }
        CanonicalBasisValue::SignedInteger { width, value } => {
            append_token(material, "value.kind", "signed");
            append_token(material, "value.width", integer_width_token(*width));
            append_token(material, "value.signed", &value.to_string());
        }
        CanonicalBasisValue::UnsignedInteger { width, value } => {
            append_token(material, "value.kind", "unsigned");
            append_token(material, "value.width", integer_width_token(*width));
            append_token(material, "value.unsigned", &value.to_string());
        }
        CanonicalBasisValue::FloatBits { width, bits } => {
            append_token(material, "value.kind", "float");
            append_token(material, "value.width", float_width_token(*width));
            append_u64(material, "value.float-bits", *bits);
        }
        CanonicalBasisValue::ExactText(value) => {
            append_token(material, "value.kind", "text");
            append_interned_string(material, "value.text", value);
        }
        CanonicalBasisValue::BytesDigest(value) => {
            append_token(material, "value.kind", "bytes-digest");
            append_bytes(material, "value.bytes-digest", value.bytes());
        }
        CanonicalBasisValue::DecimalText(value) => {
            append_token(material, "value.kind", "decimal");
            append_interned_string(material, "value.decimal", value);
        }
        CanonicalBasisValue::BigIntText(value) => {
            append_token(material, "value.kind", "bigint");
            append_interned_string(material, "value.bigint", value);
        }
        CanonicalBasisValue::RationalText {
            numerator,
            denominator,
        } => {
            append_token(material, "value.kind", "rational");
            append_interned_string(material, "value.rational.numerator", numerator);
            append_interned_string(material, "value.rational.denominator", denominator);
        }
        CanonicalBasisValue::BytesRefId(value) => {
            append_token(material, "value.kind", "bytes-ref");
            append_u64(material, "value.bytes-ref", *value);
        }
        CanonicalBasisValue::ContentRefId(value) => {
            append_token(material, "value.kind", "content-ref");
            append_u64(material, "value.content-ref", *value);
        }
        CanonicalBasisValue::EntityRef {
            partition_id,
            local_slot,
            generation,
        } => {
            append_token(material, "value.kind", "entity-ref");
            append_u64(material, "value.entity.partition", u64::from(*partition_id));
            append_u64(material, "value.entity.slot", *local_slot);
            append_u64(material, "value.entity.generation", u64::from(*generation));
        }
        CanonicalBasisValue::DateDays(value) => {
            append_token(material, "value.kind", "date-days");
            append_i64(material, "value.date-days", i64::from(*value));
        }
        CanonicalBasisValue::TimeNanos(value) => {
            append_token(material, "value.kind", "time-nanos");
            append_u64(material, "value.time-nanos", *value);
        }
        CanonicalBasisValue::TimestampMicros(value) => {
            append_token(material, "value.kind", "timestamp-micros");
            append_i64(material, "value.timestamp-micros", *value);
        }
        CanonicalBasisValue::TimestampTz {
            utc_micros_since_unix_epoch,
            offset_minutes,
        } => {
            append_token(material, "value.kind", "timestamp-tz");
            append_i64(
                material,
                "value.timestamp-tz.utc-micros",
                *utc_micros_since_unix_epoch,
            );
            append_i32(
                material,
                "value.timestamp-tz.offset-minutes",
                *offset_minutes,
            );
        }
        CanonicalBasisValue::UuidBytes(bytes) => {
            append_token(material, "value.kind", "uuid");
            append_bytes(material, "value.uuid", bytes);
        }
        CanonicalBasisValue::NestedSequence(value) => {
            append_token(material, "value.kind", "nested-sequence");
            append_u64(material, "value.nested-sequence", u64::from(*value));
        }
    }
}

fn append_interned_string(material: &mut String, label: &str, value: &InternedString) {
    match value {
        InternedString::Raw(value) => {
            append_token(material, &format!("{label}.raw"), value);
        }
        InternedString::Symbol(symbol) => {
            append_u64(material, &format!("{label}.symbol"), u64::from(symbol.0));
        }
    }
}

fn append_token(material: &mut String, label: &str, value: &str) {
    material.push_str(label);
    material.push('#');
    material.push_str(&value.len().to_string());
    material.push(':');
    material.push_str(value);
    material.push(';');
}

fn append_bytes(material: &mut String, label: &str, value: &[u8]) {
    material.push_str(label);
    material.push('#');
    material.push_str(&value.len().to_string());
    material.push(':');
    for byte in value {
        material.push_str(&format!("{byte:02x}"));
    }
    material.push(';');
}

fn append_u32(material: &mut String, label: &str, value: u32) {
    append_token(material, label, &value.to_string());
}

fn append_u64(material: &mut String, label: &str, value: u64) {
    append_token(material, label, &value.to_string());
}

fn append_i32(material: &mut String, label: &str, value: i32) {
    append_token(material, label, &value.to_string());
}

fn append_i64(material: &mut String, label: &str, value: i64) {
    append_token(material, label, &value.to_string());
}

pub(super) fn stable_fixture_digest(material: &[u8]) -> [u8; 32] {
    let mut lanes = [
        0xcbf2_9ce4_8422_2325_u64,
        0x9e37_79b9_7f4a_7c15_u64,
        0x94d0_49bb_1331_11eb_u64,
        0xd6e8_feb8_6659_fd93_u64,
    ];
    for (index, byte) in material.iter().enumerate() {
        let lane = index % lanes.len();
        lanes[lane] ^= u64::from(*byte) + ((index as u64) << 8);
        lanes[lane] = lanes[lane].wrapping_mul(0x1000_0000_01b3);
        lanes[lane] = lanes[lane].rotate_left(13);
    }

    let mut bytes = [0_u8; 32];
    for (index, lane) in lanes.iter().enumerate() {
        bytes[index * 8..(index + 1) * 8].copy_from_slice(&lane.to_be_bytes());
    }
    bytes
}

fn input_domain_token(domain: CanonicalDigestInputDomain) -> String {
    match domain {
        CanonicalDigestInputDomain::Single(domain) => {
            format!("single:{}", domain_material_token(domain))
        }
        CanonicalDigestInputDomain::DomainBundle => "domain-bundle".to_string(),
        CanonicalDigestInputDomain::ExportBundle => "export-bundle".to_string(),
    }
}

fn input_shape_token(shape: CanonicalDigestInputShape) -> &'static str {
    match shape {
        CanonicalDigestInputShape::SingleSequence => "single-sequence",
        CanonicalDigestInputShape::DomainBundle => "domain-bundle",
        CanonicalDigestInputShape::ExportBundle => "export-bundle",
    }
}

pub(super) fn domain_material_token(domain: CanonicalBasisDomain) -> &'static str {
    match domain {
        CanonicalBasisDomain::Value => "value",
        CanonicalBasisDomain::AspectContract => "aspect-contract",
        CanonicalBasisDomain::AspectMask => "aspect-mask",
        CanonicalBasisDomain::AuthoritativeState => "authoritative-state",
        CanonicalBasisDomain::AuthoritativePatch => "authoritative-patch",
        CanonicalBasisDomain::Identity => "identity",
        CanonicalBasisDomain::Locator => "locator",
        CanonicalBasisDomain::CompatibilityLowering => "compatibility-lowering",
        CanonicalBasisDomain::Future(value) => value,
    }
}

fn entry_kind_token(kind: CanonicalBasisEntryKind) -> &'static str {
    match kind {
        CanonicalBasisEntryKind::Header => "header",
        CanonicalBasisEntryKind::Shape => "shape",
        CanonicalBasisEntryKind::Value => "value",
        CanonicalBasisEntryKind::Field => "field",
        CanonicalBasisEntryKind::Mask => "mask",
        CanonicalBasisEntryKind::StateAspect => "state-aspect",
        CanonicalBasisEntryKind::PatchOperation => "patch-operation",
        CanonicalBasisEntryKind::Identity => "identity",
        CanonicalBasisEntryKind::Locator => "locator",
        CanonicalBasisEntryKind::CompatibilityOrigin => "compatibility-origin",
        CanonicalBasisEntryKind::Cost => "cost",
        CanonicalBasisEntryKind::Future(value) => value,
    }
}

fn integer_width_token(width: CanonicalIntegerWidth) -> &'static str {
    match width {
        CanonicalIntegerWidth::Bits8 => "i8",
        CanonicalIntegerWidth::Bits16 => "i16",
        CanonicalIntegerWidth::Bits32 => "i32",
        CanonicalIntegerWidth::Bits64 => "i64",
        CanonicalIntegerWidth::Bits128 => "i128",
    }
}

fn float_width_token(width: CanonicalFloatWidth) -> &'static str {
    match width {
        CanonicalFloatWidth::Bits32 => "f32",
        CanonicalFloatWidth::Bits64 => "f64",
    }
}
