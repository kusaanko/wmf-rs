/// The BitmapInfoHeader Object contains information about the dimensions and
/// color format of a device-independent bitmap (DIB).
#[derive(Clone, Debug)]
pub struct BitmapInfoHeader {
    /// HeaderSize (4 bytes): A 32-bit unsigned integer that defines the size
    /// of this object, in bytes.
    pub header_size: u32,
    /// Width (4 bytes): A 32-bit signed integer that defines the width of the
    /// DIB, in pixels. This value MUST be positive.
    ///
    /// This field SHOULD specify the width of the decompressed image file, if
    /// the Compression value specifies JPEG or PNG format. (Windows NT 3.1,
    /// Windows NT 3.5, Windows NT 3.51, Windows 95, and Windows NT 4.0:
    /// Neither JPEG nor PNG format is supported.)
    pub width: i32,
    /// Height (4 bytes): A 32-bit signed integer that defines the height of
    /// the DIB, in pixels. This value MUST NOT be zero.
    ///
    /// | Value | Meaning |
    /// |-|-|
    /// | 0x00000000 < value | If this value is positive, the DIB is a bottom-up bitmap, and its origin is the lower-left corner. This field SHOULD specify the height of the decompressed image file, if the Compression value specifies JPEG or PNG format. |
    /// | value < 0x00000000 | If this value is negative, the DIB is a top-down bitmap, and its origin is the upper-left corner. Top-down bitmaps do not support compression. |
    pub height: i32,
    /// Planes (2 bytes): A 16-bit unsigned integer that defines the number of
    /// planes for the target device. This value MUST be 0x0001.
    pub planes: u16,
    /// BitCount (2 bytes): A 16-bit unsigned integer that defines the number
    /// of bits that define each pixel and the maximum number of colors in the
    /// DIB. This value MUST be in the BitCount Enumeration.
    pub bit_count: crate::BitCount,
    /// Compression (4 bytes): A 32-bit unsigned integer that defines the
    /// compression mode of the DIB. This value MUST be in the Compression
    /// Enumeration.
    ///
    /// This value MUST NOT specify a compressed format if the DIB is a
    /// top-down bitmap, as indicated by the Height value.
    pub compression: crate::Compression,
    /// ImageSize (4 bytes): A 32-bit unsigned integer that defines the size,
    /// in bytes, of the image.
    ///
    /// If the Compression value is BI_RGB, this value SHOULD be zero and MUST
    /// be ignored. (Windows implementations might write a nonzero value to
    /// this field, but it is ignored when the metafile is parsed.)
    ///
    /// If the Compression value is BI_JPEG or BI_PNG, this value MUST specify
    /// the size of the JPEG or PNG image buffer, respectively.
    pub image_size: u32,
    /// XPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
    /// horizontal resolution, in pixels-per-meter, of the target device for
    /// the DIB.
    pub x_pels_per_meter: i32,
    /// YPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
    /// vertical resolution, in pixels-per-meter, of the target device for the
    /// DIB.
    pub y_pels_per_meter: i32,
    /// ColorUsed (4 bytes): A 32-bit unsigned integer that specifies the
    /// number of indexes in the color table used by the DIB, as follows:
    ///
    /// - If this value is zero, the DIB uses the maximum number of colors that
    ///   correspond to the BitCount value.
    /// - If this value is nonzero and the BitCount value is less than 16, this
    ///   value specifies the number of colors used by the DIB.
    /// - If this value is nonzero and the BitCount value is 16 or greater,
    ///   this value specifies the size of the color table used to optimize
    ///   performance of the system palette.
    ///
    /// Note If this value is nonzero and greater than the maximum possible
    /// size of the color table based on the BitCount value, the maximum color
    /// table size SHOULD be assumed.
    pub color_used: u32,
    /// ColorImportant (4 bytes): A 32-bit unsigned integer that defines the
    /// number of color indexes that are required for displaying the DIB. If
    /// this value is zero, all color indexes are required.
    ///
    /// A DIB is specified by a DeviceIndependentBitmap Object.
    ///
    /// When the array of pixels in the DIB immediately follows the
    /// BitmapInfoHeader, the DIB is a packed bitmap. In a packed bitmap, the
    /// ColorUsed value MUST be either 0x00000000 or the actual size of the
    /// color table.
    pub color_important: u32,
}

impl BitmapInfoHeader {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (header_size, consumed_bytes) = crate::read_u32_from_le_bytes(buf)?;
        let (header, c) = Self::parse_with_header_size(buf, header_size)?;

        Ok((header, consumed_bytes + c))
    }
}

impl BitmapInfoHeader {
    pub fn parse_with_header_size<R: std::io::Read>(
        buf: &mut R,
        header_size: u32,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (width, width_bytes),
            (height, height_bytes),
            (planes, planes_bytes),
            (bit_count, bit_count_bytes),
            (compression, compression_bytes),
            (image_size, image_size_bytes),
            (x_pels_per_meter, x_pels_per_meter_bytes),
            (y_pels_per_meter, y_pels_per_meter_bytes),
            (color_used, color_used_bytes),
            (color_important, color_important_bytes),
        ) = (
            crate::read_i32_from_le_bytes(buf)?,
            crate::read_i32_from_le_bytes(buf)?,
            crate::read_u16_from_le_bytes(buf)?,
            crate::BitCount::parse(buf)?,
            crate::Compression::parse(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_i32_from_le_bytes(buf)?,
            crate::read_i32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
        );
        let consumed_bytes = width_bytes
            + height_bytes
            + planes_bytes
            + image_size_bytes
            + x_pels_per_meter_bytes
            + y_pels_per_meter_bytes
            + color_used_bytes
            + color_important_bytes
            + bit_count_bytes
            + compression_bytes;

        if width < 0 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The width field must be positive".to_owned(),
            });
        }

        if height == 0 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The height field must not be zero".to_owned(),
            });
        }

        if planes != 0x0001 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The planes field must be 0x01".to_owned(),
            });
        }

        Ok((
            Self {
                header_size,
                width,
                height,
                planes,
                bit_count,
                compression,
                image_size,
                x_pels_per_meter,
                y_pels_per_meter,
                color_used,
                color_important,
            },
            consumed_bytes,
        ))
    }
}
