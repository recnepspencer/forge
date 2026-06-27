use std::collections::BTreeSet;

use super::declared_input_role::ReplayUndoDeclaredInputRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoDeclaredInputRoleSet {
    roles: BTreeSet<ReplayUndoDeclaredInputRole>,
}

impl ReplayUndoDeclaredInputRoleSet {
    pub(crate) fn new(roles: &[ReplayUndoDeclaredInputRole]) -> Self {
        Self {
            roles: roles.iter().copied().collect(),
        }
    }

    pub fn contains(&self, role: ReplayUndoDeclaredInputRole) -> bool {
        self.roles.contains(&role)
    }

    pub fn iter(&self) -> impl Iterator<Item = ReplayUndoDeclaredInputRole> + '_ {
        self.roles.iter().copied()
    }
}
