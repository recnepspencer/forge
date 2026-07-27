use std::marker::PhantomData;

use worth_foundational::facade::{AspectValue, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    ApplicationCurrencyMarker, ApplicationFieldRef, DeclaredApplicationCurrency,
    EqualityPredicate, ReadOnly, TypedApplicationValue, TypedCurrencyApplicationValue,
};

struct Schema;
struct Entity;
struct Aspect;
struct Balance;
struct UsdCurrency;
struct USD;
struct EUR;
struct Money<C>(PhantomData<C>);

impl ApplicationCurrencyMarker<USD> for UsdCurrency {
    const NAME: &'static str = "UsdCurrency";
}

impl<C> TypedApplicationValue for Money<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(0)
    }
}

impl<C: 'static> TypedCurrencyApplicationValue for Money<C> {
    type Currency = C;
}

fn main() {
    let _: ApplicationFieldRef<
        Schema,
        Entity,
        Aspect,
        Balance,
        Money<EUR>,
        ReadOnly,
        EqualityPredicate,
        DeclaredApplicationCurrency<
            UsdCurrency,
            <Money<EUR> as TypedCurrencyApplicationValue>::Currency,
        >,
    > = ApplicationFieldRef::from_schema_identifiers("Entity", "Aspect", "Balance");
}
