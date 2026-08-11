//! Trait impls for values that hold nothing.
//!
//! `#[derive]` places its bounds on the *type parameters*, so deriving
//! `PartialEq` on a zero-sized proof over `<P, A>` demands `P: PartialEq` and
//! `A: PartialEq`. Those parameters are markers — unit structs that exist to
//! be named — and requiring derives on them is how a consumer ends up writing
//! `#[derive(Debug, PartialEq, Eq)]` on a marker for reasons nobody can
//! explain, or hitting an error the moment they compare two proofs. This
//! crate's own tests did the former.
//!
//! A value with no fields carries no information, so two of them are always
//! equal and cloning one copies nothing. These impls say that directly, and
//! the bounds stay where they belong: on the marker traits the type already
//! requires.

/// Implement `Debug`, `PartialEq`, and `Eq` for a type whose every field is
/// `PhantomData`. Prefix with `copy` to add `Clone` and `Copy`.
///
/// Bounds come from the type's own marker traits, never from the derived
/// traits. Do **not** use this for a type carrying a value, do not add `copy`
/// to anything whose duplication should be refused, and never add `Default` —
/// a zero-sized proof with a `Default` impl is a public constructor by another
/// name.
///
/// ```text
/// type_level_traits!(copy Proof<P, A>);
/// type_level_traits!(AuthorityWitness<A: AuthorityMarker>);
/// ```
macro_rules! type_level_traits {
    (copy $name:ident < $( $param:ident $(: $bound:path)? ),+ $(,)? >) => {
        $crate::type_level_traits!($name < $( $param $(: $bound)? ),+ >);

        impl<$($param),+> ::core::clone::Clone for $name<$($param),+>
        where
            $( $($param: $bound,)? )+
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<$($param),+> ::core::marker::Copy for $name<$($param),+>
        where
            $( $($param: $bound,)? )+
        {
        }
    };
    ($name:ident < $( $param:ident $(: $bound:path)? ),+ $(,)? >) => {
        impl<$($param),+> ::core::fmt::Debug for $name<$($param),+>
        where
            $( $($param: $bound,)? )+
        {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                formatter.write_str(::core::stringify!($name))
            }
        }

        impl<$($param),+> ::core::cmp::PartialEq for $name<$($param),+>
        where
            $( $($param: $bound,)? )+
        {
            /// Always `true`: there is nothing to differ.
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }

        impl<$($param),+> ::core::cmp::Eq for $name<$($param),+>
        where
            $( $($param: $bound,)? )+
        {
        }
    };
}

pub(crate) use type_level_traits;
