use super::super::seed::WorthGraphReadAccessInventorySeedParts;

pub(crate) fn seed_parts_with_authority_digests(
    authority_digests: Vec<String>,
) -> WorthGraphReadAccessInventorySeedParts {
    WorthGraphReadAccessInventorySeedParts {
        authority_digests,
        ..valid_seed_parts()
    }
}

pub(crate) fn seed_parts_with_selected_obligation_count(
    selected_obligation_count: usize,
) -> WorthGraphReadAccessInventorySeedParts {
    WorthGraphReadAccessInventorySeedParts {
        selected_obligation_count,
        ..valid_seed_parts()
    }
}

pub(crate) fn seed_parts_with_selected_registration_digests(
    selected_registration_digests: Vec<String>,
) -> WorthGraphReadAccessInventorySeedParts {
    WorthGraphReadAccessInventorySeedParts {
        selected_registration_digests,
        ..valid_seed_parts()
    }
}

pub(crate) fn seed_parts_with_touch_descriptor_digests(
    touch_descriptor_digests: Vec<String>,
) -> WorthGraphReadAccessInventorySeedParts {
    WorthGraphReadAccessInventorySeedParts {
        touch_descriptor_digests,
        ..valid_seed_parts()
    }
}

fn valid_seed_parts() -> WorthGraphReadAccessInventorySeedParts {
    WorthGraphReadAccessInventorySeedParts {
        selected_obligation_count: 2,
        selected_registration_count: 2,
        execution_row_count: 2,
        authority_digests: two_digests("authority"),
        touch_descriptor_digests: two_digests("touch"),
        selected_registration_digests: two_digests("registration"),
        residue_manifest_digests: two_digests("residue"),
        execution_proof_digests: two_digests("execution"),
        adoption_manifest_digests: two_digests("adoption"),
        selector_precision_report_digests: two_digests("precision"),
    }
}

fn two_digests(prefix: &str) -> Vec<String> {
    vec![format!("{prefix}-a"), format!("{prefix}-b")]
}
