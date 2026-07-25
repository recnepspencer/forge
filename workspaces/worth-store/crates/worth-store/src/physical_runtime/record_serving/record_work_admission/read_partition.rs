use worth_signal::facade::PartitionSubscription;
use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::work::{
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalWorkSemanticBasis,
    PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
};

use super::{boundary_fact, contract, security_scope};

const ROOT_ASPECT_KEY: &str = "store.physical.record.root-read-basis";
const ARTIFACT_ASPECT_KEY: &str = "store.physical.record.artifact-read-basis";
const FRAME_ASPECT_KEY: &str = "store.physical.record.frame-read-basis";
const SCAN_ASPECT_KEY: &str = "store.physical.record.scan-read-basis";

const ROOT_PARTITION: &str = "store.physical.record.root";
const ARTIFACT_PARTITION: &str = "store.physical.record.artifact";
const FRAME_PARTITION: &str = "store.physical.record.frame";
const SCAN_PARTITION: &str = "store.physical.record.scan";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum RecordReadPartition {
    Root,
    Artifact,
    Frame,
    Scan,
}

pub(super) struct InstalledRecordReadSemantics {
    pub(super) bases: RecordReadSemanticBases,
    pub(super) security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
    pub(super) scheduler_security: worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
    pub(super) declarations: [PhysicalSignalAspectDeclaration; 4],
}

#[derive(Clone, Debug)]
pub(super) struct RecordReadSemanticBases {
    root: PhysicalWorkSemanticBasis,
    artifact: PhysicalWorkSemanticBasis,
    frame: PhysicalWorkSemanticBasis,
    scan: PhysicalWorkSemanticBasis,
}

pub(super) fn install(
    witness: worth_store_aspect_native::StorePhysicalBoundaryWitness,
) -> InstalledRecordReadSemantics {
    let root = read_semantics(
        ROOT_ASPECT_KEY,
        1_301,
        ROOT_PARTITION,
        "record-root-read-admitted",
        witness,
    );
    let (security, scheduler_security) = security_scope(&root.fact);
    let artifact = read_semantics(
        ARTIFACT_ASPECT_KEY,
        1_302,
        ARTIFACT_PARTITION,
        "record-artifact-read-admitted",
        witness,
    );
    let frame = read_semantics(
        FRAME_ASPECT_KEY,
        1_303,
        FRAME_PARTITION,
        "record-frame-read-admitted",
        witness,
    );
    let scan = read_semantics(
        SCAN_ASPECT_KEY,
        1_304,
        SCAN_PARTITION,
        "record-scan-read-admitted",
        witness,
    );
    InstalledRecordReadSemantics {
        bases: RecordReadSemanticBases {
            root: root.basis,
            artifact: artifact.basis,
            frame: frame.basis,
            scan: scan.basis,
        },
        security,
        scheduler_security,
        declarations: [
            root.declaration,
            artifact.declaration,
            frame.declaration,
            scan.declaration,
        ],
    }
}

impl RecordReadSemanticBases {
    pub(super) fn for_partition(
        &self,
        partition: RecordReadPartition,
    ) -> PhysicalWorkSemanticBasis {
        match partition {
            RecordReadPartition::Root => self.root.clone(),
            RecordReadPartition::Artifact => self.artifact.clone(),
            RecordReadPartition::Frame => self.frame.clone(),
            RecordReadPartition::Scan => self.scan.clone(),
        }
    }
}

impl RecordReadPartition {
    pub(in crate::physical_runtime) const fn for_range(artifact: RecordArtifactFile) -> Self {
        match artifact {
            RecordArtifactFile::BootstrapCatalog
            | RecordArtifactFile::CatalogCandidate { .. }
            | RecordArtifactFile::RootManifest { .. }
            | RecordArtifactFile::RootRoutingBlock { .. } => Self::Root,
            RecordArtifactFile::Segment { .. } | RecordArtifactFile::Extent { .. } => Self::Frame,
            RecordArtifactFile::SegmentManifest { .. }
            | RecordArtifactFile::SegmentMembershipBlock { .. }
            | RecordArtifactFile::ExtentManifest { .. }
            | RecordArtifactFile::FreeSpaceManifest { .. }
            | RecordArtifactFile::FreeSpaceMembershipBlock { .. } => Self::Artifact,
        }
    }

    pub(in crate::physical_runtime) const fn for_metadata(artifact: RecordArtifactFile) -> Self {
        match artifact {
            RecordArtifactFile::BootstrapCatalog
            | RecordArtifactFile::CatalogCandidate { .. }
            | RecordArtifactFile::RootManifest { .. }
            | RecordArtifactFile::RootRoutingBlock { .. } => Self::Root,
            _ => Self::Artifact,
        }
    }
}

struct ReadSemantics {
    fact: worth_store_aspect_native::StoreAspectBoundaryFact,
    basis: PhysicalWorkSemanticBasis,
    declaration: PhysicalSignalAspectDeclaration,
}

fn read_semantics(
    key: &'static str,
    contract_identity: u64,
    partition: &'static str,
    value: &'static str,
    witness: worth_store_aspect_native::StorePhysicalBoundaryWitness,
) -> ReadSemantics {
    let (contract, identity, admission) = contract(key, contract_identity, witness);
    let fact = boundary_fact(&contract, identity, witness, value);
    let basis = PhysicalWorkSemanticBasis::projection(fact.clone(), admission.clone())
        .expect("built-in read fact and contract are constructed together");
    let declaration =
        PhysicalSignalAspectDeclaration::new(admission, PhysicalSignalAspectRole::Dependency)
            .for_families(PhysicalWorkSignalFamilySet::only(
                PhysicalWorkSignalFamily::ReadFault,
            ))
            .with_partition(PartitionSubscription::whole_partition(partition));
    ReadSemantics {
        fact,
        basis,
        declaration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_read_semantics_are_distinct_and_exactly_partitioned() {
        let installed = install(super::super::physical_witness());
        let expected = [
            (RecordReadPartition::Root, ROOT_ASPECT_KEY, ROOT_PARTITION),
            (
                RecordReadPartition::Artifact,
                ARTIFACT_ASPECT_KEY,
                ARTIFACT_PARTITION,
            ),
            (
                RecordReadPartition::Frame,
                FRAME_ASPECT_KEY,
                FRAME_PARTITION,
            ),
            (RecordReadPartition::Scan, SCAN_ASPECT_KEY, SCAN_PARTITION),
        ];
        for ((partition, aspect_key, partition_name), declaration) in
            expected.iter().zip(&installed.declarations)
        {
            let basis = installed.bases.for_partition(*partition);
            assert_eq!(basis.aspect_identity().aspect_key().as_str(), *aspect_key);
            let subscription = declaration
                .partition()
                .expect("every record read dependency is partitioned");
            assert_eq!(subscription.partition.0, *partition_name);
            assert!(subscription.detail.is_none());
            assert_eq!(
                subscription.match_mode,
                worth_signal::facade::PartitionMatchMode::WholePartition
            );
        }

        for (index, (partition, _, _)) in expected.iter().enumerate() {
            let basis = installed.bases.for_partition(*partition);
            for (other, _, _) in &expected[index + 1..] {
                assert_ne!(
                    basis,
                    installed.bases.for_partition(*other),
                    "read dependency categories must not collapse onto one basis"
                );
            }
        }
    }

    #[test]
    fn artifact_types_select_only_the_internal_read_partition_taxonomy() {
        use worth_store_physical_format::RecordArtifactFile;

        assert_eq!(
            RecordReadPartition::for_range(RecordArtifactFile::RootManifest { generation: 3 }),
            RecordReadPartition::Root
        );
        assert_eq!(
            RecordReadPartition::for_range(RecordArtifactFile::SegmentManifest {
                segment: 7,
                generation: 3,
            }),
            RecordReadPartition::Artifact
        );
        assert_eq!(
            RecordReadPartition::for_range(RecordArtifactFile::Extent {
                extent: 9,
                generation: 3,
            }),
            RecordReadPartition::Frame
        );
        assert_eq!(
            RecordReadPartition::for_metadata(RecordArtifactFile::Extent {
                extent: 9,
                generation: 3,
            }),
            RecordReadPartition::Artifact
        );
    }
}
