use crate::collections::{NonEmpty, Pair};
use crate::composition::{AuthoritativeFamilyMember, CompositionFamilySymbol};
use crate::recipe::{Recipe, Unresolved};

pub fn recipe<T>(payload: T) -> Recipe<Unresolved, T> {
    Recipe::new(payload)
}

pub fn pair<T>(left: T, right: T) -> Pair<T> {
    Pair::new(left, right)
}

pub fn non_empty<T>(head: T, tail: Vec<T>) -> NonEmpty<T> {
    NonEmpty::new(head, tail)
}

pub fn sym<S>(symbol: S) -> CompositionFamilySymbol<S> {
    CompositionFamilySymbol::new(symbol)
}

pub fn member<A>(member: A) -> AuthoritativeFamilyMember<A> {
    AuthoritativeFamilyMember::new(member)
}

#[cfg(test)]
mod tests {
    use crate::collections::{NonEmpty, Pair};
    use crate::composition::{AuthoritativeFamilyMember, CompositionFamilySymbol};
    use crate::recipe::{Recipe, Unresolved};

    use super::{member, non_empty, pair, recipe, sym};

    #[test]
    fn helper_recipe_matches_raw_constructor() {
        let pleasant = recipe("payload");
        let raw = Recipe::<Unresolved, _>::new("payload");

        assert_eq!(pleasant, raw);
    }

    #[test]
    fn helper_fixed_shape_constructors_match_raw_constructors() {
        let pleasant_pair = pair(1_u8, 2_u8);
        let raw_pair = Pair::new(1_u8, 2_u8);
        let pleasant_non_empty = non_empty(1_u8, vec![2_u8, 3_u8]);
        let raw_non_empty = NonEmpty::new(1_u8, vec![2_u8, 3_u8]);

        assert_eq!(pleasant_pair, raw_pair);
        assert_eq!(pleasant_non_empty, raw_non_empty);
    }

    #[test]
    fn helper_family_identity_constructors_match_raw_constructors() {
        let pleasant_symbol = sym(7_u8);
        let raw_symbol = CompositionFamilySymbol::new(7_u8);
        let pleasant_member = member(11_u16);
        let raw_member = AuthoritativeFamilyMember::new(11_u16);

        assert_eq!(pleasant_symbol, raw_symbol);
        assert_eq!(pleasant_member, raw_member);
    }
}
