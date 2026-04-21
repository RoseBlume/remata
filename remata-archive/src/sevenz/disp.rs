use std::fmt;

impl fmt::Display for super::SevenZFolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SevenZFolder {{")?;

        writeln!(f, "  coders: {} entries", self.coders.len())?;

        writeln!(f, "  bind_pairs: [")?;
        for (a, b) in &self.bind_pairs {
            writeln!(f, "    ({}, {})", a, b)?;
        }
        writeln!(f, "  ]")?;

        writeln!(f, "  packed_streams: {:?},", self.packed_streams)?;

        match self.unpacked_size {
            Some(size) => writeln!(f, "  unpacked_size: {},", size)?,
            None => writeln!(f, "  unpacked_size: None,")?,
        }

        match self.crc {
            Some(crc) => writeln!(f, "  crc: {:#010x}", crc)?,
            None => writeln!(f, "  crc: None")?,
        }

        write!(f, "}}")
    }
}
