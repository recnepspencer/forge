#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAppearanceRoleAttachmentDenial {
    MissingComponentReference,
    UnknownRole,
    StaleRoleRevision,
    AspectContractMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAppearanceRoleAttachment {
    target: crate::capability::ComponentId,
    role: worth_ui_dsl::UiAppearanceRoleIdentity,
    revision: worth_ui_dsl::UiAppearanceRoleRevision,
    aspect_contract: worth_ui_dsl::UiAppearanceAspectContract,
}

impl UiAppearanceRoleAttachment {
    pub(crate) fn admit(
        declaration: &worth_ui_dsl::UiAppearanceRoleAttachmentDeclaration,
        target: Option<&crate::capability::ComponentId>,
        snapshot: &crate::capability::CapabilitySnapshot,
    ) -> Result<Self, UiAppearanceRoleAttachmentDenial> {
        let target = target
            .ok_or(UiAppearanceRoleAttachmentDenial::MissingComponentReference)?
            .clone();
        let component = snapshot
            .components()
            .get(&target)
            .expect("component reference was admitted against this frozen snapshot");
        let role = snapshot
            .appearance_roles()
            .get(declaration.role())
            .ok_or(UiAppearanceRoleAttachmentDenial::UnknownRole)?;
        if role.revision() != declaration.revision() {
            return Err(UiAppearanceRoleAttachmentDenial::StaleRoleRevision);
        }
        let target_contract = component
            .appearance_aspect_contract()
            .ok_or(UiAppearanceRoleAttachmentDenial::AspectContractMismatch)?;
        if target_contract != role.aspect_contract() {
            return Err(UiAppearanceRoleAttachmentDenial::AspectContractMismatch);
        }
        Ok(Self {
            target,
            role: role.role().clone(),
            revision: role.revision(),
            aspect_contract: target_contract.clone(),
        })
    }

    pub(crate) const fn target(&self) -> &crate::capability::ComponentId {
        &self.target
    }

    pub(crate) const fn role(&self) -> &worth_ui_dsl::UiAppearanceRoleIdentity {
        &self.role
    }

    pub(crate) const fn revision(&self) -> worth_ui_dsl::UiAppearanceRoleRevision {
        self.revision
    }

    pub(crate) const fn aspect_contract(&self) -> &worth_ui_dsl::UiAppearanceAspectContract {
        &self.aspect_contract
    }
}
