pub const UI_APPEARANCE_DECISION_CELL_CAPACITY: usize = 512;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiAppearanceStateAxis {
    Operability,
    Focus,
    Validation,
    Selection,
    Hover,
    Pressed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiAppearanceStateAxisVersion {
    axis: UiAppearanceStateAxis,
    revision: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiAppearanceAxisClass {
    OperabilityReady,
    OperabilityPending,
    OperabilityOccupied,
    OperabilityDenied,
    OperabilityUnsupported,
    OperabilityStale,
    FocusUnfocused,
    FocusFocused,
    FocusVisible,
    FocusedWindowInactive,
    ValidationUnspecified,
    ValidationValid,
    ValidationAdvisory,
    ValidationInvalid,
    ValidationPending,
    ValidationStale,
    SelectionUnselected,
    SelectionSelected,
    SelectionAnchor,
    SelectionCursor,
    SelectedAnchorCursor,
    HoverOutside,
    Hovered,
    PressedIdle,
    PressedArmedInside,
    PressedCapturedOutside,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAppearanceAxisDomain {
    version: UiAppearanceStateAxisVersion,
    classes: Box<[UiAppearanceAxisClass]>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiAppearanceDecisionResult {
    slot: super::UiThemeSlotIdentity,
    value_kind: super::UiThemeValueKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAppearanceDecisionRule {
    predicates: Box<[UiAppearanceAxisPredicate]>,
    result: UiAppearanceDecisionResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAppearanceAxisPredicate {
    axis: UiAppearanceStateAxis,
    class: Option<UiAppearanceAxisClass>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAppearanceDecisionCell {
    classes: Box<[UiAppearanceAxisClass]>,
    result: UiAppearanceDecisionResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAppearanceDecisionPartition {
    axes: Box<[UiAppearanceStateAxisVersion]>,
    cells: Box<[UiAppearanceDecisionCell]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAppearanceDecisionPartitionDenial {
    DuplicateAxis,
    PredicateArity,
    DuplicatePredicateAxis,
    MissingPredicateAxis,
    PredicateClassMismatch,
    CellCapacityExceeded,
    AmbiguousCell,
    MissingCell,
}

impl UiAppearanceStateAxisVersion {
    pub const fn current(axis: UiAppearanceStateAxis) -> Self {
        Self { axis, revision: 1 }
    }

    pub const fn axis(self) -> UiAppearanceStateAxis {
        self.axis
    }
    pub const fn revision(self) -> u16 {
        self.revision
    }
}

impl UiAppearanceAxisClass {
    pub const fn axis(self) -> UiAppearanceStateAxis {
        match self {
            Self::OperabilityReady
            | Self::OperabilityPending
            | Self::OperabilityOccupied
            | Self::OperabilityDenied
            | Self::OperabilityUnsupported
            | Self::OperabilityStale => UiAppearanceStateAxis::Operability,
            Self::FocusUnfocused
            | Self::FocusFocused
            | Self::FocusVisible
            | Self::FocusedWindowInactive => UiAppearanceStateAxis::Focus,
            Self::ValidationUnspecified
            | Self::ValidationValid
            | Self::ValidationAdvisory
            | Self::ValidationInvalid
            | Self::ValidationPending
            | Self::ValidationStale => UiAppearanceStateAxis::Validation,
            Self::SelectionUnselected
            | Self::SelectionSelected
            | Self::SelectionAnchor
            | Self::SelectionCursor
            | Self::SelectedAnchorCursor => UiAppearanceStateAxis::Selection,
            Self::HoverOutside | Self::Hovered => UiAppearanceStateAxis::Hover,
            Self::PressedIdle | Self::PressedArmedInside | Self::PressedCapturedOutside => {
                UiAppearanceStateAxis::Pressed
            }
        }
    }
}

impl UiAppearanceAxisDomain {
    pub fn complete(axis: UiAppearanceStateAxis) -> Self {
        Self {
            version: UiAppearanceStateAxisVersion::current(axis),
            classes: complete_classes(axis).into(),
        }
    }

    pub const fn version(&self) -> UiAppearanceStateAxisVersion {
        self.version
    }
    pub fn classes(&self) -> &[UiAppearanceAxisClass] {
        &self.classes
    }
}

fn complete_classes(axis: UiAppearanceStateAxis) -> &'static [UiAppearanceAxisClass] {
    use UiAppearanceAxisClass::*;
    match axis {
        UiAppearanceStateAxis::Operability => &[
            OperabilityReady,
            OperabilityPending,
            OperabilityOccupied,
            OperabilityDenied,
            OperabilityUnsupported,
            OperabilityStale,
        ],
        UiAppearanceStateAxis::Focus => &[
            FocusUnfocused,
            FocusFocused,
            FocusVisible,
            FocusedWindowInactive,
        ],
        UiAppearanceStateAxis::Validation => &[
            ValidationUnspecified,
            ValidationValid,
            ValidationAdvisory,
            ValidationInvalid,
            ValidationPending,
            ValidationStale,
        ],
        UiAppearanceStateAxis::Selection => &[
            SelectionUnselected,
            SelectionSelected,
            SelectionAnchor,
            SelectionCursor,
            SelectedAnchorCursor,
        ],
        UiAppearanceStateAxis::Hover => &[HoverOutside, Hovered],
        UiAppearanceStateAxis::Pressed => {
            &[PressedIdle, PressedArmedInside, PressedCapturedOutside]
        }
    }
}

impl UiAppearanceDecisionResult {
    pub fn theme_slot(
        slot: super::UiThemeSlotIdentity,
        value_kind: super::UiThemeValueKind,
    ) -> Self {
        Self { slot, value_kind }
    }
    pub const fn slot(&self) -> &super::UiThemeSlotIdentity {
        &self.slot
    }
    pub const fn value_kind(&self) -> super::UiThemeValueKind {
        self.value_kind
    }
}

impl UiAppearanceDecisionRule {
    pub fn new(
        predicates: impl IntoIterator<Item = UiAppearanceAxisPredicate>,
        result: UiAppearanceDecisionResult,
    ) -> Self {
        Self {
            predicates: predicates.into_iter().collect(),
            result,
        }
    }
}

impl UiAppearanceAxisPredicate {
    pub const fn any(axis: UiAppearanceStateAxis) -> Self {
        Self { axis, class: None }
    }

    pub const fn exact(class: UiAppearanceAxisClass) -> Self {
        Self {
            axis: class.axis(),
            class: Some(class),
        }
    }

    pub const fn axis(self) -> UiAppearanceStateAxis {
        self.axis
    }

    pub const fn class(self) -> Option<UiAppearanceAxisClass> {
        self.class
    }
}

impl UiAppearanceDecisionPartition {
    pub fn compile(
        domains: impl IntoIterator<Item = UiAppearanceAxisDomain>,
        rules: impl IntoIterator<Item = UiAppearanceDecisionRule>,
    ) -> Result<Self, UiAppearanceDecisionPartitionDenial> {
        let mut domains = domains.into_iter().collect::<Vec<_>>();
        domains.sort_by_key(UiAppearanceAxisDomain::version);
        if domains
            .windows(2)
            .any(|pair| pair[0].version.axis == pair[1].version.axis)
        {
            return Err(UiAppearanceDecisionPartitionDenial::DuplicateAxis);
        }
        let cell_count = admit_cell_count(domains.iter().map(|domain| domain.classes.len()))?;
        let rules = rules.into_iter().collect::<Vec<_>>();
        validate_rules(&domains, &rules)?;
        let mut cells = Vec::with_capacity(cell_count);
        expand_cells(&domains, 0, &mut Vec::new(), &mut |classes| {
            let matches = rules
                .iter()
                .filter(|rule| rule_matches(rule, classes))
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [] => Err(UiAppearanceDecisionPartitionDenial::MissingCell),
                [rule] => {
                    cells.push(UiAppearanceDecisionCell {
                        classes: classes.to_vec().into_boxed_slice(),
                        result: rule.result.clone(),
                    });
                    Ok(())
                }
                _ => Err(UiAppearanceDecisionPartitionDenial::AmbiguousCell),
            }
        })?;
        Ok(Self {
            axes: domains
                .iter()
                .map(UiAppearanceAxisDomain::version)
                .collect(),
            cells: cells.into_boxed_slice(),
        })
    }

    pub fn axes(&self) -> &[UiAppearanceStateAxisVersion] {
        &self.axes
    }
    pub fn cells(&self) -> &[UiAppearanceDecisionCell] {
        &self.cells
    }
}

fn admit_cell_count(
    cardinalities: impl IntoIterator<Item = usize>,
) -> Result<usize, UiAppearanceDecisionPartitionDenial> {
    let count = cardinalities
        .into_iter()
        .try_fold(1_usize, usize::checked_mul)
        .ok_or(UiAppearanceDecisionPartitionDenial::CellCapacityExceeded)?;
    if count > UI_APPEARANCE_DECISION_CELL_CAPACITY {
        Err(UiAppearanceDecisionPartitionDenial::CellCapacityExceeded)
    } else {
        Ok(count)
    }
}

fn validate_rules(
    domains: &[UiAppearanceAxisDomain],
    rules: &[UiAppearanceDecisionRule],
) -> Result<(), UiAppearanceDecisionPartitionDenial> {
    for rule in rules {
        if rule.predicates.len() != domains.len() {
            return Err(UiAppearanceDecisionPartitionDenial::PredicateArity);
        }
        let mut axes = rule
            .predicates
            .iter()
            .map(|predicate| predicate.axis)
            .collect::<Vec<_>>();
        axes.sort_unstable();
        if axes.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(UiAppearanceDecisionPartitionDenial::DuplicatePredicateAxis);
        }
        for domain in domains {
            let predicate = rule
                .predicates
                .iter()
                .find(|predicate| predicate.axis == domain.version.axis)
                .ok_or(UiAppearanceDecisionPartitionDenial::MissingPredicateAxis)?;
            if predicate
                .class
                .is_some_and(|class| !domain.classes.contains(&class))
            {
                return Err(UiAppearanceDecisionPartitionDenial::PredicateClassMismatch);
            }
        }
    }
    Ok(())
}

fn expand_cells(
    domains: &[UiAppearanceAxisDomain],
    index: usize,
    current: &mut Vec<UiAppearanceAxisClass>,
    emit: &mut impl FnMut(&[UiAppearanceAxisClass]) -> Result<(), UiAppearanceDecisionPartitionDenial>,
) -> Result<(), UiAppearanceDecisionPartitionDenial> {
    if index == domains.len() {
        return emit(current);
    }
    for class in domains[index].classes.iter().copied() {
        current.push(class);
        expand_cells(domains, index + 1, current, emit)?;
        current.pop();
    }
    Ok(())
}

fn rule_matches(rule: &UiAppearanceDecisionRule, cell: &[UiAppearanceAxisClass]) -> bool {
    cell.iter().all(|class| {
        rule.predicates
            .iter()
            .find(|predicate| predicate.axis == class.axis())
            .is_some_and(|predicate| predicate.class.is_none_or(|value| value == *class))
    })
}

impl UiAppearanceDecisionCell {
    pub fn classes(&self) -> &[UiAppearanceAxisClass] {
        &self.classes
    }
    pub const fn result(&self) -> &UiAppearanceDecisionResult {
        &self.result
    }
}

#[cfg(test)]
#[path = "state_partition_tests.rs"]
mod tests;
