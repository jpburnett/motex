use std::{
    io::{self, Write, Read, Cursor},
    path::Path,
};
use byteorder::{BigEndian, ReadBytesExt};
use anyhow::Result;

use crate::n64_graphics::textures;

#[derive(Default)]
pub struct N64Image {
    format: textures::N64Codec,
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl N64Image {
    /// Load an image from a file.
    pub fn read<R: Read>(
        mut reader: R,
        format: textures::N64Codec,
        width: usize,
        height: usize,
    ) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            format,
            width,
            height,
            data,
        })
    }

    pub fn decode<W: Write>(&self, writer: &mut W) -> Result<()> {

        let mut cursor = Cursor::new(&self.data);

        match self.format {

            textures::N64Codec::I4 => {
                for _h in 0..self.height {
                    // Each row is 1/2 the width in bytes
                    // Each pixel is 4 bits, so 2 pixels per byte
                    for _w in (0..self.width).step_by(2) {
                        // Read a byte from the cursor
                        let byte = cursor.read_u8()?;

                        // Split the byte into two 4-bit values
                        let intensity = byte & 0xF0;
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;

                        let intensity = (byte & 0x0F) << 4;
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;
                    }
                }
            }

            textures::N64Codec::I8 => {
                for _h in 0..self.height {
                    for _w in 0..self.width {
                        let intensity = cursor.read_u8()?;
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;
                    }
                }
            }

            textures::N64Codec::RGBA32 => {
                for _h in 0..self.height {
                    for _w in 0..self.width {
                        let pixel = cursor.read_u32::<BigEndian>()?;
                        let color = textures::Color::from_u32(pixel);
                        writer.write_all(&[color.r, color.g, color.b, color.a])?;
                    }
                }
            }

            // TODO: Implement other formats...This is to get rust to not complain...
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unsupported format",
                )
                .into());
            }
        }

        Ok(())
    }
}
