#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternReplayCounters {
    quotient_tiles_checked: usize,
    translation_rules_checked: usize,
    periodic_quotient_conflicts_checked: usize,
    color_holonomy_loops_checked: usize,
    translation_rotation_closures_checked: usize,
    substitution_certificates_checked: usize,
    finite_patch_extensions_checked: usize,
    query_declarations_performed: usize,
    screening_evaluations_performed: usize,
}

impl GeneratedPatternReplayCounters {
    pub(crate) fn new(
        quotient_tiles: usize,
        translation_rules: usize,
        periodic_quotient_conflicts: usize,
        color_holonomy_loops: usize,
        translation_rotation_closures: usize,
        substitution_certificates: usize,
        finite_patch_extensions: usize,
        query_declarations: usize,
        screening_evaluations: usize,
    ) -> Self {
        Self {
            quotient_tiles_checked: quotient_tiles,
            translation_rules_checked: translation_rules,
            periodic_quotient_conflicts_checked: periodic_quotient_conflicts,
            color_holonomy_loops_checked: color_holonomy_loops,
            translation_rotation_closures_checked: translation_rotation_closures,
            substitution_certificates_checked: substitution_certificates,
            finite_patch_extensions_checked: finite_patch_extensions,
            query_declarations_performed: query_declarations,
            screening_evaluations_performed: screening_evaluations,
        }
    }

    pub fn quotient_tiles_checked(&self) -> usize {
        self.quotient_tiles_checked
    }

    pub fn translation_rules_checked(&self) -> usize {
        self.translation_rules_checked
    }

    pub fn wraparound_checks_performed(&self) -> usize {
        self.translation_rules_checked
    }

    pub fn periodic_quotient_conflicts_checked(&self) -> usize {
        self.periodic_quotient_conflicts_checked
    }

    pub fn color_holonomy_loops_checked(&self) -> usize {
        self.color_holonomy_loops_checked
    }

    pub fn translation_rotation_closures_checked(&self) -> usize {
        self.translation_rotation_closures_checked
    }

    pub fn substitution_certificates_checked(&self) -> usize {
        self.substitution_certificates_checked
    }

    pub fn finite_patch_extensions_checked(&self) -> usize {
        self.finite_patch_extensions_checked
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.query_declarations_performed
    }

    pub fn screening_evaluations_performed(&self) -> usize {
        self.screening_evaluations_performed
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.quotient_tiles_checked,
            self.translation_rules_checked,
            self.periodic_quotient_conflicts_checked,
            self.color_holonomy_loops_checked,
            self.translation_rotation_closures_checked,
            self.substitution_certificates_checked,
            self.finite_patch_extensions_checked,
            self.query_declarations_performed,
            self.screening_evaluations_performed
        )
    }
}
