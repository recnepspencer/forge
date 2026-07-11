macro_rules! define_owner_outcome {
    (
        $visibility:vis $outcome:ident,
        $view_visibility:vis $view:ident,
        $inner:ident,
        $machine:ident,
        $operation:ident,
        [
            $(
                $constructor:ident => $variant:ident($payload:ty):
                $from:ident => $transition:ident => $to:ident
            ),+ $(,)?
        ]
    ) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $inner { $($variant($payload)),+ }

        #[derive(Debug, PartialEq, Eq)]
        $visibility struct $outcome {
            issued: crate::production_transition::S8OwnerIssuedCase<$inner>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $view_visibility enum $view<'a> { $($variant(&'a $payload)),+ }

        impl $outcome {
            $(pub(crate) fn $constructor(value: $payload) -> Self {
                Self::from_owner_payload($inner::$variant(value))
            })+

            fn from_owner_payload(payload: $inner) -> Self {
                let transition = Self::transition_for_payload(&payload);
                Self {
                    issued: crate::production_transition::S8OwnerIssuedCase::issue(
                        payload,
                        transition,
                    ),
                }
            }

            pub fn view(&self) -> $view<'_> {
                match self.issued.payload() { $($inner::$variant(value) => $view::$variant(value)),+ }
            }

            fn into_owner_payload(self) -> $inner { self.issued.into_payload() }

            pub const fn production_transition(
                &self,
            ) -> crate::production_transition::S8LayoutProductionTransition {
                self.issued.transition()
            }

            const fn transition_for_payload(
                payload: &$inner,
            ) -> crate::production_transition::S8LayoutProductionTransition {
                match payload {
                    $($inner::$variant(_) => crate::production_transition::owner_transition(
                        crate::production_transition::S8LayoutStateMachine::$machine,
                        crate::production_transition::S8LayoutProductionOperation::$operation,
                        stringify!($variant),
                        crate::production_transition::S8LayoutMachineState::$from,
                        crate::production_transition::S8LayoutMachineTransition::$transition,
                        crate::production_transition::S8LayoutMachineState::$to,
                    )),+
                }
            }

            pub(crate) fn owner_transition_contract(
            ) -> crate::production_transition::S8OwnerTransitionContract {
                static FACTS: std::sync::OnceLock<Box<[
                    crate::production_transition::S8LayoutProductionTransition
                ]>> = std::sync::OnceLock::new();
                let facts = FACTS.get_or_init(|| [$(crate::production_transition::owner_transition(
                    crate::production_transition::S8LayoutStateMachine::$machine,
                    crate::production_transition::S8LayoutProductionOperation::$operation,
                    stringify!($variant),
                    crate::production_transition::S8LayoutMachineState::$from,
                    crate::production_transition::S8LayoutMachineTransition::$transition,
                    crate::production_transition::S8LayoutMachineState::$to,
                )),+].into());
                crate::production_transition::S8OwnerTransitionContract::from_owner_outcomes(
                    crate::production_transition::S8LayoutStateMachine::$machine,
                    crate::production_transition::S8LayoutProductionOperation::$operation,
                    facts,
                )
            }
        }
    };
}

pub(crate) use define_owner_outcome;
