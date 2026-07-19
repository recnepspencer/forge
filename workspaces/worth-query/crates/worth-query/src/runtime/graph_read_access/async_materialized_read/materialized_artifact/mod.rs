use super::WorthQueryGraphReadMaterializationReceipt;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializedRowProof {
    digest: String,
    materialization_digest: String,
    row_ordinal: usize,
}

impl WorthQueryGraphReadMaterializedRowProof {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn row_ordinal(&self) -> usize {
        self.row_ordinal
    }

    fn from_materialization(materialization_digest: &str, row_ordinal: usize) -> Self {
        let digest = hash_parts(&[
            "worth_query_graph_read_materialized_row_proof_v1".to_string(),
            format!("materialization:{materialization_digest}"),
            format!("row_ordinal:{row_ordinal}"),
        ]);
        Self {
            digest,
            materialization_digest: materialization_digest.to_string(),
            row_ordinal,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializedArtifact {
    digest: String,
    receipt: WorthQueryGraphReadMaterializationReceipt,
    row_proofs: Vec<WorthQueryGraphReadMaterializedRowProof>,
}

impl WorthQueryGraphReadMaterializedArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn receipt(&self) -> &WorthQueryGraphReadMaterializationReceipt {
        &self.receipt
    }

    pub fn materialization_digest(&self) -> &str {
        self.receipt.materialization_digest()
    }

    pub fn row_count(&self) -> usize {
        self.row_proofs.len()
    }

    pub fn row_proofs(&self) -> &[WorthQueryGraphReadMaterializedRowProof] {
        &self.row_proofs
    }

    pub fn request_digest(&self) -> &str {
        self.receipt.request_digest()
    }

    pub fn final_checkpoint_digest(&self) -> &str {
        self.receipt.final_checkpoint_digest()
    }

    pub(crate) fn from_receipt(receipt: WorthQueryGraphReadMaterializationReceipt) -> Self {
        let row_proofs = materialized_row_proofs_for_receipt(&receipt);
        let row_proof_digest = materialized_row_proof_set_digest(&row_proofs);
        let digest = hash_parts(&[
            "worth_query_graph_read_materialized_artifact_v1".to_string(),
            format!("receipt:{}", receipt.digest()),
            format!("materialization:{}", receipt.materialization_digest()),
            format!("rows:{}", row_proofs.len()),
            format!("row_proofs:{row_proof_digest}"),
        ]);
        Self {
            digest,
            receipt,
            row_proofs,
        }
    }
}

fn materialized_row_proofs_for_receipt(
    receipt: &WorthQueryGraphReadMaterializationReceipt,
) -> Vec<WorthQueryGraphReadMaterializedRowProof> {
    (0..receipt.emitted_rows())
        .map(|row_ordinal| {
            WorthQueryGraphReadMaterializedRowProof::from_materialization(
                receipt.materialization_digest(),
                row_ordinal,
            )
        })
        .collect()
}

fn materialized_row_proof_set_digest(
    row_proofs: &[WorthQueryGraphReadMaterializedRowProof],
) -> String {
    hash_parts(
        &row_proofs
            .iter()
            .map(|row| format!("row:{}", row.digest()))
            .collect::<Vec<_>>(),
    )
}
