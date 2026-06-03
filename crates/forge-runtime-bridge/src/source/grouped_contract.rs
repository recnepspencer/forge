use forge_foundational::facade::{AspectKey, AspectValue};

use crate::snapshot::TruthSnapshotIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBindingProof {
    aspect_key: AspectKey,
}

impl IdentityBindingProof {
    pub(crate) fn new(aspect_key: AspectKey) -> Self {
        Self { aspect_key }
    }

    pub fn aspect_key(&self) -> &str {
        self.aspect_key.as_str()
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupingBindingProof {
    aspect_key: AspectKey,
}

impl GroupingBindingProof {
    pub(crate) fn new(aspect_key: AspectKey) -> Self {
        Self { aspect_key }
    }

    pub fn aspect_key(&self) -> &str {
        self.aspect_key.as_str()
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedProjectionContract {
    grouping_aspect: AspectKey,
    identity_binding: IdentityBindingProof,
    grouping_binding: GroupingBindingProof,
}

impl GroupedProjectionContract {
    pub(crate) fn from_source(source: &impl GroupedProjectionSource) -> Self {
        Self {
            grouping_aspect: source.grouping_aspect_key().clone(),
            identity_binding: IdentityBindingProof::new(
                source.identity_binding_aspect_key().clone(),
            ),
            grouping_binding: GroupingBindingProof::new(
                source.grouping_binding_aspect_key().clone(),
            ),
        }
    }

    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_str()
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn identity_binding(&self) -> &IdentityBindingProof {
        &self.identity_binding
    }

    pub fn grouping_binding(&self) -> &GroupingBindingProof {
        &self.grouping_binding
    }
}

pub trait GroupedProjectionMemberSource {
    fn row_identity(&self) -> &str;
    fn identity_value(&self) -> &AspectValue;
    fn grouping_value(&self) -> &AspectValue;
}

pub trait GroupedProjectionSource {
    type Member: GroupedProjectionMemberSource;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity;
    fn grouping_aspect_key(&self) -> &AspectKey;
    fn identity_binding_aspect_key(&self) -> &AspectKey;
    fn grouping_binding_aspect_key(&self) -> &AspectKey;
    fn members(&self) -> &[Self::Member];
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{AspectKey, AspectValue};

    use super::{
        GroupedProjectionContract, GroupedProjectionMemberSource, GroupedProjectionSource,
    };
    use crate::snapshot::TruthSnapshotIdentity;

    #[derive(Clone)]
    struct EmptyMember;

    impl GroupedProjectionMemberSource for EmptyMember {
        fn row_identity(&self) -> &str {
            "row"
        }

        fn identity_value(&self) -> &AspectValue {
            static VALUE: AspectValue = AspectValue::Null;
            &VALUE
        }

        fn grouping_value(&self) -> &AspectValue {
            static VALUE: AspectValue = AspectValue::Null;
            &VALUE
        }
    }

    struct ContractSource {
        grouping_aspect: AspectKey,
        identity_binding: AspectKey,
        grouping_binding: AspectKey,
        members: Vec<EmptyMember>,
    }

    impl GroupedProjectionSource for ContractSource {
        type Member = EmptyMember;

        fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity {
            static SNAPSHOT: std::sync::LazyLock<TruthSnapshotIdentity> =
                std::sync::LazyLock::new(|| TruthSnapshotIdentity::new("snapshot-a"));
            &SNAPSHOT
        }

        fn grouping_aspect_key(&self) -> &AspectKey {
            &self.grouping_aspect
        }

        fn identity_binding_aspect_key(&self) -> &AspectKey {
            &self.identity_binding
        }

        fn grouping_binding_aspect_key(&self) -> &AspectKey {
            &self.grouping_binding
        }

        fn members(&self) -> &[Self::Member] {
            &self.members
        }
    }

    #[test]
    fn grouped_projection_contract_preserves_native_aspect_keys() {
        let contract = GroupedProjectionContract::from_source(&ContractSource {
            grouping_aspect: aspect_key("status"),
            identity_binding: aspect_key("identity.id"),
            grouping_binding: aspect_key("status.lane"),
            members: Vec::new(),
        });

        assert_eq!(contract.grouping_aspect(), "status");
        assert_eq!(contract.native_grouping_aspect_key().as_str(), "status");
        assert_eq!(contract.identity_binding().aspect_key(), "identity.id");
        assert_eq!(contract.grouping_binding().aspect_key(), "status.lane");
        assert_eq!(
            contract.identity_binding().native_aspect_key().as_str(),
            "identity.id"
        );
        assert_eq!(
            contract.grouping_binding().native_aspect_key().as_str(),
            "status.lane"
        );
    }

    #[test]
    fn grouped_projection_source_cannot_supply_unvalidated_aspect_key_text() {
        let source = ContractSource {
            grouping_aspect: aspect_key("status"),
            identity_binding: aspect_key("identity.id"),
            grouping_binding: aspect_key("status.lane"),
            members: Vec::new(),
        };

        assert_eq!(source.grouping_aspect_key().as_str(), "status");
        assert_eq!(source.identity_binding_aspect_key().as_str(), "identity.id");
        assert_eq!(source.grouping_binding_aspect_key().as_str(), "status.lane");
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("test aspect key must be foundational")
    }
}
