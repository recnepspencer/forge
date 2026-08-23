use std::sync::Arc;

use crate::history::data::CanonicalCommitEnvelope;
use crate::storage::overlay::PartitionAccess;
use crate::storage::RelationalPublishedPartitionDelta;

use super::{
    axes, capture, PreparedRelationalBranchRootCapture, RelationalBranchRoot,
    RelationalBranchRootAxes, RelationalBranchRootCaptureDenial,
    RelationalBranchRootIdentityIssuer, RelationalBranchRootPublicationCost,
    RelationalBranchRootSchemaAuthority, RelationalBranchVisibilityCommitment,
    RelationalCommittedBranchRoot, RelationalPersistentRegionSet, RelationalRootCorrectnessIndex,
};

struct PreparedRootRegions {
    regions: Arc<RelationalPersistentRegionSet>,
    storage_root: [u8; 32],
    content_accumulator: [u8; 32],
    publication_cost: RelationalBranchRootPublicationCost,
}

struct PreparedRootSchema {
    authority: Arc<RelationalBranchRootSchemaAuthority>,
    schema_root: [u8; 32],
    allocation: PreparedSchemaAllocation,
}

enum PreparedSchemaAllocation {
    Reused,
    New { authoritative_bytes: u64 },
}

pub(super) fn prepare(
    issuer: &RelationalBranchRootIdentityIssuer,
    partitions: &impl PartitionAccess,
    published_delta: &RelationalPublishedPartitionDelta,
    previous: Option<&Arc<RelationalBranchRoot>>,
    envelope: Arc<CanonicalCommitEnvelope>,
    registry: &crate::schema::data::RelationalSchemaRegistry,
    symbols: &crate::symbols::data::StringInterner,
) -> Result<PreparedRelationalBranchRootCapture, RelationalBranchRootCaptureDenial> {
    let mut next_issuer = issuer.clone();
    let root_id = next_issuer.issue_root_id()?;
    let prepared_regions = prepare_regions(
        &mut next_issuer,
        root_id,
        partitions,
        published_delta,
        previous,
        symbols,
    )?;
    let prepared_schema = prepare_schema(&mut next_issuer, previous, &envelope, registry)?;
    let publication_cost = prepared_schema.apply_cost(prepared_regions.publication_cost);
    let correctness_index = RelationalRootCorrectnessIndex::AuthoritativeFallback;
    let axes = RelationalBranchRootAxes {
        storage_version: envelope.commit.version_id.0,
        storage_root: prepared_regions.storage_root,
        schema_root: prepared_schema.schema_root,
        correctness_index,
        visibility: RelationalBranchVisibilityCommitment::for_root(
            &envelope,
            prepared_regions.storage_root,
            prepared_schema.schema_root,
            correctness_index,
        ),
    };
    let root = Arc::new(RelationalBranchRoot {
        id: root_id,
        regions: prepared_regions.regions,
        content_accumulator: prepared_regions.content_accumulator,
        schema_authority: prepared_schema.authority,
        committed: Some(RelationalCommittedBranchRoot {
            descriptor: super::RelationalBranchRootDescriptor::new(
                axes.storage_root,
                axes.schema_root,
            ),
            canonical_envelope: envelope,
            axes,
            publication_cost,
        }),
    });
    Ok(PreparedRelationalBranchRootCapture { root, next_issuer })
}

fn prepare_regions(
    issuer: &mut RelationalBranchRootIdentityIssuer,
    root_id: u64,
    partitions: &impl PartitionAccess,
    published_delta: &RelationalPublishedPartitionDelta,
    previous: Option<&Arc<RelationalBranchRoot>>,
    symbols: &crate::symbols::data::StringInterner,
) -> Result<PreparedRootRegions, RelationalBranchRootCaptureDenial> {
    let (regions, publication_cost) = match previous {
        Some(previous) if previous.committed.is_some() => {
            capture::build_incremental_regions(issuer, root_id, published_delta, previous, symbols)?
        }
        _ => {
            let projected = published_delta.projected_partitions_from_access(partitions);
            capture::build_initial_regions(issuer, root_id, &projected, symbols)?
        }
    };
    let (storage_root, content_accumulator) = axes::storage_root_for(&regions);
    Ok(PreparedRootRegions {
        regions,
        storage_root,
        content_accumulator,
        publication_cost,
    })
}

fn prepare_schema(
    issuer: &mut RelationalBranchRootIdentityIssuer,
    previous: Option<&Arc<RelationalBranchRoot>>,
    envelope: &CanonicalCommitEnvelope,
    registry: &crate::schema::data::RelationalSchemaRegistry,
) -> Result<PreparedRootSchema, RelationalBranchRootCaptureDenial> {
    let schema_root =
        crate::schema::data::schema_authority_snapshot_digest_bytes(&envelope.schema_authority);
    if let Some(previous) =
        previous.filter(|root| root.schema_authority.matches(&envelope.schema_authority))
    {
        return Ok(PreparedRootSchema {
            authority: Arc::clone(&previous.schema_authority),
            schema_root,
            allocation: PreparedSchemaAllocation::Reused,
        });
    }
    let allocation_id = issuer.issue_schema_authority_id()?;
    let authority = RelationalBranchRootSchemaAuthority::capture(
        allocation_id,
        registry,
        &envelope.schema_authority,
        envelope.descriptor_semantics_version,
        previous.map(|root| root.schema_authority.as_ref()),
    )
    .ok_or(RelationalBranchRootCaptureDenial::SchemaRootMismatch {
        descriptor_root: schema_root,
        canonical_root: registry.authority_digest_bytes(),
    })?;
    Ok(PreparedRootSchema {
        allocation: PreparedSchemaAllocation::New {
            authoritative_bytes: authority.authoritative_allocation_bytes(),
        },
        authority,
        schema_root,
    })
}

impl PreparedRootSchema {
    fn apply_cost(
        &self,
        mut cost: RelationalBranchRootPublicationCost,
    ) -> RelationalBranchRootPublicationCost {
        match self.allocation {
            PreparedSchemaAllocation::Reused => cost.reused_schema_authorities = 1,
            PreparedSchemaAllocation::New {
                authoritative_bytes,
            } => {
                cost.new_schema_authorities = 1;
                cost.new_authoritative_bytes = cost
                    .new_authoritative_bytes
                    .saturating_add(authoritative_bytes);
            }
        }
        cost
    }
}
