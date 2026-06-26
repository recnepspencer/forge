use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationOperatorCutoverSourceFirewallViolation {
    source_path: &'static str,
    forbidden_surface: &'static str,
    owner: &'static str,
    removal_trigger: &'static str,
}

impl DerivedInvalidationOperatorCutoverSourceFirewallViolation {
    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn forbidden_surface(&self) -> &'static str {
        self.forbidden_surface
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationOperatorCutoverSourceFirewall {
    violations: Vec<DerivedInvalidationOperatorCutoverSourceFirewallViolation>,
    report_digest: String,
}

impl DerivedInvalidationOperatorCutoverSourceFirewall {
    pub(crate) fn from_current_sources() -> Self {
        Self::from_source_inputs(current_source_inputs())
    }

    fn from_source_inputs(sources: impl IntoIterator<Item = SourceInput>) -> Self {
        let violations = collect_forbidden_source_violations(sources);
        let mut parts = vec![
            "worth-topo:derived-invalidation-operator-cutover-source-firewall:v1".to_string(),
            format!("violations:{}", violations.len()),
        ];
        parts.extend(violations.iter().map(|violation| {
            format!(
                "violation:{}:{}",
                violation.source_path, violation.forbidden_surface
            )
        }));
        let report_digest = super::super::catalog::catalog_digest(parts);
        Self {
            violations,
            report_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_sources_for_tests(
        sources: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Self {
        Self::from_source_inputs(
            sources
                .into_iter()
                .map(|(path, contents)| SourceInput { path, contents }),
        )
    }

    pub fn violations(&self) -> &[DerivedInvalidationOperatorCutoverSourceFirewallViolation] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn current_operator_cutover_source_firewall() -> DerivedInvalidationOperatorCutoverSourceFirewall
{
    DerivedInvalidationOperatorCutoverSourceFirewall::from_current_sources()
}

#[derive(Clone, Copy)]
struct SourceInput {
    path: &'static str,
    contents: &'static str,
}

#[derive(Clone, Copy)]
struct ForbiddenPattern {
    forbidden_surface: &'static str,
    owner: &'static str,
    removal_trigger: &'static str,
}

fn collect_forbidden_source_violations(
    sources: impl IntoIterator<Item = SourceInput>,
) -> Vec<DerivedInvalidationOperatorCutoverSourceFirewallViolation> {
    sources
        .into_iter()
        .flat_map(|source| {
            forbidden_patterns()
                .into_iter()
                .filter(move |pattern| source.contents.contains(pattern.forbidden_surface))
                .map(
                    move |pattern| DerivedInvalidationOperatorCutoverSourceFirewallViolation {
                        source_path: source.path,
                        forbidden_surface: pattern.forbidden_surface,
                        owner: pattern.owner,
                        removal_trigger: pattern.removal_trigger,
                    },
                )
        })
        .collect()
}

fn current_source_inputs() -> [SourceInput; 5] {
    [
        SourceInput {
            path: "topology_operators/application/mod.rs",
            contents: include_str!("../../../topology_operators/application/mod.rs"),
        },
        SourceInput {
            path: "topology_operators/application/declared_mutation_artifact.rs",
            contents: include_str!(
                "../../../topology_operators/application/declared_mutation_artifact.rs"
            ),
        },
        SourceInput {
            path: "projection/runtime_boundary/read_stage.rs",
            contents: include_str!("../../../projection/runtime_boundary/read_stage.rs"),
        },
        SourceInput {
            path: "projection/runtime_boundary/query_runtime/operator_post_write.rs",
            contents: include_str!(
                "../../../projection/runtime_boundary/query_runtime/operator_post_write.rs"
            ),
        },
        SourceInput {
            path: "derived_topology/invalidation_plan/operator_cutover/mod.rs",
            contents: include_str!("mod.rs"),
        },
    ]
}

fn forbidden_patterns() -> [ForbiddenPattern; 6] {
    [
        ForbiddenPattern {
            forbidden_surface: "operator_dirty_products",
            owner: "derived-invalidation operator cutover",
            removal_trigger: "derive dirty product scope from selected invalidation plan",
        },
        ForbiddenPattern {
            forbidden_surface: "dirty_product_expectations",
            owner: "derived-invalidation operator cutover",
            removal_trigger: "replace local expectation arrays with execution receipt rows",
        },
        ForbiddenPattern {
            forbidden_surface: "derived_expectation_array",
            owner: "derived-invalidation operator cutover",
            removal_trigger: "consume covered product rows from Phase 6 and execution receipts",
        },
        ForbiddenPattern {
            forbidden_surface: "expand_dirty_scope",
            owner: "projection read-stage cutover",
            removal_trigger: "route expansion through selected invalidation plan",
        },
        ForbiddenPattern {
            forbidden_surface: "fallback_policy_accepted_as_invalidation",
            owner: "operator closeout cutover",
            removal_trigger:
                "require Milestone 10 execution receipt instead of fallback acceptance",
        },
        ForbiddenPattern {
            forbidden_surface: "old_dirty_data_to_invalidation_receipt",
            owner: "derived-invalidation operator cutover",
            removal_trigger: "mint receipts only from selected invalidation plan execution",
        },
    ]
}
