use crate::runtime::WorthUiSourceBackedDeclarationWitness;
use worth_ui_dsl::{UiDslSourceProvenance, WorthUiDslPackage};

/// Comparison-safe identity of the declaration source admitted for one
/// prepared generation. Its construction basis remains private to preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPreparedDeclarationSourceIdentity {
    digest: u64,
}

impl WorthUiPreparedDeclarationSourceIdentity {
    pub(crate) fn derive(
        package: &WorthUiDslPackage,
        source_witness: Option<&WorthUiSourceBackedDeclarationWitness>,
    ) -> Self {
        let mut digest = fold_text(package.package_name());
        for receipt in package.runtime_lowering_receipts() {
            digest = digest.rotate_left(7) ^ receipt.semantic_input_digest();
            digest = digest.rotate_left(13) ^ provenance_digest(receipt.source_provenance());
        }
        if let Some(source_witness) = source_witness {
            digest = digest.rotate_left(17) ^ source_witness.identity_digest();
        }
        Self { digest }
    }
}

fn provenance_digest(provenance: &UiDslSourceProvenance) -> u64 {
    match provenance {
        UiDslSourceProvenance::FileAuthored {
            module_path,
            declaration_index,
        }
        | UiDslSourceProvenance::RustAuthored {
            module_path,
            declaration_index,
        } => {
            // Authoring lane is diagnostic provenance, not declaration
            // meaning. Equivalent file and Rust composition must converge on
            // the same preparation identity.
            fold_text("authored-source")
                ^ fold_text(module_path).rotate_left(7)
                ^ (*declaration_index as u64).rotate_left(19)
        }
    }
}

fn fold_text(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, byte| {
            (digest ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
        })
}
