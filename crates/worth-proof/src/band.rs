//! Compile-time package-band recognition for macro expansion fences.

/// Returns whether `package_name` begins with one of the supplied legal prefixes.
///
/// This symbol is public only because exported macros must be able to reach it
/// through `$crate`. Use [`crate::band_guard!`] rather than calling it directly.
#[doc(hidden)]
pub const fn __band_guard_package_matches_any_prefix(
    package_name: &str,
    legal_prefixes: &[&str],
) -> bool {
    let mut prefix_index = 0;
    while prefix_index < legal_prefixes.len() {
        if package_name_starts_with(package_name, legal_prefixes[prefix_index]) {
            return true;
        }
        prefix_index += 1;
    }
    false
}

const fn package_name_starts_with(package_name: &str, legal_prefix: &str) -> bool {
    let package_bytes = package_name.as_bytes();
    let prefix_bytes = legal_prefix.as_bytes();
    if prefix_bytes.is_empty() || prefix_bytes.len() > package_bytes.len() {
        return false;
    }

    let mut byte_index = 0;
    while byte_index < prefix_bytes.len() {
        if package_bytes[byte_index] != prefix_bytes[byte_index] {
            return false;
        }
        byte_index += 1;
    }
    true
}

/// Rejects expansion unless the expanding Cargo package has a legal prefix.
///
/// The prefixes belong to the macro surface invoking this mechanism; they are
/// deliberately not encoded in `worth-proof`. The assertion is evaluated as a
/// `const` over `env!("CARGO_PKG_NAME")` in the expanding crate and therefore
/// leaves no runtime value, branch, or allocation.
///
/// # Adoption law
///
/// Every public macro on a Query audience facade must embed a band guard for
/// its audience. Declaration macros that lower into Query handles are the first
/// mandatory adopters in Milestone 3.
///
/// ```
/// worth_proof::band_guard!("worth-", "worthy-");
/// ```
#[macro_export]
macro_rules! band_guard {
    ($first_legal_prefix:literal $(, $additional_legal_prefix:literal)* $(,)?) => {
        const _: () = {
            const EXPANDING_PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
            assert!(
                $crate::__band_guard_package_matches_any_prefix(
                    EXPANDING_PACKAGE_NAME,
                    &[$first_legal_prefix $(, $additional_legal_prefix)*],
                ),
                concat!(
                    "worth_proof::band_guard! rejected package `",
                    env!("CARGO_PKG_NAME"),
                    "`; legal package prefixes: ",
                    $first_legal_prefix,
                    $(", ", $additional_legal_prefix,)*
                    "; see cad/docs/worthy-foundations/BOUNDARIES.md",
                ),
            );
        };
    };
}

#[cfg(test)]
mod tests {
    use super::__band_guard_package_matches_any_prefix;

    const PLATFORM_ENTRY_MATCHES: bool = __band_guard_package_matches_any_prefix(
        "worth-entry-construct",
        &["worth-entry-", "worthy-entry-"],
    );
    const PRODUCT_ENTRY_MATCHES: bool = __band_guard_package_matches_any_prefix(
        "worthy-entry-construct",
        &["worth-entry-", "worthy-entry-"],
    );
    const NEAR_PREFIX_DOES_NOT_MATCH: bool = __band_guard_package_matches_any_prefix(
        "worth-entryless-construct",
        &["worth-entry-", "worthy-entry-"],
    );

    #[test]
    fn package_prefix_recognition_is_const_and_exact() {
        // Const blocks, so a regression here is a compilation failure rather
        // than a test that has to be run to find out.
        const { assert!(PLATFORM_ENTRY_MATCHES) };
        const { assert!(PRODUCT_ENTRY_MATCHES) };
        const { assert!(!NEAR_PREFIX_DOES_NOT_MATCH) };
        assert!(__band_guard_package_matches_any_prefix(
            "worth-entry-",
            &["worth-entry-"]
        ));
        assert!(!__band_guard_package_matches_any_prefix(
            "worth-entry-construct",
            &[]
        ));
        assert!(!__band_guard_package_matches_any_prefix(
            "any-package",
            &[""]
        ));
    }
}
