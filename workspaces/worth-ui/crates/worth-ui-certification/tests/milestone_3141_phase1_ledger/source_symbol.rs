use std::path::Path;

use syn::visit::{self, Visit};

pub(super) fn validate(source: &Path, symbol: &str) -> Result<(), String> {
    syn::parse_str::<syn::Ident>(symbol).map_err(|_| format!("invalid Rust symbol {symbol}"))?;
    let source_text = std::fs::read_to_string(source)
        .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
    let syntax = syn::parse_file(&source_text)
        .map_err(|error| format!("cannot parse {}: {error}", source.display()))?;
    let mut visitor = NamedSymbol { symbol, matches: 0 };
    visitor.visit_file(&syntax);
    (visitor.matches == 1).then_some(()).ok_or_else(|| {
        format!(
            "expected one function named {symbol} in {}, found {}",
            source.display(),
            visitor.matches
        )
    })
}

struct NamedSymbol<'a> {
    symbol: &'a str,
    matches: usize,
}

impl<'ast> Visit<'ast> for NamedSymbol<'_> {
    fn visit_signature(&mut self, signature: &'ast syn::Signature) {
        if signature.ident == self.symbol {
            self.matches += 1;
        }
        visit::visit_signature(self, signature);
    }
}
