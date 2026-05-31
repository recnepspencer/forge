//! Public API boundary for `worth-schema`.

pub mod platform {
    //! Worth-owned platform descriptor surfaces.
    //!
    //! These modules expose Worth semantic catalogs that initialize lower
    //! platform layers. They are not the ordinary lifecycle entry lane; Query
    //! owns that lifecycle.

    pub mod relations {
        //! Worth relation catalogs used for platform registration and truth
        //! descriptor assembly.

        pub use crate::data::relations::{
            DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
            TopologyRelationKind,
        };
    }

    pub mod entities {
        //! Worth entity catalogs used for platform registration and truth
        //! descriptor assembly.

        pub use crate::data::entities::{
            DiagnosticsEntityKind, EntityKind, GeometryEntityKind, NamingEntityKind,
            TopologyEntityKind,
        };
    }

    pub mod aspects {
        //! Worth aspect catalogs used for platform registration and truth
        //! descriptor assembly.

        pub use crate::data::aspects::{
            Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
        };
    }

    pub mod authority {
        //! Worth authority vocabulary used for platform descriptor assembly,
        //! write-side truth authoring, and related domain semantics.

        pub use crate::data::authority::commit_flow::{
            CreateKey, EntityReference, MutationOrigin, RawTopologyIntent, TopologyMutation,
        };
        pub use crate::data::authority::derived_invalidation::{
            milestone_two_invalidation_declarations, DerivedInvalidationTarget,
            DerivedTruthSurfaceKind, TruthToDerivedInvalidationDeclaration,
        };
        pub use crate::data::authority::geometry_binding::{
            CoedgeCurveKind, CurveBindingKind, CurveProvenanceKind, SurfaceBindingKind,
            SurfaceRelationKind, VertexGeometryProvenanceKind, VertexToleranceRegime,
        };
        pub use crate::data::authority::interpretation::{
            ShellInterpretationClass, ShellInterpretationRecord, TopologyInterpretationRecordSet,
            WireInterpretationClass, WireInterpretationRecord,
        };
        pub use crate::data::authority::precision_fallback::{
            FallbackDisposition, FallbackProofClass, PrecisionBudgetFallbackRecord,
            PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime,
        };
        pub use crate::data::authority::topology_class::TopologyClass;
    }
}

pub mod topology_authoring {
    pub use crate::data::authority::commit_flow::{
        AuthoritativeTopologySnapshot, CertifiedTopologyInterpretation, DerivedTopologyReadBasis,
        DerivedTruthBasisIdentity, PersistedTopologyTruth, TopologyCommittedMutationSet,
        TopologyReadArtifact,
    };
    pub use crate::topology_authoring::{
        build_milestone_one_primitive_intent, build_minimal_topology_intent,
        commit_topology_intent, commit_topology_intent_on_branch, commit_topology_mutation_set,
        commit_topology_mutation_set_on_branch, created_ref,
        milestone_one_admitted_range_sweep_out_of_class_scenarios,
        milestone_one_admitted_range_sweep_scenarios, milestone_one_default_primitive_corpus,
        milestone_one_heavy_branch_local_sweep_scenarios, seed_milestone_one_primitive,
        seed_milestone_one_primitive_on_branch, seed_minimal_topology,
        seed_minimal_topology_commit, MilestoneOnePrimitiveAuthoringError,
        MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
        MilestoneOnePrimitiveScenario, MinimalTopologySeed, SeededTopologyCommit,
        TopologyCreateBatchBuilder, TopologyIntentCommitError, TopologyMutationSetCommitError,
    };
}

pub use crate::data::bootstrap::{
    bootstrap_schema_registry, SchemaBuildError, SchemaBuilder, SCHEMA_ID, SCHEMA_VERSION_ID,
};
pub use crate::data::query::{
    query_aspect_path_strings, query_aspect_paths, query_aspect_paths_from_set, QueryAspectFamily,
    QueryAspectPath, QueryCollection, QueryLiveField, QuerySchemaBasis,
};
