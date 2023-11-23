// read_crapmap.rs
use std::fs::File;
use std::io::{self, Read};
use byteorder::{ReadBytesExt};

pub(crate) const BACKGROUND_COLOR: [u8; 4] = [111, 111, 111, 254];

pub fn read_crapmap(filename: &str) -> io::Result<Vec<Vec<(u8, u8, u8, u8)>>> {
    read_crapmap_transparent(filename, BACKGROUND_COLOR)
}

pub fn read_crapmap_transparent(filename: &str, transparent_color: [u8; 4]) -> io::Result<Vec<Vec<(u8, u8, u8, u8)>>> {
    let mut f = File::open(filename)?;

    // Read and check magic bytes
    let mut magic = [0; 4];
    f.read_exact(&mut magic)?;
    if &magic != b"CRAP" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic bytes"));
    }

    // Read version
    let version = f.read_u8()?;
    if version != 0x01 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid version"));
    }

    // Read width and height
    let width = f.read_u8()?;
    let height = f.read_u8()?;

    // Read colors (if present)
    let colors_byte = f.read_u8()?;
    let mut color_table = Vec::new();

    if colors_byte != 0 {
        for _ in 0..colors_byte {
            let r = f.read_u8()?;
            let g = f.read_u8()?;
            let b = f.read_u8()?;
            color_table.push((r, g, b));
        }
    }

    // Read pixel data
    let mut image_data = Vec::new();
    for _ in 0..height {
        let mut image_row: Vec<(u8, u8, u8, u8)> = Vec::new();
        for _ in 0..width {
            let pixel = f.read_u8()?;
            if pixel == 0x00 {
                // transparent is the background color
                image_row.push((
                    transparent_color[0],
                    transparent_color[1],
                    transparent_color[2],
                    transparent_color[3]
                ));
            } else if (1..=color_table.len() as u8).contains(&pixel) {
                let color = color_table[(pixel - 1) as usize];
                image_row.push((color.0, color.1, color.2, 255));
            } else {
                image_row.push((0, 0, 0, 255));
            }
        }
        image_data.push(image_row);
    }

    Ok(image_data)
}
