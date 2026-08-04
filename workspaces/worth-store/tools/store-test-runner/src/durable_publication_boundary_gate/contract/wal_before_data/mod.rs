mod barrier;
mod constructor_locations;
mod data;

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let start = source.find(signature)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn function_signature<'a>(source: &'a str, start: &str) -> Option<&'a str> {
    let (_, tail) = source.split_once(start)?;
    tail.split_once('{').map(|(signature, _)| signature)
}

fn contains_in_order(source: &str, required: &[&str]) -> bool {
    let mut offset = 0;
    required.iter().all(|needle| {
        let Some(found) = source[offset..].find(needle) else {
            return false;
        };
        offset += found + needle.len();
        true
    })
}

#[test]
fn signature_extraction_is_anchored_to_a_declaration_not_an_earlier_call() {
    let source = "owner.dispatch_wal_durable_data(value);\n\
                  pub fn dispatch_wal_durable_data(argument: ExactAuthority) -> Outcome { todo!() }";
    let signature = function_signature(source, "pub fn dispatch_wal_durable_data(")
        .expect("find declared signature");
    assert!(signature.contains("argument: ExactAuthority"));
}
