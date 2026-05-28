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

        pub use crate::data::authority::{
            milestone_two_invalidation_declarations, AuthoritativeTopologySnapshot,
            CanonicalTopologyMutationBatch, CertifiedTopologyInterpretation, CoedgeCurveKind,
            CreateKey, CurveBindingKind, CurveProvenanceKind, DerivedInvalidationTarget,
            DerivedTopologyReadBasis, DerivedTruthBasisIdentity, DerivedTruthSurfaceKind,
            EntityReference, FallbackDisposition, FallbackProofClass, MutationOrigin,
            PersistedTopologyTruthBatch, PrecisionBudgetFallbackRecord,
            PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime, RawTopologyIntent,
            ShellInterpretationClass, ShellInterpretationRecord, SurfaceBindingKind,
            SurfaceRelationKind, TopologyClass, TopologyInterpretationRecordSet,
            TopologyMutation, TopologyMutationBatch, TopologyReadArtifact,
            TruthToDerivedInvalidationDeclaration, VertexGeometryProvenanceKind,
            VertexToleranceRegime, WireInterpretationClass, WireInterpretationRecord,
        };
    }
}

pub mod topology_authoring {
    pub use crate::topology_authoring::{
        build_milestone_one_primitive_intent, created_ref,
        milestone_one_admitted_range_sweep_out_of_class_scenarios,
        milestone_one_admitted_range_sweep_scenarios, milestone_one_default_primitive_corpus,
        milestone_one_heavy_branch_local_sweep_scenarios, seed_milestone_one_primitive,
        seed_milestone_one_primitive_on_branch, seed_minimal_topology,
        MilestoneOnePrimitiveAuthoringError, MilestoneOnePrimitiveCase,
        MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
        MilestoneOnePrimitiveScenario, MinimalTopologySeed, TopologyCreateBatchBuilder,
    };
}

pub use crate::data::bootstrap::{
    bootstrap_schema_registry, SchemaBuildError, SchemaBuilder, SCHEMA_ID, SCHEMA_VERSION_ID,
};
pub use crate::data::query::{
    query_aspect_path_strings, query_aspect_paths, query_aspect_paths_from_set, QueryAspectFamily,
    QueryAspectPath, QueryCollection, QueryLiveField, QuerySchemaBasis,
};
