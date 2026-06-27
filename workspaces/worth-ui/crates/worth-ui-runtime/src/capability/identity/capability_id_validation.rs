use super::capability_id_error::CapabilityIdError;

pub(super) fn validate_capability_id_text(raw_text: &str) -> Result<(), CapabilityIdError> {
    if raw_text.is_empty() {
        return Err(CapabilityIdError::Empty);
    }

    let mut segment_start = 0;
    let mut expecting_segment_start = true;

    for (byte_index, character) in raw_text.char_indices() {
        if character == '.' {
            if expecting_segment_start {
                return Err(CapabilityIdError::EmptySegment { byte_index });
            }
            expecting_segment_start = true;
            segment_start = byte_index + character.len_utf8();
            continue;
        }

        if expecting_segment_start {
            validate_segment_start(byte_index, character)?;
            expecting_segment_start = false;
            continue;
        }

        validate_segment_character(byte_index, character)?;
    }

    if expecting_segment_start {
        return Err(CapabilityIdError::EmptySegment {
            byte_index: segment_start,
        });
    }

    Ok(())
}

fn validate_segment_start(byte_index: usize, character: char) -> Result<(), CapabilityIdError> {
    if character.is_ascii_lowercase() {
        Ok(())
    } else {
        Err(CapabilityIdError::InvalidSegmentStart {
            byte_index,
            found: character,
        })
    }
}

fn validate_segment_character(byte_index: usize, character: char) -> Result<(), CapabilityIdError> {
    if character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_' {
        Ok(())
    } else {
        Err(CapabilityIdError::InvalidSegmentCharacter {
            byte_index,
            found: character,
        })
    }
}
