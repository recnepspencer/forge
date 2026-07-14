use crate::PhysicalFoundationEvidenceField;

#[test]
fn physical_substrate_foundational_residency_fields_keep_counter_boundary_terms_distinct() {
    let fields =
        PhysicalFoundationEvidenceField::required_for_physical_substrate_foundational_residency();

    assert_eq!(
        fields,
        [
            PhysicalFoundationEvidenceField::CounterSnapshot,
            PhysicalFoundationEvidenceField::PhysicalLayoutReport,
            PhysicalFoundationEvidenceField::FoundationalProfileMaterializationPlan,
            PhysicalFoundationEvidenceField::FoundationalProvenanceSupportTruth,
            PhysicalFoundationEvidenceField::FoundationalCounterBackedPerformanceReceipt,
        ]
    );
}
