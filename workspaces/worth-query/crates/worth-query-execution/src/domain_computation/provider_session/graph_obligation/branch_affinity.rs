use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::{RelationalExecutionBasisIdentity, RelationalRuntime};
use worth_relational::facade::snapshots::SnapshotHandle;
use worth_runtime_bridge::facade::TruthBranchIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryGraphWorkBranchAffinity {
    relational_branch: BranchId,
    truth_branch: TruthBranchIdentity,
}

impl WorthQueryGraphWorkBranchAffinity {
    pub(in crate::domain_computation) fn from_installed_runtime(
        runtime: &RelationalRuntime,
    ) -> Self {
        let relational_branch = runtime.config().history.main_branch.clone();
        let truth_branch =
            TruthBranchIdentity::from_relational_branch_id(relational_branch.0.clone());
        Self {
            relational_branch,
            truth_branch,
        }
    }

    pub(in crate::domain_computation) const fn relational_branch(&self) -> &BranchId {
        &self.relational_branch
    }

    pub(in crate::domain_computation) const fn truth_branch(&self) -> &TruthBranchIdentity {
        &self.truth_branch
    }

    pub(super) fn admits_snapshot(&self, snapshot: &SnapshotHandle) -> bool {
        snapshot.branch_id == self.relational_branch
    }

    pub(super) fn admits_execution_basis(&self, basis: &RelationalExecutionBasisIdentity) -> bool {
        basis.branch_id() == &self.relational_branch
    }
}

#[cfg(test)]
mod tests {
    use worth_relational::facade::history::BranchId;
    use worth_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeConfig};

    use super::WorthQueryGraphWorkBranchAffinity;
    use crate::domain_computation::provider_session::{
        WorthQueryGraphWorkBasisAffinity, WorthQueryGraphWorkSessionStartDenial,
    };

    #[test]
    fn equal_snapshot_ordinals_from_another_branch_cannot_enter_mutation_basis() {
        let mut runtime = runtime_on("ordinary");
        let affinity = WorthQueryGraphWorkBranchAffinity::from_installed_runtime(&runtime);
        let snapshot = runtime.snapshots().snapshot();
        let mut foreign = snapshot.clone();
        foreign.branch_id = BranchId("foreign".to_owned());

        assert!(WorthQueryGraphWorkBasisAffinity::mutation(&snapshot, &affinity).is_ok());
        assert!(matches!(
            WorthQueryGraphWorkBasisAffinity::mutation(&foreign, &affinity),
            Err(WorthQueryGraphWorkSessionStartDenial::BranchMismatch)
        ));

        runtime.snapshots().release_snapshot(&snapshot);
    }

    #[test]
    fn equal_version_execution_basis_from_another_branch_is_rejected() {
        let mut ordinary = runtime_on("ordinary");
        let mut foreign = runtime_on("foreign");
        let affinity = WorthQueryGraphWorkBranchAffinity::from_installed_runtime(&ordinary);
        let ordinary_version = seed_commit(&mut ordinary, "ordinary");
        let foreign_version = seed_commit(&mut foreign, "foreign");
        assert_eq!(ordinary_version, foreign_version);
        let ordinary_basis = ordinary
            .snapshots()
            .admit_execution_basis(&BranchId("ordinary".to_owned()), ordinary_version)
            .expect("initial ordinary basis");
        let foreign_basis = foreign
            .snapshots()
            .admit_execution_basis(&BranchId("foreign".to_owned()), foreign_version)
            .expect("initial foreign basis");

        assert!(
            WorthQueryGraphWorkBasisAffinity::query(ordinary_basis.identity(), &affinity).is_ok()
        );
        assert!(matches!(
            WorthQueryGraphWorkBasisAffinity::query(foreign_basis.identity(), &affinity),
            Err(WorthQueryGraphWorkSessionStartDenial::BranchMismatch)
        ));

        assert!(ordinary_basis.release().released());
        assert!(foreign_basis.release().released());
    }

    fn runtime_on(branch: &str) -> RelationalRuntime {
        use worth_relational::facade::identity::KindId;
        use worth_relational::facade::schema::{
            EntityKindRegistration, KindAspectContractDeclarations, RelationalSchemaRegistry,
            SchemaId, SchemaVersionId,
        };

        let mut config = RelationalRuntimeConfig::default();
        config.history.main_branch = BranchId(branch.to_owned());
        config.schema.registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "branch-affinity.entity".to_owned(),
                schema_id: SchemaId("branch-affinity".to_owned()),
                schema_version_id: SchemaVersionId(1),
                aspect_contract_declarations: KindAspectContractDeclarations::default(),
            })
            .expect("branch fixture schema");
        RelationalRuntime::new(config)
    }

    fn seed_commit(
        runtime: &mut RelationalRuntime,
        label: &str,
    ) -> worth_relational::facade::identity::VersionId {
        use worth_relational::facade::identity::{KindId, PartitionId};
        use worth_relational::facade::symbols::ClientKey;
        use worth_relational::facade::transactions::{
            AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent, TransactionOptions,
            WorkerIntentBatch,
        };

        let branch = runtime.config().history.main_branch.clone();
        let mut transaction = runtime.begin_transaction(TransactionOptions::for_branch(branch));
        transaction.push_batch(WorkerIntentBatch::new(label).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: ClientKey::raw(label),
                fields: AspectFieldPatch::default(),
            }),
        )));
        let committed = transaction.commit().expect("branch fixture commit");
        let version = committed.snapshot.version_id;
        assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
        version
    }
}
