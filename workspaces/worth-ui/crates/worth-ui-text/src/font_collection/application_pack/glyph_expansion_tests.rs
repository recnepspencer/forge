use super::{append_operations, derive_from_gsub, simulate_expansion, LookupPlan};
use crate::font_collection::UiFontCollectionAdmissionDenial;

#[test]
fn multiple_substitution_expansion_is_derived_from_real_coverage_and_outputs() {
    let bytes = single_multiple_gsub(7);
    assert_eq!(derive_from_gsub(&bytes).unwrap(), 7);
}

#[test]
fn symbolic_expansion_follows_outputs_and_rejects_recursive_context_calls() {
    let plans = [
        LookupPlan {
            replacements: [(1, vec![2, 2].into_boxed_slice())].into_iter().collect(),
            dependencies: Vec::new(),
        },
        LookupPlan {
            replacements: [(2, vec![3, 3, 3].into_boxed_slice())]
                .into_iter()
                .collect(),
            dependencies: Vec::new(),
        },
    ];
    assert_eq!(simulate_expansion(&plans, &[0, 1]).unwrap(), 6);

    let cyclic = [LookupPlan {
        replacements: Default::default(),
        dependencies: vec![0],
    }];
    assert_eq!(
        append_operations(0, &cyclic, &mut [false], &mut Vec::new()).unwrap_err(),
        UiFontCollectionAdmissionDenial::UnboundedGlyphExpansion
    );
}

#[test]
fn every_repository_font_has_a_finite_usable_derived_expansion_bound() {
    let observed = super::super::super::profile_inputs_from_repository()
        .into_vec()
        .into_iter()
        .map(|input| {
            let font = harfrust::FontRef::from_index(&input.bytes, 0).unwrap();
            (input.id, super::derive(&font).unwrap())
        })
        .collect::<Vec<_>>();
    assert_eq!(observed.len(), 30);
    println!("derived GSUB expansion bounds: {observed:?}");
    assert!(observed.iter().all(|(_, bound)| (1..=64).contains(bound)));
}

fn single_multiple_gsub(output_count: u16) -> Vec<u8> {
    let sequence_end = 32 + usize::from(output_count) * 2;
    let coverage = sequence_end;
    let feature_list = coverage + 6;
    let script_list = feature_list + 14;
    let mut bytes = vec![0u8; script_list + 20];
    put(&mut bytes, 0, 1);
    put(&mut bytes, 4, script_list as u16);
    put(&mut bytes, 6, feature_list as u16);
    put(&mut bytes, 8, 10);
    put(&mut bytes, 10, 1);
    put(&mut bytes, 12, 4);
    put(&mut bytes, 14, 2);
    put(&mut bytes, 18, 1);
    put(&mut bytes, 20, 8);
    put(&mut bytes, 22, 1);
    put(&mut bytes, 24, (coverage - 22) as u16);
    put(&mut bytes, 26, 1);
    put(&mut bytes, 28, 8);
    put(&mut bytes, 30, output_count);
    for index in 0..output_count {
        put(&mut bytes, 32 + usize::from(index) * 2, index + 1);
    }
    put(&mut bytes, coverage, 1);
    put(&mut bytes, coverage + 2, 1);
    put(&mut bytes, coverage + 4, 1);
    put(&mut bytes, feature_list, 1);
    bytes[feature_list + 2..feature_list + 6].copy_from_slice(b"test");
    put(&mut bytes, feature_list + 6, 8);
    put(&mut bytes, feature_list + 10, 1);
    put(&mut bytes, script_list, 1);
    bytes[script_list + 2..script_list + 6].copy_from_slice(b"DFLT");
    put(&mut bytes, script_list + 6, 8);
    put(&mut bytes, script_list + 8, 4);
    put(&mut bytes, script_list + 14, u16::MAX);
    put(&mut bytes, script_list + 16, 1);
    bytes
}

fn put(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
}
