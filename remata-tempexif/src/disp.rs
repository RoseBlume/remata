use super::ExifData;
use std::fmt;

impl fmt::Display for ExifData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ---------- Single-value fields ----------
        if !self.data.is_empty() {
            writeln!(f, "Single-value metadata:")?;

            let mut keys: Vec<_> = self.data.keys().collect();
            keys.sort();

            for key in keys {
                if let Some(value) = self.data.get(key) {
                    writeln!(f, "  {}: {}", key, value)?;
                }
            }
        }

        // ---------- Multi-value fields ----------
        if !self.vec_data.is_empty() {
            if !self.data.is_empty() {
                writeln!(f)?; // spacing between sections
            }

            writeln!(f, "Multi-value metadata:")?;

            let mut keys: Vec<_> = self.vec_data.keys().collect();
            keys.sort();

            for key in keys {
                if let Some(values) = self.vec_data.get(key) {
                    write!(f, "  {}: ", key)?;

                    if values.len() == 1 {
                        writeln!(f, "{}", values[0])?;
                    } else {
                        writeln!(f, "[{}]", values.join(", "))?;
                    }
                }
            }
        }

        // ---------- Empty case ----------
        if self.data.is_empty() && self.vec_data.is_empty() {
            write!(f, "No EXIF metadata")?;
        }

        Ok(())
    }
}