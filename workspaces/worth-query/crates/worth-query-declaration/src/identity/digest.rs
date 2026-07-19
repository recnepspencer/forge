use super::digest_hash::digest_hash_parts;

macro_rules! declaration_digest {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn from_parts(parts: &[String]) -> Self {
                Self(digest_hash_parts(parts))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

declaration_digest!(CanonicalQueryDigest);
declaration_digest!(CanonicalResultShapeDigest);
declaration_digest!(SchemaBasisDigest);
declaration_digest!(ValidatedQueryDigest);
declaration_digest!(ValidatedResultShapeDigest);
declaration_digest!(CollectionPlanDigest);
declaration_digest!(BindingFulfillmentDigest);

pub use super::digest_hash::hash_parts;
