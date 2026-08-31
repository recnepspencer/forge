#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenAppearanceRoleCapabilities {
    roles: Vec<worth_ui_dsl::UiAppearanceRoleDeclaration>,
}

impl FrozenAppearanceRoleCapabilities {
    #[cfg(test)]
    pub(crate) const fn empty() -> Self {
        Self { roles: Vec::new() }
    }

    pub(crate) fn from_accepted(
        mut roles: Vec<worth_ui_dsl::UiAppearanceRoleDeclaration>,
        accepted: &super::AppearanceRoleAcceptedRegistrationProof,
    ) -> Self {
        roles.retain(|role| accepted.admits(role));
        roles.sort_by(|left, right| left.role().cmp(right.role()));
        Self { roles }
    }

    pub fn len(&self) -> usize {
        self.roles.len()
    }

    pub fn get(
        &self,
        identity: &worth_ui_dsl::UiAppearanceRoleIdentity,
    ) -> Option<&worth_ui_dsl::UiAppearanceRoleDeclaration> {
        self.roles
            .binary_search_by(|role| role.role().cmp(identity))
            .ok()
            .map(|index| &self.roles[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.roles
            .iter()
            .fold(0x6170_7065_6172_616e, |mut digest, role| {
                for byte in role.role().as_str().as_bytes() {
                    digest = super::semantic_digest::fold_semantic_digest(digest, u64::from(*byte));
                }
                digest = super::semantic_digest::fold_semantic_digest(
                    digest,
                    u64::from(role.schema().revision()),
                );
                digest =
                    super::semantic_digest::fold_semantic_digest(digest, role.revision().value());
                digest = super::semantic_digest::fold_semantic_digest(
                    digest,
                    role.aspect_contract().applicability() as u64 + 1,
                );
                digest =
                    super::semantic_digest::fold_semantic_digest(digest, 0x7265_7175_6972_6564);
                digest = super::semantic_digest::fold_semantic_digest(
                    digest,
                    role.aspect_contract().required().len() as u64,
                );
                for aspect in role.aspect_contract().required() {
                    digest =
                        super::semantic_digest::fold_semantic_digest(digest, *aspect as u64 + 1);
                }
                digest =
                    super::semantic_digest::fold_semantic_digest(digest, 0x6f70_7469_6f6e_616c);
                digest = super::semantic_digest::fold_semantic_digest(
                    digest,
                    role.aspect_contract().optional().len() as u64,
                );
                for aspect in role.aspect_contract().optional() {
                    digest =
                        super::semantic_digest::fold_semantic_digest(digest, *aspect as u64 + 1);
                }
                for (aspect, partition) in role.partitions() {
                    digest =
                        super::semantic_digest::fold_semantic_digest(digest, *aspect as u64 + 1);
                    for axis in partition.axes() {
                        digest = super::semantic_digest::fold_semantic_digest(
                            digest,
                            axis.axis() as u64 + 1,
                        );
                        digest = super::semantic_digest::fold_semantic_digest(
                            digest,
                            u64::from(axis.revision()),
                        );
                    }
                    for cell in partition.cells() {
                        digest = super::semantic_digest::fold_semantic_digest(
                            digest,
                            cell.result().value_kind() as u64 + 1,
                        );
                        for byte in cell.result().slot().as_str().as_bytes() {
                            digest = super::semantic_digest::fold_semantic_digest(
                                digest,
                                u64::from(*byte),
                            );
                        }
                    }
                }
                digest
            })
    }
}
