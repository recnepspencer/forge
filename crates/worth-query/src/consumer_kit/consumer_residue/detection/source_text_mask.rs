pub(super) fn mask_comments_and_string_literals(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut lexical_state = SourceTextMaskState::Code;

    while let Some(ch) = chars.next() {
        lexical_state = lexical_state.mask_next_character(ch, &mut chars, &mut output);
    }

    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SourceTextMaskState {
    Code,
    LineComment,
    BlockComment,
    StringLiteral,
    CharLiteral,
    RawStringLiteral { hash_count: usize },
}

impl SourceTextMaskState {
    fn mask_next_character(
        self,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
        output: &mut String,
    ) -> Self {
        match self {
            Self::Code => mask_code_character(ch, chars, output),
            Self::LineComment => mask_line_comment_character(ch, output),
            Self::BlockComment => mask_block_comment_character(ch, chars, output),
            Self::StringLiteral => mask_string_literal_character(ch, chars, output),
            Self::CharLiteral => mask_char_literal_character(ch, chars, output),
            Self::RawStringLiteral { hash_count } => {
                mask_raw_string_literal_character(ch, chars, output, hash_count)
            }
        }
    }
}

fn mask_code_character(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceTextMaskState {
    if ch == '/' && chars.peek() == Some(&'/') {
        chars.next();
        output.push(' ');
        return SourceTextMaskState::LineComment;
    }
    if ch == '/' && chars.peek() == Some(&'*') {
        chars.next();
        output.push(' ');
        return SourceTextMaskState::BlockComment;
    }
    if ch == '"' {
        output.push(' ');
        return SourceTextMaskState::StringLiteral;
    }
    if ch == '\'' {
        output.push(' ');
        return SourceTextMaskState::CharLiteral;
    }
    if ch == 'r' {
        if let Some((hash_count, consumed_delimiter_width)) = try_enter_raw_string(chars) {
            output.extend(std::iter::repeat_n(' ', consumed_delimiter_width + 1));
            return SourceTextMaskState::RawStringLiteral { hash_count };
        }
    }
    output.push(ch);
    SourceTextMaskState::Code
}

fn mask_line_comment_character(ch: char, output: &mut String) -> SourceTextMaskState {
    if ch == '\n' {
        output.push('\n');
        SourceTextMaskState::Code
    } else {
        output.push(' ');
        SourceTextMaskState::LineComment
    }
}

fn mask_block_comment_character(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceTextMaskState {
    let ends_comment = ch == '*' && chars.peek() == Some(&'/');
    if ends_comment {
        chars.next();
    }
    output.push(if ch == '\n' { '\n' } else { ' ' });
    if ends_comment {
        SourceTextMaskState::Code
    } else {
        SourceTextMaskState::BlockComment
    }
}

fn mask_string_literal_character(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceTextMaskState {
    let ends_string = ch == '"';
    if ch == '\\' {
        chars.next();
    }
    output.push(' ');
    if ends_string {
        SourceTextMaskState::Code
    } else {
        SourceTextMaskState::StringLiteral
    }
}

fn mask_char_literal_character(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
) -> SourceTextMaskState {
    let ends_char = ch == '\'';
    if ch == '\\' {
        chars.next();
    }
    output.push(' ');
    if ends_char {
        SourceTextMaskState::Code
    } else {
        SourceTextMaskState::CharLiteral
    }
}

fn mask_raw_string_literal_character(
    ch: char,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    output: &mut String,
    hash_count: usize,
) -> SourceTextMaskState {
    let ends_raw_string = ch == '"' && consume_raw_string_hashes(chars, hash_count);
    if ends_raw_string {
        output.extend(std::iter::repeat_n(' ', hash_count));
    }
    output.push(if ch == '\n' { '\n' } else { ' ' });
    if ends_raw_string {
        SourceTextMaskState::Code
    } else {
        SourceTextMaskState::RawStringLiteral { hash_count }
    }
}

fn try_enter_raw_string(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Option<(usize, usize)> {
    let mut clone = chars.clone();
    let mut hash_count = 0;
    while clone.peek() == Some(&'#') {
        clone.next();
        hash_count += 1;
    }
    if clone.next() != Some('"') {
        return None;
    }
    for _ in 0..hash_count {
        chars.next();
    }
    chars.next();
    Some((hash_count, hash_count + 1))
}

fn consume_raw_string_hashes(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    hash_count: usize,
) -> bool {
    let mut clone = chars.clone();
    for _ in 0..hash_count {
        if clone.next() != Some('#') {
            return false;
        }
    }
    for _ in 0..hash_count {
        chars.next();
    }
    true
}
