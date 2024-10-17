/// The Font object specifies the attributes of a logical font.
#[derive(Clone, Debug)]
pub struct Font {
    /// Height (2 bytes): A 16-bit signed integer that specifies the height, in
    /// logical units, of the font's character cell. The character height is
    /// computed as the character cell height minus the internal leading.
    ///
    /// For all height comparisons, the font mapper SHOULD find the largest
    /// physical font that does not exceed the requested size.
    ///
    /// All Windows versions: mapping the logical font size to the available
    /// physical fonts occurs the first time the logical font needs to be used
    /// in a drawing operation. For the MM_TEXT mapping mode, the following
    /// formula can be used to compute the height of a font with a specified
    /// point size.
    ///
    /// ```
    /// Height = -MulDiv(PointSize, GetDeviceCaps(hDC, LOGPIXELSY), 72);
    /// ```
    pub height: i16,
    /// Width (2 bytes): A 16-bit signed integer that defines the average
    /// width, in logical units, of characters in the font. If Width is 0x0000,
    /// the aspect ratio of the device SHOULD be matched against the
    /// digitization aspect ratio of the available fonts to find the closest
    /// match, determined by the absolute value of the difference.
    pub width: i16,
    /// Escapement (2 bytes): A 16-bit signed integer that defines the angle,
    /// in tenths of degrees, between the escapement vector and the x-axis of
    /// the device. The escapement vector is parallel to the base line of a row
    /// of text.
    pub escapement: i16,
    /// Orientation (2 bytes): A 16-bit signed integer that defines the angle,
    /// in tenths of degrees, between each character's base line and the x-axis
    /// of the device.
    pub orientation: i16,
    /// Weight (2 bytes): A 16-bit signed integer that defines the weight of
    /// the font in the range 0 through 1000. For example, 400 is normal and
    /// 700 is bold. If this value is 0x0000, a default weight SHOULD be used.
    pub weight: i16,
    /// Italic (1 byte): A 8-bit Boolean value that specifies the italic
    /// attribute of the font.
    pub italic: bool,
    /// Underline (1 byte): An 8-bit Boolean value that specifies the underline
    /// attribute of the font.
    pub underline: bool,
    /// StrikeOut (1 byte): An 8-bit Boolean value that specifies the strikeout
    /// attribute of the font.
    pub strike_out: bool,
    /// CharSet (1 byte): An 8-bit unsigned integer that defines the character
    /// set. It SHOULD be set to a value in the CharacterSet Enumeration.
    ///
    /// The DEFAULT_CHARSET value MAY be used to allow the name and size of a
    /// font to fully describe the logical font. If the specified font name
    /// does not exist, a font in another character set MAY be substituted. The
    /// DEFAULT_CHARSET value is set to a value based on the current system
    /// locale. For example, when the system locale is United States, it is set
    /// to ANSI_CHARSET.
    ///
    /// If a typeface name in the FaceName field is specified, the CharSet
    /// value MUST match the character set of that typeface.
    pub charset: crate::parser::CharacterSet,
    /// OutPrecision (1 byte): An 8-bit unsigned integer that defines the
    /// output precision. The output precision defines how closely the output
    /// matches the requested font height, width, character orientation,
    /// escapement, pitch, and font type. It MUST be one of the values from the
    /// OutPrecision Enumeration.
    ///
    /// Applications can use the OUT_DEVICE_PRECIS, OUT_RASTER_PRECIS,
    /// OUT_TT_PRECIS, and OUT_PS_ONLY_PRECIS values to control how the font
    /// mapper selects a font when the operating system contains more than one
    /// font with a specified name. For example, if an operating system
    /// contains a font named "Symbol" in raster and TrueType forms, specifying
    /// OUT_TT_PRECIS forces the font mapper to select the TrueType version.
    /// Specifying OUT_TT_ONLY_PRECIS forces the font mapper to select a
    /// TrueType font, even if it substitutes a TrueType font of another name.
    pub out_precision: crate::parser::OutPrecision,
    /// ClipPrecision (1 byte): An 8-bit unsigned integer that defines the
    /// clipping precision. The clipping precision defines how to clip
    /// characters that are partially outside the clipping region. It MUST be a
    /// combination of one or more of the bit settings in the ClipPrecision
    /// Flags.
    pub clip_precision: crate::parser::ClipPrecision,
    /// Quality (1 byte): An 8-bit unsigned integer that defines the output
    /// quality. The output quality defines how carefully to attempt to match
    /// the logical font attributes to those of an actual physical font. It
    /// MUST be one of the values in the FontQuality Enumeration.
    pub quality: crate::parser::FontQuality,
    /// PitchAndFamily (1 byte): A PitchAndFamily Object that defines the pitch
    /// and the family of the font. Font families specify the look of fonts in
    /// a general way and are intended for specifying fonts when the exact
    /// typeface wanted is not available.
    pub pitch_and_family: crate::parser::PitchAndFamily,
    /// Facename (32 bytes): A null-terminated string of up to 32 8-bit Latin-1
    /// [ISO/IEC-8859-1] ANSI characters that specifies the typeface name of
    /// the font. Any characters following the terminating null are ignored.
    pub facename: String,
}

impl Font {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (height, height_bytes),
            (width, width_bytes),
            (escapement, escapement_bytes),
            (orientation, orientation_bytes),
            (weight, weight_bytes),
            (italic, italic_bytes),
            (underline, underline_bytes),
            (strike_out, strike_out_bytes),
            (charset, charset_bytes),
            (out_precision, out_precision_bytes),
            (clip_precision, clip_precision_bytes),
            (quality, quality_bytes),
            (pitch_and_family, pitch_and_family_bytes),
        ) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            {
                let (v, c) = crate::parser::read_u8_from_le_bytes(buf)?;
                (v == 0x01, c)
            },
            {
                let (v, c) = crate::parser::read_u8_from_le_bytes(buf)?;
                (v == 0x01, c)
            },
            {
                let (v, c) = crate::parser::read_u8_from_le_bytes(buf)?;
                (v == 0x01, c)
            },
            crate::parser::CharacterSet::parse(buf)?,
            crate::parser::OutPrecision::parse(buf)?,
            crate::parser::ClipPrecision::parse(buf)?,
            crate::parser::FontQuality::parse(buf)?,
            crate::parser::PitchAndFamily::parse(buf)?,
        );
        let mut consumed_bytes = height_bytes
            + width_bytes
            + escapement_bytes
            + orientation_bytes
            + weight_bytes
            + italic_bytes
            + underline_bytes
            + strike_out_bytes
            + charset_bytes
            + out_precision_bytes
            + clip_precision_bytes
            + quality_bytes
            + pitch_and_family_bytes;

        let facename = {
            let mut bytes = vec![];
            let c = buf.read_to_end(&mut bytes).map_err(|err| {
                crate::parser::ParseError::UnexpectedPattern {
                    cause: err.to_string(),
                }
            })?;
            consumed_bytes += c;

            // Find the position of the first null byte (0)
            let len = bytes.iter().position(|&c| c == 0).unwrap_or(32);
            let encoding: &'static encoding_rs::Encoding = charset.into();

            let (cow, _, had_errors) = encoding.decode(&bytes[..len]);
            if had_errors {
                "undecodable facename".to_owned()
            } else {
                cow.trim_end().to_owned()
            }
        };

        Ok((
            Self {
                height,
                width,
                escapement,
                orientation,
                weight,
                italic,
                underline,
                strike_out,
                charset,
                out_precision,
                clip_precision,
                quality,
                pitch_and_family,
                facename,
            },
            consumed_bytes,
        ))
    }
}
