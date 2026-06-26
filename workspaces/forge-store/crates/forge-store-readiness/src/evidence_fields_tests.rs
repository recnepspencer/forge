use crate::PhysicalFoundationEvidenceField;

#[test]
fn s2_foundational_residency_fields_keep_counter_boundary_terms_distinct() {
    let fields = PhysicalFoundationEvidenceField::required_for_s2_foundational_residency();

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
