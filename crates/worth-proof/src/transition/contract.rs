pub trait Transition<Input> {
    type Output;

    fn transition(&self, input: Input) -> Self::Output;
}

pub trait ContextualTransition<Input, Context> {
    type Output;

    fn transition(&self, input: Input, context: Context) -> Self::Output;
}

pub fn apply_transition<T, Input>(transition: &T, input: Input) -> T::Output
where
    T: Transition<Input>,
{
    transition.transition(input)
}

pub fn apply_contextual_transition<T, Input, Context>(
    transition: &T,
    input: Input,
    context: Context,
) -> T::Output
where
    T: ContextualTransition<Input, Context>,
{
    transition.transition(input, context)
}

#[cfg(test)]
mod tests {
    use super::{apply_contextual_transition, apply_transition, ContextualTransition, Transition};

    struct IncrementTransition;

    impl Transition<u64> for IncrementTransition {
        type Output = u64;

        fn transition(&self, input: u64) -> Self::Output {
            input + 1
        }
    }

    struct MultiplyWithContext;

    impl ContextualTransition<u64, u64> for MultiplyWithContext {
        type Output = u64;

        fn transition(&self, input: u64, context: u64) -> Self::Output {
            input * context
        }
    }

    #[test]
    fn transition_contract_supports_static_progression_without_runtime_registry() {
        assert_eq!(apply_transition(&IncrementTransition, 7), 8);
    }

    #[test]
    fn contextual_transition_contract_keeps_context_explicit() {
        assert_eq!(apply_contextual_transition(&MultiplyWithContext, 7, 3), 21);
    }
}
