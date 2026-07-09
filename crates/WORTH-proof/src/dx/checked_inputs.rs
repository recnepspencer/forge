use core::convert::Infallible;

use crate::transition::{PreConstructionGate, TransitionReadiness};

pub fn gate_ready<C>(context: C) -> PreConstructionGate<C, Infallible, Infallible> {
    PreConstructionGate::ready(context)
}

pub fn ready_now<C>(
    context: C,
) -> TransitionReadiness<C, Infallible, Infallible, Infallible, Infallible, Infallible> {
    TransitionReadiness::ready(context)
}

#[cfg(test)]
mod tests {
    use crate::transition::{PreConstructionGate, TransitionReadiness};

    use super::{gate_ready, ready_now};

    #[test]
    fn checked_ready_helpers_match_raw_ready_constructors() {
        let gate = gate_ready(7_u8);
        let readiness = ready_now(11_u8);

        assert_eq!(
            gate,
            PreConstructionGate::<u8, core::convert::Infallible, core::convert::Infallible>::ready(
                7_u8
            )
        );
        assert_eq!(
            readiness,
            TransitionReadiness::<
                u8,
                core::convert::Infallible,
                core::convert::Infallible,
                core::convert::Infallible,
                core::convert::Infallible,
                core::convert::Infallible,
            >::ready(11_u8)
        );
    }
}
