impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_CLIP_TO_PATH<
        R: std::io::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let (
            (byte_count, byte_count_bytes),
            (clip_function, clip_function_bytes),
            (reserved1, reserved1_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::PostScriptClipping::parse(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );
        record_size
            .consume(byte_count_bytes + clip_function_bytes + reserved1_bytes);

        if byte_count != 0x0004 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#06X}` field must be `0x0004`",
                ),
            });
        }

        if reserved1 != 0x0000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The reserved1 `{reserved1:#06X}` field should be `0x0000`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::CLIP_TO_PATH {
            record_size,
            record_function,
            byte_count,
            clip_function,
            reserved1,
        })
    }
}
