use crate::ordinary::{mutation, workflow};
use crate::runtime::WorthQueryWorkspace;
use crate::session_label::WorthQuerySessionLabel;

/// Domain extension contract for contributing declarative mutation meaning.
///
/// Implementations contribute vocabulary only. They receive no runtime,
/// preview session, lowered plan, or mutation authority.
pub trait WorthQueryDomainWorkflowContribution {
    type Error;

    fn contribute(&self) -> Result<mutation::WorthQueryMutationDeclaration, Self::Error>;
}

pub struct WorthQueryDomainWorkflowDeclaration {
    workflow: workflow::WorthQueryWorkflowDeclaration,
}

impl WorthQueryDomainWorkflowDeclaration {
    pub fn identity(&self) -> &workflow::WorthQueryWorkflowDeclarationIdentity {
        self.workflow.identity()
    }

    pub fn using(
        self,
        context: WorthQueryDomainWorkflowContext,
    ) -> WorthQueryDomainWorkflowRequest {
        WorthQueryDomainWorkflowRequest {
            workflow: self.workflow.using(context.workflow),
        }
    }
}

pub fn declare<C>(
    label: WorthQuerySessionLabel,
    contribution: C,
) -> Result<WorthQueryDomainWorkflowDeclaration, C::Error>
where
    C: WorthQueryDomainWorkflowContribution,
{
    contribution
        .contribute()
        .map(|mutation| WorthQueryDomainWorkflowDeclaration {
            workflow: workflow::declare(label, mutation),
        })
}

pub struct WorthQueryDomainWorkflowContext {
    workflow: workflow::WorthQueryWorkflowContext,
}

pub type WorthQueryDomainWorkflowContextStop = workflow::WorthQueryWorkflowContextStop;

pub fn preview(
    workspace: &WorthQueryWorkspace,
    label: WorthQuerySessionLabel,
) -> Result<WorthQueryDomainWorkflowContext, WorthQueryDomainWorkflowContextStop> {
    workflow::preview(workspace, label).map(|workflow| WorthQueryDomainWorkflowContext { workflow })
}

pub struct WorthQueryDomainWorkflowRequest {
    workflow: workflow::WorthQueryWorkflowRequest,
}

impl WorthQueryDomainWorkflowRequest {
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryDomainWorkflowOutcome {
        match self.workflow.run(workspace) {
            workflow::WorthQueryWorkflowOutcome::Completed(completion) => {
                WorthQueryDomainWorkflowOutcome::Completed(WorthQueryDomainWorkflowCompletion {
                    workflow: completion,
                })
            }
            workflow::WorthQueryWorkflowOutcome::Stopped(stop) => {
                WorthQueryDomainWorkflowOutcome::Stopped(stop)
            }
        }
    }
}

pub struct WorthQueryDomainWorkflowCompletion {
    workflow: workflow::WorthQueryWorkflowCompletion,
}

impl WorthQueryDomainWorkflowCompletion {
    pub fn workflow(&self) -> &workflow::WorthQueryWorkflowCompletion {
        &self.workflow
    }
}

pub enum WorthQueryDomainWorkflowOutcome {
    Completed(WorthQueryDomainWorkflowCompletion),
    Stopped(workflow::WorthQueryWorkflowStop),
}

impl WorthQueryDomainWorkflowOutcome {
    pub fn completed(&self) -> Option<&WorthQueryDomainWorkflowCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&workflow::WorthQueryWorkflowStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }
}
