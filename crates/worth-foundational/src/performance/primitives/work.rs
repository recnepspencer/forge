use super::{
    FoundationalPerformancePrimitiveDefinition, FoundationalPerformanceWorkClassDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceWorkClass {
    AuthoritativeRead,
    AuthoritativeMutation,
    AuthoritativeObservation,
    ValidationPlanning,
    PublicationDelivery,
    ReplayReconstruction,
    SupportReportAssembly,
    ForensicParity,
}

pub fn foundational_performance_work_class_definitions(
) -> [FoundationalPerformanceWorkClassDefinition; 8] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::AuthoritativeRead,
            "authoritative_read",
            "ordinary authoritative reads from the current admitted basis",
            "planning, mutation, publication, replay, or reconstruction",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            "authoritative_mutation",
            "authoritative mutation or state-advancing work",
            "publication, replay, or support assembly by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::AuthoritativeObservation,
            "authoritative_observation",
            "authoritative read or observation work that does not advance source truth",
            "mutation, publication, replay, or support assembly by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::ValidationPlanning,
            "validation_planning",
            "validation, shaping, or planning work performed within the named lane",
            "proof that mutation or replay also happened",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::PublicationDelivery,
            "publication_delivery",
            "publication or delivery work after the primary operation",
            "mutation, replay, or report assembly equivalence",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::ReplayReconstruction,
            "replay_reconstruction",
            "replay, reconstruction, or rehydration work",
            "fresh authoritative execution",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::SupportReportAssembly,
            "support_report_assembly",
            "support or report rows assembled for visibility or forensics",
            "narrow hot-path execution by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceWorkClass::ForensicParity,
            "forensic_parity",
            "forensic comparison or parity work that widens the lane",
            "ordinary narrow operational work",
        ),
    ]
}
