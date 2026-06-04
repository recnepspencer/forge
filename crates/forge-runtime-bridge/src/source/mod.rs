mod capabilities;
mod contracts;
mod counters;
mod declaration;
mod failures;
mod grouped_contract;
mod grouped_truth_view;
mod materialization;
mod packet_set_digest_basis;
mod planning;
mod records;
mod row_set;
mod validation;

pub use capabilities::{BridgeSourceCapability, BridgeSourceCapabilitySet};
pub use contracts::{AdmittedSourceContract, AdmittedSourceRegistry};
pub use counters::SourceMaterializationCounters;
pub use declaration::{SourceDeclaration, SourceDeclarationIdentity};
pub use failures::{SourceFailureClass, SourceFailureRecord, SourceFailureRecordIdentity};
pub use grouped_contract::{GroupedProjectionMemberSource, GroupedProjectionSource};
pub use grouped_truth_view::{
    materialize_bridge_grouped_truth_view_from_projection, BridgeGroupedBindingValueFamily,
    BridgeGroupedLaneValue, BridgeGroupedMemberRow, BridgeGroupedTruthViewArtifact,
    BridgeGroupedTruthViewDigest, BridgeGroupedTruthViewError,
};
pub use materialization::MaterializedTruthViewPacketSet;
pub use planning::PlannedSourceReadPacketSet;
pub use records::{SourceMaterializationRecord, SourceMaterializationRecordIdentity};
pub use row_set::{
    materialize_bridge_row_set, BridgeMaterializedFieldIdentity, BridgeMaterializedFieldProjection,
    BridgeMaterializedFieldValue, BridgeMaterializedRowArtifact, BridgeMaterializedRowSetArtifact,
    BridgeMaterializedRowSetDigest, BridgeRowIdentity, BridgeRowSetMaterializationError,
};
pub use validation::ValidatedSourceDeclaration;
