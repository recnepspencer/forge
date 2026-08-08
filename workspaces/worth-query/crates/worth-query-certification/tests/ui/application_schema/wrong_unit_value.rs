use std::marker::PhantomData;

use worth_foundational::facade::{AspectValue, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    ApplicationUnitMarker, ApplicationFieldRef, DeclaredApplicationUnit,
    EqualityPredicate, ReadOnly, TypedApplicationValue, TypedUnitApplicationValue,
};

struct Schema;
struct Entity;
struct Aspect;
struct Balance;
struct UsdCurrency;
struct USD;
struct EUR;
struct Money<C>(PhantomData<C>);

impl ApplicationUnitMarker<USD> for UsdCurrency {
    const NAME: &'static str = "UsdCurrency";
}

impl<C> TypedApplicationValue for Money<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(0)
    }
}

impl<C: 'static> TypedUnitApplicationValue for Money<C> {
    type Unit = C;
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
        DeclaredApplicationUnit<
            UsdCurrency,
            <Money<EUR> as TypedUnitApplicationValue>::Unit,
        >,
    > = ApplicationFieldRef::from_schema_identifiers("Entity", "Aspect", "Balance");
}
