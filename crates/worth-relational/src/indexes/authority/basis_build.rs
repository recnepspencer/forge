use crate::branch::AdmittedRelationalBranchBasis;
use crate::indexes::data::{DerivedIndexBuildOutcome, DerivedIndexBuildRequest};

use super::{
    choose_index_preparation_strategy, execute_index_packets, failed_build_outcome,
    plan_index_packets, planned_index_definitions, publish_index_generations,
    record_index_preparation_strategy_counters, IndexAuthority, IndexGenerationPublicationBasis,
    IndexProjectionSource,
};

impl IndexAuthority<'_> {
    /// Build against the exact immutable root already carried by an admitted
    /// owner basis. This path performs no historical retention reacquisition.
    pub fn build_for_basis(
        &mut self,
        request: DerivedIndexBuildRequest,
        basis: &AdmittedRelationalBranchBasis,
    ) -> DerivedIndexBuildOutcome {
        let mut generations = Vec::new();
        let mut failed_indexes = Vec::new();
        let observation = basis.observation();
        let Some(source_commit_id) = observation.commit_id() else {
            return failed_build_outcome(
                request.source_commit_id,
                generations,
                request.index_ids,
                None,
            );
        };
        if request.source_commit_id != source_commit_id {
            return failed_build_outcome(
                request.source_commit_id,
                generations,
                request.index_ids,
                Some(crate::branch::RelationalBranchBasisDenial::MixedAxis(
                    crate::branch::RelationalBranchBasisMismatchAxis::Commit,
                )),
            );
        }
        let source_branch_id = observation.identity().branch_id().clone();
        if request.branch_id != source_branch_id {
            return failed_build_outcome(
                request.source_commit_id,
                generations,
                request.index_ids,
                Some(crate::branch::RelationalBranchBasisDenial::MixedAxis(
                    crate::branch::RelationalBranchBasisMismatchAxis::Branch,
                )),
            );
        }
        let version_id = observation.version_id();
        let projection = match self.runtime.read_truth().project_observation(&observation) {
            Ok(projection) => projection,
            Err(denial) => {
                return failed_build_outcome(
                    request.source_commit_id,
                    generations,
                    request.index_ids,
                    Some(denial),
                );
            }
        };
        let projection = IndexProjectionSource::exact(&projection)
            .expect("owner-admitted observation projects one exact root");
        let (definitions, missing_indexes) =
            planned_index_definitions(self.runtime, &request.index_ids);
        failed_indexes.extend(missing_indexes);
        let strategy = choose_index_preparation_strategy(self.runtime, definitions.len());
        record_index_preparation_strategy_counters(self.runtime, definitions.len(), &strategy);
        let Some(schema_version) = projection.schema_version() else {
            failed_indexes.extend(definitions.iter().map(|definition| definition.index_id));
            return failed_build_outcome(
                request.source_commit_id,
                generations,
                failed_indexes,
                None,
            );
        };
        let packets = plan_index_packets(&definitions);
        let results =
            execute_index_packets(self.runtime, &projection, &packets, strategy.selected_mode);
        let publication_basis = IndexGenerationPublicationBasis::new(
            &request,
            source_branch_id,
            version_id,
            schema_version,
        );
        generations = publish_index_generations(
            self.runtime,
            &publication_basis,
            results,
            &mut failed_indexes,
        );
        self.publish_build_diagnostic(
            &request,
            &publication_basis.branch_id,
            &generations,
            &failed_indexes,
        );
        DerivedIndexBuildOutcome {
            source_commit_id: request.source_commit_id,
            generations,
            failed_indexes,
            basis_denial: None,
        }
    }
}
