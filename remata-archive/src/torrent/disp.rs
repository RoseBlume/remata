use super::TorrentInfo;
use std::fmt;

impl fmt::Display for TorrentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.file_duration {
            writeln!(f, "File Duration: {}", v)?;
        }

        if let Some(v) = &self.file_media {
            writeln!(f, "File Media: {}", v)?;
        }

        if let Some(files) = &self.files {
            if !files.is_empty() {
                writeln!(f, "Files:")?;
                for file in files {
                    writeln!(f, "{}", file)?;
                }
            }
        }

        if let Some(v) = self.length {
            writeln!(f, "Length: {}", v)?;
        }

        if let Some(v) = &self.md5sum {
            writeln!(f, "MD5: {}", v)?;
        }

        if let Some(v) = &self.name {
            writeln!(f, "Name: {}", v)?;
        }

        if let Some(v) = &self.name_utf8 {
            writeln!(f, "Name UTF-8: {}", v)?;
        }

        if let Some(v) = self.piece_length {
            writeln!(f, "Piece Length: {}", v)?;
        }

        if let Some(pieces) = &self.pieces {
            if !pieces.is_empty() {
                writeln!(f, "Pieces:")?;

                for hash in pieces {
                    // Convert [u8; 20] → hex string
                    let hex = hash.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();

                    writeln!(f, "{}", hex)?;
                }
            }
        }

        if let Some(v) = self.private {
            writeln!(f, "Private: {}", v)?;
        }

        if let Some(profiles) = &self.profiles {
            if !profiles.is_empty() {
                writeln!(f, "Profiles:")?;
                for profile in profiles {
                    writeln!(f, "{}", profile)?;
                }
            }
        }

        Ok(())
    }
}