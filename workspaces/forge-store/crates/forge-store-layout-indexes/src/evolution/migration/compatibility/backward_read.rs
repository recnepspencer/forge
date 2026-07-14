use forge_store_compatibility::BackwardReadCompatibilityWitness;

use super::{LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial};

#[derive(Debug, Clone, Copy)]
pub struct LayoutBackwardReadRequest<'a> {
    declaration: LayoutEvolutionDeclaration,
    binding: &'a LayoutBindingWitness,
    compatibility: BackwardReadCompatibilityWitness,
}

impl<'a> LayoutBackwardReadRequest<'a> {
    pub const fn new(
        declaration: LayoutEvolutionDeclaration,
        binding: &'a LayoutBindingWitness,
        compatibility: BackwardReadCompatibilityWitness,
    ) -> Self {
        Self {
            declaration,
            binding,
            compatibility,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBackwardReadEvidence {
    binding: LayoutBindingWitness,
    compatibility: BackwardReadCompatibilityWitness,
}

impl LayoutBackwardReadEvidence {
    pub const fn binding(&self) -> &LayoutBindingWitness {
        &self.binding
    }

    pub const fn compatibility(&self) -> BackwardReadCompatibilityWitness {
        self.compatibility
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LayoutBackwardReadCase {
    Admitted(Box<LayoutBackwardReadEvidence>),
    Denied(Box<LayoutEvolutionDenial>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBackwardReadOutcome {
    case: LayoutBackwardReadCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutBackwardReadView<'a> {
    Admitted(&'a LayoutBackwardReadEvidence),
    Denied(&'a LayoutEvolutionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBackwardReadCompatibilityCaseId(&'static str);

impl LayoutBackwardReadCompatibilityCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn layout_backward_read_compatibility_cases(
) -> impl Iterator<Item = LayoutBackwardReadCompatibilityCaseId> {
    [
        LayoutBackwardReadCompatibilityCaseId("layout.compatibility.backward_read.admitted"),
        LayoutBackwardReadCompatibilityCaseId(
            "layout.compatibility.backward_read.denied.window_mismatch",
        ),
        LayoutBackwardReadCompatibilityCaseId(
            "layout.compatibility.backward_read.denied.binding_mismatch",
        ),
        LayoutBackwardReadCompatibilityCaseId(
            "layout.compatibility.backward_read.denied.undeclared_version",
        ),
    ]
    .into_iter()
}

impl LayoutBackwardReadOutcome {
    fn admitted(evidence: LayoutBackwardReadEvidence) -> Self {
        Self {
            case: LayoutBackwardReadCase::Admitted(Box::new(evidence)),
        }
    }

    fn denied(denial: LayoutEvolutionDenial) -> Self {
        Self {
            case: LayoutBackwardReadCase::Denied(Box::new(denial)),
        }
    }

    pub const fn view(&self) -> LayoutBackwardReadView<'_> {
        match &self.case {
            LayoutBackwardReadCase::Admitted(evidence) => {
                LayoutBackwardReadView::Admitted(evidence)
            }
            LayoutBackwardReadCase::Denied(denial) => LayoutBackwardReadView::Denied(denial),
        }
    }

    pub fn case_id(&self) -> LayoutBackwardReadCompatibilityCaseId {
        match &self.case {
            LayoutBackwardReadCase::Admitted(_) => {
                LayoutBackwardReadCompatibilityCaseId("layout.compatibility.backward_read.admitted")
            }
            LayoutBackwardReadCase::Denied(denial)
                if matches!(
                    denial.as_ref(),
                    LayoutEvolutionDenial::CompatibilityAdmissionMismatch
                ) =>
            {
                LayoutBackwardReadCompatibilityCaseId(
                    "layout.compatibility.backward_read.denied.window_mismatch",
                )
            }
            LayoutBackwardReadCase::Denied(denial)
                if matches!(
                    denial.as_ref(),
                    LayoutEvolutionDenial::CompatibilityBindingVersionMismatch { .. }
                ) =>
            {
                LayoutBackwardReadCompatibilityCaseId(
                    "layout.compatibility.backward_read.denied.binding_mismatch",
                )
            }
            LayoutBackwardReadCase::Denied(_) => LayoutBackwardReadCompatibilityCaseId(
                "layout.compatibility.backward_read.denied.undeclared_version",
            ),
        }
    }

    pub fn into_admitted(self) -> Result<LayoutBackwardReadEvidence, LayoutEvolutionDenial> {
        match self.case {
            LayoutBackwardReadCase::Admitted(evidence) => Ok(*evidence),
            LayoutBackwardReadCase::Denied(denial) => Err(*denial),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutBackwardReadCompatibility;

pub const fn layout_backward_read_compatibility() -> LayoutBackwardReadCompatibility {
    LayoutBackwardReadCompatibility
}

impl LayoutBackwardReadCompatibility {
    pub fn admit(self, request: LayoutBackwardReadRequest<'_>) -> LayoutBackwardReadOutcome {
        if request.compatibility.window()
            != request.declaration.compatibility_window().artifact_window()
        {
            return LayoutBackwardReadOutcome::denied(
                LayoutEvolutionDenial::CompatibilityAdmissionMismatch,
            );
        }
        if request.compatibility.admitted_version()
            != request.binding.bound_version().format_version()
        {
            return LayoutBackwardReadOutcome::denied(
                LayoutEvolutionDenial::CompatibilityBindingVersionMismatch {
                    binding: request.binding.bound_version(),
                    compatibility: request.compatibility.admitted_version(),
                },
            );
        }
        if !request
            .declaration
            .declares_readable_version(request.binding.bound_version())
        {
            return LayoutBackwardReadOutcome::denied(
                LayoutEvolutionDenial::UndeclaredCompatibleLayoutVersion {
                    source: request.binding.bound_version(),
                },
            );
        }
        LayoutBackwardReadOutcome::admitted(LayoutBackwardReadEvidence {
            binding: request.binding.clone(),
            compatibility: request.compatibility,
        })
    }
}
