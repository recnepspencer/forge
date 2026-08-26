use super::WorthQueryPortableTypeIdentity;

/// A Rust type whose package-relevant meaning has an explicit stable identity.
pub trait WorthQueryPortableType {
    const PORTABLE_TYPE_IDENTITY: WorthQueryPortableTypeIdentity;
}

macro_rules! primitive_portable_types {
    ($($ty:ty => $identity:literal),+ $(,)?) => {
        $(
            impl WorthQueryPortableType for $ty {
                const PORTABLE_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
                    WorthQueryPortableTypeIdentity::declared($identity);
            }
        )+
    };
}

primitive_portable_types!(
    () => "worth.rust.unit",
    bool => "worth.rust.bool",
    i8 => "worth.rust.i8",
    i16 => "worth.rust.i16",
    i32 => "worth.rust.i32",
    i64 => "worth.rust.i64",
    i128 => "worth.rust.i128",
    u8 => "worth.rust.u8",
    u16 => "worth.rust.u16",
    u32 => "worth.rust.u32",
    u64 => "worth.rust.u64",
    u128 => "worth.rust.u128",
    f32 => "worth.rust.f32",
    f64 => "worth.rust.f64",
    char => "worth.rust.char",
    String => "worth.rust.string",
);

impl WorthQueryPortableType for worth_foundational::facade::InternedString {
    const PORTABLE_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
        WorthQueryPortableTypeIdentity::declared("worth.foundational.interned_string.v1");
}
