#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use topology::facade::current_topology_query_consumer_kit_adoption_status;
    use worth_kernel::query_adoption::{
        assert_authority_promotion_allowed, current_kernel_query_consumer_kit_adoption_status,
        detect_seeded_forbidden_patterns, WorthQueryAdoptionClassification,
        WorthQueryAdoptionForbiddenPattern, WorthQueryAdoptionInventoryOwner,
        WorthQueryAdoptionInventoryReport, WorthQueryAuthorityBoundaryReport,
        WorthQueryAuthorityCategory, WorthQueryAuthorityDomain, WorthQueryAuthorityPromotionTarget,
    };
    use worth_spatial::facade::query_adoption::current_spatial_query_consumer_kit_adoption_status;

    #[test]
    fn query_adoption_inventory_covers_phase_one_source_sets() {
        let report = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 1 inventory should validate");

        for source_set in [
            "crates/worth-kernel/src/workload_composition",
            "crates/worth-spatial/src/workload_platform",
            "crates/worth-topo/src/projection/runtime_boundary",
            "crates/worth-topo/src/workload_platform",
            "crates/forge-query/src/consumer_kit/support_pinning",
            "crates/forge-query/src/consumer_kit/evidence_report",
        ] {
            let row = report
                .require_source_set(source_set)
                .unwrap_or_else(|| panic!("missing audited source set {source_set}"));
            assert_eq!(
                row.classification(),
                WorthQueryAdoptionClassification::Production
            );
            assert!(!row.responsibility().is_empty());
            assert!(!row.replacement_surface().is_empty());
        }

        assert_eq!(report.counters().audited_source_sets(), 17);
        assert_eq!(report.counters().production_source_sets(), 9);
        assert_eq!(report.counters().test_support_source_sets(), 2);
        assert_eq!(report.counters().certification_only_source_sets(), 3);
        assert_eq!(report.counters().explicit_residue_source_sets(), 3);
        assert_eq!(report.counters().source_sets_with_forbidden_patterns(), 13);
    }

    #[test]
    fn query_adoption_inventory_keeps_crate_ownership_localized() {
        let report = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 1 inventory should validate");

        assert_eq!(
            report
                .require_source_set("crates/worth-kernel/src/workload_composition")
                .expect("kernel workload composition row")
                .owner(),
            WorthQueryAdoptionInventoryOwner::Kernel
        );
        assert_eq!(
            report
                .require_source_set("crates/worth-spatial/src/workload_platform")
                .expect("spatial workload platform row")
                .owner(),
            WorthQueryAdoptionInventoryOwner::Spatial
        );
        assert_eq!(
            report
                .require_source_set("crates/worth-topo/src/projection/runtime_boundary")
                .expect("topology runtime boundary row")
                .owner(),
            WorthQueryAdoptionInventoryOwner::Topology
        );
        assert_eq!(
            report
                .require_source_set("crates/forge-query/src/consumer_kit/support_pinning")
                .expect("Query support row")
                .owner(),
            WorthQueryAdoptionInventoryOwner::ForgeQuery
        );
        assert_eq!(
            report
                .require_source_set("crates/forge-query/src/consumer_kit/evidence_report")
                .expect("Query evidence row")
                .owner(),
            WorthQueryAdoptionInventoryOwner::ForgeQuery
        );
    }

    #[test]
    fn query_adoption_inventory_keeps_forbidden_patterns_typed_across_crates() {
        let report = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 1 inventory should validate");

        assert_eq!(
            report
                .require_source_set("crates/worth-spatial/src/witness_resolution")
                .expect("spatial witness-resolution row")
                .forbidden_pattern(),
            Some(WorthQueryAdoptionForbiddenPattern::TestFixtureTruthPromotion)
        );
        assert_eq!(
            report
                .require_source_set("crates/worth-spatial/src/workload_platform/vocabulary")
                .expect("spatial workload vocabulary residue row")
                .forbidden_pattern(),
            Some(WorthQueryAdoptionForbiddenPattern::ForgedEvidenceRow)
        );
        assert_eq!(
            report
                .require_source_set("crates/worth-topo/src/projection/runtime_boundary")
                .expect("topology runtime boundary row")
                .forbidden_pattern(),
            None
        );
        assert_eq!(
            report
                .require_source_set(
                    "crates/worth-topo/src/projection/runtime_boundary/read_lowering/relationship_proof.rs"
                )
                .expect("topology relationship proof row")
                .forbidden_pattern(),
            None
        );
        assert_eq!(
            report
                .require_source_set(
                    "crates/worth-topo/src/projection/runtime_boundary/query_support"
                )
                .expect("topology query-support residue row")
                .forbidden_pattern(),
            Some(WorthQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption)
        );
    }

    #[test]
    fn query_adoption_inventory_replacement_surfaces_are_repo_locatable() {
        let report = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 3 inventory should validate");
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");

        for row in report.rows() {
            let replacement_surface = repo_root.join(row.replacement_surface());
            assert!(
                replacement_surface.exists(),
                "replacement surface must be locatable for {}: {}",
                row.source_set(),
                row.replacement_surface()
            );
        }
    }

    #[test]
    fn seeded_source_audit_localizes_forbidden_shortcuts() {
        let source = r#"
            let _receipt = SyntheticReceipt::manual receipt();
            let _row = ForgedEvidenceRow::hand_built();
            let support = support_posture_assumed();
            let identity = from_raw_identity("bridge-label");
        "#;

        let findings = detect_seeded_forbidden_patterns("seeded/source.rs", source);
        let patterns: Vec<_> = findings.iter().map(|finding| finding.pattern()).collect();

        assert_eq!(findings.len(), 4);
        assert!(patterns.contains(&WorthQueryAdoptionForbiddenPattern::SyntheticReceipt));
        assert!(patterns.contains(&WorthQueryAdoptionForbiddenPattern::ForgedEvidenceRow));
        assert!(
            patterns.contains(&WorthQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption)
        );
        assert!(patterns
            .contains(&WorthQueryAdoptionForbiddenPattern::LowerAuthorityIdentityReconstruction));
        assert!(findings
            .iter()
            .all(|finding| finding.source_name() == "seeded/source.rs"));
        assert!(findings
            .iter()
            .all(|finding| !finding.localized_phrase().is_empty()));
    }

    #[test]
    fn authority_boundary_report_keeps_docs_support_and_inventory_in_parity() {
        let inventory = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 2 inventory should validate");
        let boundary_report = WorthQueryAuthorityBoundaryReport::from_inventory(&inventory);

        assert_eq!(boundary_report.rows().len(), inventory.rows().len());
        assert!(boundary_report.all_rows_are_in_parity());

        for row in inventory.rows() {
            let projection = boundary_report
                .require_source_set(row.source_set())
                .unwrap_or_else(|| panic!("missing authority projection for {}", row.source_set()));
            assert_eq!(
                projection.machine_inventory_category(),
                row.authority_category()
            );
            assert_eq!(
                projection.support_report_category(),
                row.authority_category()
            );
            assert_eq!(projection.docs_category(), row.authority_category());
        }

        assert_eq!(
            inventory
                .require_source_set("crates/worth-topo/src/projection/runtime_boundary")
                .expect("topology runtime row")
                .authority_domain(),
            WorthQueryAuthorityDomain::TopologyTruth
        );
        assert_eq!(
            inventory
                .require_source_set("crates/worth-spatial/src/witness_resolution")
                .expect("spatial witness row")
                .authority_domain(),
            WorthQueryAuthorityDomain::SpatialWitnessTruth
        );
        assert_eq!(
            inventory
                .require_source_set("crates/worth-kernel/src/workload_composition")
                .expect("kernel composition row")
                .authority_category(),
            WorthQueryAuthorityCategory::Derived
        );
        assert_eq!(
            inventory
                .require_source_set("crates/forge-query/src/consumer_kit/support_pinning")
                .expect("query support row")
                .authority_domain(),
            WorthQueryAuthorityDomain::QuerySupport
        );
        assert_eq!(
            inventory
                .require_source_set("crates/forge-query/src/consumer_kit/evidence_report")
                .expect("query evidence row")
                .authority_domain(),
            WorthQueryAuthorityDomain::QueryEvidence
        );
    }

    #[test]
    fn lower_authority_rows_cannot_promote_into_truth_or_query_evidence_targets() {
        let inventory = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .expect("phase 2 inventory should validate");

        let kernel_row = inventory
            .require_source_set("crates/worth-kernel/src/workload_composition")
            .expect("kernel composition row");
        let spatial_evidence_row = inventory
            .require_source_set("crates/worth-spatial/src/workload_platform")
            .expect("spatial evidence row");
        let topology_residue_row = inventory
            .require_source_set("crates/worth-topo/src/projection/runtime_boundary/query_support")
            .expect("topology query support residue row");

        for target in [
            WorthQueryAuthorityPromotionTarget::TopologyTruth,
            WorthQueryAuthorityPromotionTarget::SpatialWitnessTruth,
            WorthQueryAuthorityPromotionTarget::SupportPin,
            WorthQueryAuthorityPromotionTarget::EvidenceReport,
        ] {
            assert!(assert_authority_promotion_allowed(kernel_row, target).is_err());
            assert!(assert_authority_promotion_allowed(topology_residue_row, target).is_err());
        }

        let spatial_denial = assert_authority_promotion_allowed(
            spatial_evidence_row,
            WorthQueryAuthorityPromotionTarget::TopologyTruth,
        )
        .expect_err("spatial evidence must not promote into topology truth");
        assert_eq!(
            spatial_denial.authority_domain(),
            WorthQueryAuthorityDomain::SpatialEvidence
        );

        assert!(assert_authority_promotion_allowed(
            inventory
                .require_source_set("crates/worth-topo/src/projection/runtime_boundary")
                .expect("topology runtime row"),
            WorthQueryAuthorityPromotionTarget::TopologyTruth
        )
        .is_ok());
        assert!(assert_authority_promotion_allowed(
            inventory
                .require_source_set("crates/worth-spatial/src/witness_resolution")
                .expect("spatial witness row"),
            WorthQueryAuthorityPromotionTarget::SpatialWitnessTruth
        )
        .is_ok());
        let query_support_row = inventory
            .require_source_set("crates/forge-query/src/consumer_kit/support_pinning")
            .expect("query support row");
        let query_evidence_row = inventory
            .require_source_set("crates/forge-query/src/consumer_kit/evidence_report")
            .expect("query evidence row");

        assert!(assert_authority_promotion_allowed(
            query_support_row,
            WorthQueryAuthorityPromotionTarget::SupportPin
        )
        .is_ok());
        assert!(assert_authority_promotion_allowed(
            query_evidence_row,
            WorthQueryAuthorityPromotionTarget::EvidenceReport
        )
        .is_ok());
        assert!(assert_authority_promotion_allowed(
            query_support_row,
            WorthQueryAuthorityPromotionTarget::EvidenceReport
        )
        .is_err());
        assert!(assert_authority_promotion_allowed(
            query_evidence_row,
            WorthQueryAuthorityPromotionTarget::SupportPin
        )
        .is_err());
    }

    #[test]
    fn query_consumer_kit_adoption_statuses_are_real_and_digest_backed() {
        let kernel = current_kernel_query_consumer_kit_adoption_status()
            .expect("kernel Query consumer-kit adoption status");
        let spatial = current_spatial_query_consumer_kit_adoption_status()
            .expect("spatial Query consumer-kit adoption status");
        let topology = current_topology_query_consumer_kit_adoption_status()
            .expect("topology Query consumer-kit adoption status");

        for (crate_name, support_digest, evidence_identity, boundary_identity) in [
            (
                "worth-kernel",
                kernel.support_pin_report_digest(),
                kernel.evidence_report_identity(),
                kernel.boundary_audit_report_identity(),
            ),
            (
                "worth-spatial",
                spatial.support_pin_report_digest(),
                spatial.evidence_report_identity(),
                spatial.boundary_audit_report_identity(),
            ),
            (
                "worth-topo",
                topology.support_pin_report_digest(),
                topology.evidence_report_identity(),
                topology.boundary_audit_report_identity(),
            ),
        ] {
            assert!(
                !support_digest.is_empty(),
                "{crate_name} support pin report must be Query evidence-backed"
            );
            assert!(
                !evidence_identity.is_empty(),
                "{crate_name} evidence report must be Query evidence-backed"
            );
            assert!(
                !boundary_identity.is_empty(),
                "{crate_name} boundary audit must be Query evidence-backed"
            );
            assert_ne!(
                support_digest, evidence_identity,
                "{crate_name} support pin and evidence report identities must be separately derived"
            );
            assert_ne!(
                evidence_identity, boundary_identity,
                "{crate_name} evidence report and boundary audit identities must be separately derived"
            );
        }

        assert_eq!(kernel.support_requirement_count(), 3);
        assert_eq!(spatial.support_requirement_count(), 3);
        assert_eq!(topology.support_requirement_count(), 3);
        assert_eq!(kernel.support_blocking_finding_count(), 0);
        assert_eq!(spatial.support_blocking_finding_count(), 0);
        assert_eq!(topology.support_blocking_finding_count(), 0);
        assert!(kernel.hard_prohibition_audit_clean());
        assert!(spatial.hard_prohibition_audit_clean());
        assert!(topology.hard_prohibition_audit_clean());
        assert_eq!(
            kernel.boundary_audit_coverage_row_count(),
            spatial.boundary_audit_coverage_row_count()
        );
        assert_eq!(
            spatial.boundary_audit_coverage_row_count(),
            topology.boundary_audit_coverage_row_count()
        );
    }
}
