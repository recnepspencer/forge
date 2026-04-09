use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, StructuralFingerprintIdentityTag};
use crate::snapshot::{MaterializedTruthViewObservation, SnapshotReadPacket};

use super::{AdmittedStructuralComparisonContract, StructuralFingerprintFamily};

pub type StructuralFingerprintIdentity = BridgeIdentity<StructuralFingerprintIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralFingerprint {
    fingerprint_identity: StructuralFingerprintIdentity,
    family: StructuralFingerprintFamily,
    semantics_version: Arc<str>,
    contract_identity: Arc<str>,
    truth_view_basis_digest: Arc<str>,
    read_packet: SnapshotReadPacket,
    planned_packet_digest: Arc<str>,
    snapshot_identity: Arc<str>,
    authority_digest: Arc<str>,
    equivalence_digest: Arc<str>,
    member_evidence: Arc<[Arc<str>]>,
    record_payload_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl StructuralFingerprint {
    pub fn from_observation(
        contract: &AdmittedStructuralComparisonContract,
        observation: &MaterializedTruthViewObservation,
    ) -> Result<Self, crate::snapshot::BridgeSnapshotReadError> {
        let validated = observation.read_planned_packet()?;
        let mut canonical_records = validated
            .records()
            .iter()
            .map(|record| {
                let payload_digest = Sha256::digest(record.payload());
                (
                    record.request_key().to_owned(),
                    format!("{payload_digest:x}"),
                )
            })
            .collect::<Vec<_>>();
        canonical_records.sort();
        let equivalence_members = canonical_records
            .iter()
            .map(|(_, payload_digest)| payload_digest.clone())
            .collect::<Vec<_>>();
        Ok(Self::from_validated_read(
            contract,
            observation.planned().read_packet(),
            observation.snapshot_identity().as_str(),
            equivalence_members,
            canonical_records
                .into_iter()
                .map(|(request_key, payload_digest)| format!("{request_key}:{payload_digest}"))
                .collect::<Vec<_>>(),
        ))
    }

    pub fn from_snapshot_read_packet(
        contract: &AdmittedStructuralComparisonContract,
        read_packet: &SnapshotReadPacket,
        snapshot_identity: impl Into<Arc<str>>,
    ) -> Self {
        let snapshot_identity = snapshot_identity.into();
        Self::from_validated_read(
            contract,
            read_packet,
            snapshot_identity,
            Vec::new(),
            Vec::new(),
        )
    }

    fn from_validated_read(
        contract: &AdmittedStructuralComparisonContract,
        read_packet: &SnapshotReadPacket,
        snapshot_identity: impl Into<Arc<str>>,
        equivalence_members: Vec<String>,
        record_digests: Vec<String>,
    ) -> Self {
        let snapshot_identity = snapshot_identity.into();
        let equivalence = contract
            .validated_declaration()
            .declaration()
            .equivalence_contract();
        let member_evidence: Arc<[Arc<str>]> = Arc::from(
            record_digests
                .into_iter()
                .map(Arc::<str>::from)
                .collect::<Vec<_>>(),
        );
        let record_payload_digest: Arc<str> = {
            let digest = Sha256::digest(
                member_evidence
                    .iter()
                    .map(|member| member.as_ref())
                    .collect::<Vec<_>>()
                    .join("|")
                    .as_bytes(),
            );
            Arc::from(format!("structural-record-payload:sha256:{digest:x}"))
        };
        let authority_digest: Arc<str> = {
            let digest = Sha256::digest(
                format!(
                    "structural-authority|snapshot={}|planned={}",
                    snapshot_identity.as_ref(),
                    read_packet.digest()
                )
                .as_bytes(),
            );
            Arc::from(format!("structural-authority:sha256:{digest:x}"))
        };
        let equivalence_digest: Arc<str> = {
            let equivalence_basis = if equivalence_members.is_empty() {
                read_packet.digest().to_owned()
            } else {
                equivalence_members.join("|")
            };
            let digest = Sha256::digest(
                format!(
                    "structural-equivalence|family:{:?}|semantics={}|members={}",
                    equivalence.fingerprint_family(),
                    equivalence.semantics_version(),
                    equivalence_basis,
                )
                .as_bytes(),
            );
            Arc::from(format!("structural-equivalence:sha256:{digest:x}"))
        };
        let canonical_basis = Arc::<str>::from(format!(
            "structural-fingerprint|family:{:?}|semantics={}|contract={}|truth-view={}|planned={}|snapshot={}|authority={}|equivalence={}|records={}|members={}",
            equivalence.fingerprint_family(),
            equivalence.semantics_version(),
            contract.contract_identity().as_str(),
            contract
                .validated_declaration()
                .declaration()
                .truth_view_basis()
                .digest(),
            read_packet.digest(),
            snapshot_identity.as_ref(),
            authority_digest.as_ref(),
            equivalence_digest.as_ref(),
            record_payload_digest.as_ref(),
            member_evidence
                .iter()
                .map(|member| member.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fingerprint_identity: StructuralFingerprintIdentity::new(format!(
                "structural-fingerprint:sha256:{digest:x}"
            )),
            family: equivalence.fingerprint_family(),
            semantics_version: Arc::from(equivalence.semantics_version()),
            contract_identity: Arc::from(contract.contract_identity().as_str()),
            truth_view_basis_digest: Arc::from(
                contract
                    .validated_declaration()
                    .declaration()
                    .truth_view_basis()
                    .digest(),
            ),
            read_packet: read_packet.clone(),
            planned_packet_digest: Arc::from(read_packet.digest()),
            snapshot_identity,
            authority_digest,
            equivalence_digest,
            member_evidence,
            record_payload_digest,
            canonical_basis,
            digest: Arc::from(format!("structural-fingerprint:sha256:{digest:x}")),
        }
    }

    pub fn fingerprint_identity(&self) -> &StructuralFingerprintIdentity {
        &self.fingerprint_identity
    }

    pub fn family(&self) -> StructuralFingerprintFamily {
        self.family
    }

    pub fn semantics_version(&self) -> &str {
        self.semantics_version.as_ref()
    }

    pub fn contract_identity(&self) -> &str {
        self.contract_identity.as_ref()
    }

    pub fn truth_view_basis_digest(&self) -> &str {
        self.truth_view_basis_digest.as_ref()
    }

    pub fn planned_packet_digest(&self) -> &str {
        self.planned_packet_digest.as_ref()
    }

    pub fn read_packet(&self) -> &SnapshotReadPacket {
        &self.read_packet
    }

    pub fn snapshot_identity(&self) -> &str {
        self.snapshot_identity.as_ref()
    }

    pub fn authority_digest(&self) -> &str {
        self.authority_digest.as_ref()
    }

    pub fn equivalence_digest(&self) -> &str {
        self.equivalence_digest.as_ref()
    }

    pub fn member_evidence(&self) -> &[Arc<str>] {
        &self.member_evidence
    }

    pub fn record_payload_digest(&self) -> &str {
        self.record_payload_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::envelope::TruthBranchIdentity;
    use crate::snapshot::{BridgeTruthViewSelector, TruthSnapshotIdentity};
    use crate::structural::{
        AdmittedStructuralRegistry, StructuralFingerprintEquivalenceContract,
        StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
        StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
        StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity,
        StructuralSchemaIdentity, StructuralTruthViewBasis,
    };

    use super::StructuralFingerprint;

    #[test]
    fn fingerprint_is_canonical_for_same_contract_and_read_packet() {
        let declaration = StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::new("structural:geometry"),
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::new("schema:geometry"),
                StructuralFingerprintFamily::TopologyFingerprint,
                "topology-v1",
                StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
                StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
                StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
            ),
            StructuralTruthViewBasis::explicit_snapshot(
                BridgeTruthViewSelector::committed_snapshot(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ),
        );
        let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
        let contract = registry.contracts()[0].clone();
        let read_packet = crate::snapshot::SnapshotReadPacket::new(vec![]);

        let left =
            StructuralFingerprint::from_snapshot_read_packet(&contract, &read_packet, "snapshot-a");
        let right =
            StructuralFingerprint::from_snapshot_read_packet(&contract, &read_packet, "snapshot-a");

        assert_eq!(left, right);
        assert_eq!(
            left.family(),
            StructuralFingerprintFamily::TopologyFingerprint
        );
    }
}
