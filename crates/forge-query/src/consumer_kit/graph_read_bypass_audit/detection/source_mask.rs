pub(in crate::consumer_kit::graph_read_bypass_audit) fn mask_comments_and_string_literals(
    source: &str,
) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut state = SourceMaskState::Code;
    while let Some(ch) = chars.next() {
        state = state.mask(ch, &mut chars, &mut output);
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SourceMaskState {
    Code,
    LineComment,
    BlockComment,
    String,
    Char,
}

impl SourceMaskState {
    fn mask(
        self,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
        output: &mut String,
    ) -> Self {
        match self {
            Self::Code => mask_code(ch, chars, output),
            Self::LineComment => mask_line_comment(ch, output),
            Self::BlockComment => mask_block_comment(ch, chars, output),
            Self::String => mask_quoted(ch, chars, output, '"', Self::String),
            Self::Char => mask_quoted(ch, chars, output, '\'', Self::Char),
        }
    }
}

fn mask_code(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceMaskState {
    if ch == '/' && chars.peek() == Some(&'/') {
        chars.next();
        output.push(' ');
        return SourceMaskState::LineComment;
    }
    if ch == '/' && chars.peek() == Some(&'*') {
        chars.next();
        output.push(' ');
        return SourceMaskState::BlockComment;
    }
    if ch == '"' {
        output.push(' ');
        return SourceMaskState::String;
    }
    if ch == '\'' {
        output.push(' ');
        return SourceMaskState::Char;
    }
    output.push(ch);
    SourceMaskState::Code
}

fn mask_line_comment(ch: char, output: &mut String) -> SourceMaskState {
    if ch == '\n' {
        output.push('\n');
        SourceMaskState::Code
    } else {
        output.push(' ');
        SourceMaskState::LineComment
    }
}

fn mask_block_comment(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceMaskState {
    let ends = ch == '*' && chars.peek() == Some(&'/');
    if ends {
        chars.next();
    }
    output.push(if ch == '\n' { '\n' } else { ' ' });
    if ends {
        SourceMaskState::Code
    } else {
        SourceMaskState::BlockComment
    }
}

fn mask_quoted(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
    delimiter: char,
    state: SourceMaskState,
) -> SourceMaskState {
    let ends = ch == delimiter;
    if ch == '\\' {
        chars.next();
    }
    output.push(if ch == '\n' { '\n' } else { ' ' });
    if ends {
        SourceMaskState::Code
    } else {
        state
    }
}
