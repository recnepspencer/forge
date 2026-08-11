//! Composition values admitted through their public constructors and checks.

pub(crate) fn family_member() -> worth_proof::AuthoritativeFamilyMember<u8> {
    worth_proof::AuthoritativeFamilyMember::new(1)
}

pub(crate) fn family_symbol() -> worth_proof::CompositionFamilySymbol<u8> {
    worth_proof::CompositionFamilySymbol::new(2)
}

pub(crate) fn family_reference() -> worth_proof::FamilyResolvedReference<u8, u8> {
    worth_proof::resolve_family_symbol(family_symbol(), family_member())
}

pub(crate) fn family_lifecycle_action() -> worth_proof::FamilyLifecycleAction<u8, u8, u8> {
    worth_proof::FamilyLifecycleAction::Create {
        symbol: family_symbol(),
        payload: 3,
    }
}

pub(crate) fn lowered_family() -> worth_proof::LoweredFamilyProgram2<u8, u8, u8> {
    let actions = worth_proof::Pair::new(
        worth_proof::FamilyLifecycleAction::Create {
            symbol: worth_proof::CompositionFamilySymbol::new(2),
            payload: 3,
        },
        worth_proof::FamilyLifecycleAction::Retire {
            target: worth_proof::AuthoritativeFamilyMember::new(1),
        },
    );
    worth_proof::lower_deterministic_family_pair(actions, |action| match action {
        worth_proof::FamilyLifecycleAction::Create { .. } => 1_u8,
        worth_proof::FamilyLifecycleAction::Retire { .. } => 0_u8,
        _ => 2_u8,
    })
}

pub(crate) fn fork_outputs() -> worth_proof::ForkOutputs2<u8, u16> {
    worth_proof::ForkOutputs2::new(1, 2)
}

pub(crate) fn join_inputs() -> worth_proof::JoinInputs2<u8, u16> {
    worth_proof::JoinInputs2::new(1, 2)
}
