use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::diagnostics::BridgeHistoricalMaterializationPath;
use crate::identity::{BridgeIdentity, SourceMaterializationRecordIdentityTag};
use crate::snapshot::{
    MaterializedTruthViewObservation, SnapshotReadPacket, TruthSnapshotIdentity,
};

use super::{AdmittedSourceContract, BridgeSourceCapability, MaterializedTruthViewPacketSet};

pub type SourceMaterializationRecordIdentity =
    BridgeIdentity<SourceMaterializationRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMaterializationRecord {
    record_identity: SourceMaterializationRecordIdentity,
    source_contract_identity: Arc<str>,
    source_declaration_identity: Arc<str>,
    source_capability_digest: Arc<str>,
    adapter_capability_digest: Arc<str>,
    planned_packet_set_digest: Arc<str>,
    materialized_packet_set_digest: Arc<str>,
    planned_packet_digests: Arc<[Arc<str>]>,
    read_packets: Arc<[SnapshotReadPacket]>,
    authority_basis_digests: Arc<[Arc<str>]>,
    snapshot_identities: Arc<[TruthSnapshotIdentity]>,
    materialization_paths: Arc<[BridgeHistoricalMaterializationPath]>,
    counters: SourceMaterializationCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceMaterializationCounters {
    source_declaration_count: usize,
    source_contract_count: usize,
    source_packet_count: usize,
    source_packet_member_count: usize,
    source_materialization_count: usize,
    source_snapshot_read_count: usize,
    source_historical_read_count: usize,
    source_branch_read_count: usize,
    source_facet_read_count: usize,
    source_capability_rejection_count: usize,
    source_contract_mismatch_count: usize,
    source_adapter_fallback_count: usize,
    source_builder_configuration_conflict_count: usize,
}

impl SourceMaterializationCounters {
    fn from_observation(
        contract: &AdmittedSourceContract,
        observation: &MaterializedTruthViewObservation,
    ) -> Self {
        Self {
            source_declaration_count: 1,
            source_contract_count: 1,
            source_packet_count: 1,
            source_packet_member_count: observation.read_packet().reads().len(),
            source_materialization_count: 1,
            source_snapshot_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::SnapshotRead),
            ),
            source_historical_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::HistoricalRead),
            ),
            source_branch_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::BranchRead),
            ),
            source_facet_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::FacetRead),
            ),
            source_capability_rejection_count: 0,
            source_contract_mismatch_count: 0,
            source_adapter_fallback_count: 0,
            source_builder_configuration_conflict_count: 0,
        }
    }

    fn from_packet_set(
        contract: &AdmittedSourceContract,
        materialized_packet_set: &MaterializedTruthViewPacketSet,
    ) -> Self {
        Self {
            source_declaration_count: 1,
            source_contract_count: 1,
            source_packet_count: materialized_packet_set.planned_packet_set().packet_count(),
            source_packet_member_count: materialized_packet_set
                .planned_packet_set()
                .packet_member_count(),
            source_materialization_count: materialized_packet_set.materialization_count(),
            source_snapshot_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::SnapshotRead),
            ),
            source_historical_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::HistoricalRead),
            ),
            source_branch_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::BranchRead),
            ),
            source_facet_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::FacetRead),
            ),
            source_capability_rejection_count: 0,
            source_contract_mismatch_count: 0,
            source_adapter_fallback_count: 0,
            source_builder_configuration_conflict_count: 0,
        }
    }

    pub fn source_declaration_count(&self) -> usize {
        self.source_declaration_count
    }

    pub fn source_contract_count(&self) -> usize {
        self.source_contract_count
    }

    pub fn source_packet_count(&self) -> usize {
        self.source_packet_count
    }

    pub fn source_packet_member_count(&self) -> usize {
        self.source_packet_member_count
    }

    pub fn source_materialization_count(&self) -> usize {
        self.source_materialization_count
    }

    pub fn source_snapshot_read_count(&self) -> usize {
        self.source_snapshot_read_count
    }

    pub fn source_historical_read_count(&self) -> usize {
        self.source_historical_read_count
    }

    pub fn source_branch_read_count(&self) -> usize {
        self.source_branch_read_count
    }

    pub fn source_facet_read_count(&self) -> usize {
        self.source_facet_read_count
    }

    pub fn source_capability_rejection_count(&self) -> usize {
        self.source_capability_rejection_count
    }

    pub fn source_contract_mismatch_count(&self) -> usize {
        self.source_contract_mismatch_count
    }

    pub fn source_adapter_fallback_count(&self) -> usize {
        self.source_adapter_fallback_count
    }

    pub fn source_builder_configuration_conflict_count(&self) -> usize {
        self.source_builder_configuration_conflict_count
    }
}

impl SourceMaterializationRecord {
    pub fn new(
        contract: &AdmittedSourceContract,
        observation: &MaterializedTruthViewObservation,
        adapter_capability_digest: impl Into<Arc<str>>,
    ) -> Self {
        let adapter_capability_digest = adapter_capability_digest.into();
        let planned_packet_set_digest = synthetic_planned_packet_set_digest(contract, observation);
        let materialized_packet_set_digest =
            synthetic_materialized_packet_set_digest(&planned_packet_set_digest, observation);
        let counters = SourceMaterializationCounters::from_observation(contract, observation);
        let planned_packet_digests = vec![Arc::from(observation.planned().digest())];
        let read_packets = vec![observation.read_packet().clone()];
        let authority_basis_digests = vec![Arc::from(observation.authority_basis().digest())];
        let snapshot_identities = vec![observation.snapshot_identity().clone()];
        let materialization_paths = vec![observation.materialization_path()];
        let canonical_basis = Arc::<str>::from(format!(
            "source-materialization-record|contract={}|declaration={}|source-capabilities={}|adapter-capabilities={}|planned-set={}|materialized-set={}|planned-packets={}|read-packets={}|authorities={}|snapshots={}|paths={}|counters={:?}",
            contract.contract_identity().as_str(),
            contract.declaration().declaration_identity().as_str(),
            contract.required_capabilities().digest(),
            adapter_capability_digest.as_ref(),
            planned_packet_set_digest.as_ref(),
            materialized_packet_set_digest.as_ref(),
            planned_packet_digests
                .iter()
                .map(|digest: &Arc<str>| digest.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            read_packets
                .iter()
                .map(SnapshotReadPacket::digest)
                .collect::<Vec<_>>()
                .join(","),
            authority_basis_digests
                .iter()
                .map(|digest: &Arc<str>| digest.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            snapshot_identities
                .iter()
                .map(TruthSnapshotIdentity::as_str)
                .collect::<Vec<_>>()
                .join(","),
            materialization_paths
                .iter()
                .map(|path| format!("{path:?}"))
                .collect::<Vec<_>>()
                .join(","),
            counters,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let record_identity = SourceMaterializationRecordIdentity::new(format!(
            "source-materialization-record:sha256:{digest:x}"
        ));

        Self {
            record_identity,
            source_contract_identity: Arc::from(contract.contract_identity().as_str()),
            source_declaration_identity: Arc::from(
                contract.declaration().declaration_identity().as_str(),
            ),
            source_capability_digest: Arc::from(contract.required_capabilities().digest()),
            adapter_capability_digest,
            planned_packet_set_digest,
            materialized_packet_set_digest,
            planned_packet_digests: Arc::from(planned_packet_digests),
            read_packets: Arc::from(read_packets),
            authority_basis_digests: Arc::from(authority_basis_digests),
            snapshot_identities: Arc::from(snapshot_identities),
            materialization_paths: Arc::from(materialization_paths),
            counters,
            canonical_basis,
            digest: Arc::from(format!("source-materialization-record:sha256:{digest:x}")),
        }
    }

    pub fn from_packet_set(
        contract: &AdmittedSourceContract,
        materialized_packet_set: &MaterializedTruthViewPacketSet,
        adapter_capability_digest: impl Into<Arc<str>>,
    ) -> Self {
        let adapter_capability_digest = adapter_capability_digest.into();
        let counters =
            SourceMaterializationCounters::from_packet_set(contract, materialized_packet_set);
        let planned_packet_digests = materialized_packet_set
            .observations()
            .iter()
            .map(|observation| Arc::from(observation.planned().digest()))
            .collect::<Vec<_>>();
        let read_packets = materialized_packet_set
            .observations()
            .iter()
            .map(|observation| observation.read_packet().clone())
            .collect::<Vec<_>>();
        let authority_basis_digests = materialized_packet_set
            .observations()
            .iter()
            .map(|observation| Arc::from(observation.authority_basis().digest()))
            .collect::<Vec<_>>();
        let snapshot_identities = materialized_packet_set
            .observations()
            .iter()
            .map(|observation| observation.snapshot_identity().clone())
            .collect::<Vec<_>>();
        let materialization_paths = materialized_packet_set
            .observations()
            .iter()
            .map(|observation| observation.materialization_path())
            .collect::<Vec<_>>();
        let canonical_basis = Arc::<str>::from(format!(
            "source-materialization-record|contract={}|declaration={}|source-capabilities={}|adapter-capabilities={}|planned-set={}|materialized-set={}|planned-packets={}|read-packets={}|authorities={}|snapshots={}|paths={}|counters={:?}",
            contract.contract_identity().as_str(),
            contract.declaration().declaration_identity().as_str(),
            contract.required_capabilities().digest(),
            adapter_capability_digest.as_ref(),
            materialized_packet_set.planned_packet_set().digest(),
            materialized_packet_set.digest(),
            planned_packet_digests
                .iter()
                .map(|digest: &Arc<str>| digest.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            read_packets
                .iter()
                .map(SnapshotReadPacket::digest)
                .collect::<Vec<_>>()
                .join(","),
            authority_basis_digests
                .iter()
                .map(|digest: &Arc<str>| digest.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            snapshot_identities
                .iter()
                .map(TruthSnapshotIdentity::as_str)
                .collect::<Vec<_>>()
                .join(","),
            materialization_paths
                .iter()
                .map(|path| format!("{path:?}"))
                .collect::<Vec<_>>()
                .join(","),
            counters,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let record_identity = SourceMaterializationRecordIdentity::new(format!(
            "source-materialization-record:sha256:{digest:x}"
        ));

        Self {
            record_identity,
            source_contract_identity: Arc::from(contract.contract_identity().as_str()),
            source_declaration_identity: Arc::from(
                contract.declaration().declaration_identity().as_str(),
            ),
            source_capability_digest: Arc::from(contract.required_capabilities().digest()),
            adapter_capability_digest,
            planned_packet_set_digest: Arc::from(
                materialized_packet_set.planned_packet_set().digest(),
            ),
            materialized_packet_set_digest: Arc::from(materialized_packet_set.digest()),
            planned_packet_digests: Arc::from(planned_packet_digests),
            read_packets: Arc::from(read_packets),
            authority_basis_digests: Arc::from(authority_basis_digests),
            snapshot_identities: Arc::from(snapshot_identities),
            materialization_paths: Arc::from(materialization_paths),
            counters,
            canonical_basis,
            digest: Arc::from(format!("source-materialization-record:sha256:{digest:x}")),
        }
    }

    pub fn record_identity(&self) -> &SourceMaterializationRecordIdentity {
        &self.record_identity
    }

    pub fn source_contract_identity(&self) -> &str {
        self.source_contract_identity.as_ref()
    }

    pub fn source_declaration_identity(&self) -> &str {
        self.source_declaration_identity.as_ref()
    }

    pub fn source_capability_digest(&self) -> &str {
        self.source_capability_digest.as_ref()
    }

    pub fn adapter_capability_digest(&self) -> &str {
        self.adapter_capability_digest.as_ref()
    }

    pub fn planned_packet_set_digest(&self) -> &str {
        self.planned_packet_set_digest.as_ref()
    }

    pub fn materialized_packet_set_digest(&self) -> &str {
        self.materialized_packet_set_digest.as_ref()
    }

    pub fn truth_view_digest(&self) -> &str {
        self.materialized_packet_set_digest()
    }

    pub fn planned_packet_digests(&self) -> &[Arc<str>] {
        &self.planned_packet_digests
    }

    pub fn read_packets(&self) -> &[SnapshotReadPacket] {
        &self.read_packets
    }

    pub fn authority_basis_digests(&self) -> &[Arc<str>] {
        &self.authority_basis_digests
    }

    pub fn snapshot_identities(&self) -> &[TruthSnapshotIdentity] {
        &self.snapshot_identities
    }

    pub fn materialization_paths(&self) -> &[BridgeHistoricalMaterializationPath] {
        &self.materialization_paths
    }

    pub fn counters(&self) -> &SourceMaterializationCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn synthetic_planned_packet_set_digest(
    contract: &AdmittedSourceContract,
    observation: &MaterializedTruthViewObservation,
) -> Arc<str> {
    let canonical_basis = format!(
        "planned-source-read-packet-set|contract={}|validated={}|packets={}",
        contract.digest(),
        super::ValidatedSourceDeclaration::from_contract(contract).digest(),
        observation.planned().digest(),
    );
    let digest = Sha256::digest(canonical_basis.as_bytes());
    Arc::from(format!("planned-source-read-packet-set:sha256:{digest:x}"))
}

fn synthetic_materialized_packet_set_digest(
    planned_packet_set_digest: &str,
    observation: &MaterializedTruthViewObservation,
) -> Arc<str> {
    let canonical_basis = format!(
        "materialized-truth-view-packet-set|planned={}|observations={}|{}|{:?}|{}",
        planned_packet_set_digest,
        observation.planned().digest(),
        observation.snapshot_identity().as_str(),
        observation.materialization_path(),
        observation.snapshot_token().snapshot_identity().as_str(),
    );
    let digest = Sha256::digest(canonical_basis.as_bytes());
    Arc::from(format!(
        "materialized-truth-view-packet-set:sha256:{digest:x}"
    ))
}
