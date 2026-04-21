use std::fmt;
use super::Zip;

impl fmt::Display for Zip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.local_file_headers.is_empty() {
            writeln!(f, "Local File Headers: {{[")?;
            for header in self.local_file_headers.iter() {

                writeln!(f, "\"")?;
                if let Some(ver) = &header.version_req_to_extract {
                    writeln!(f, "\tVersion Required to Extract: {}", ver)?;
                }
                if let Some(bf) = header.bit_flag {
                    writeln!(f, "\tBit Flag: {}", bf)?;
                }

                if let Some(comp) = header.compression {
                    writeln!(f, "\tCompression: {}", comp)?;
                }

                if let Some(time) = &header.modify_time {
                    writeln!(f, "\tModify Time: {}", time)?;
                }
                if let Some(date) = &header.modify_date {
                    writeln!(f, "\tModify Date: {}", date)?;
                }

                if let Some(crc) = header.crc {
                    writeln!(f, "\tCRC 32: {}", crc)?;
                }

                if let Some(comp) = header.compressed_size {
                    writeln!(f, "\tCompressed Size: {}", comp)?;
                }

                if let Some(decomp) = header.uncompressed_size {
                    writeln!(f, "\tUncompressed Size: {}", decomp)?;
                }

                if let Some(name_length) = header.file_name_length {
                    writeln!(f, "\tFile Name Length: {}", name_length)?;
                }
                if let Some(name) = &header.file_name {
                    writeln!(f, "\tFile Name: {}", name)?;
                }

                if let Some(extra_length) = header.extra_field_length {
                    writeln!(f, "\tExtra Field Length: {}", extra_length)?;
                }
                if let Some(extra) = &header.extra_field {
                    writeln!(f, "\tExtra Field: {:?}", extra)?;
                }
                writeln!(f, "\",")?
            }

            writeln!(f, "]}}")?;
        }
        if !self.central_directory_headers.is_empty() {
            writeln!(f, "Central Directory Headers: {{[")?;
            for header in self.central_directory_headers.iter() {

                if let Some(ver) = &header.version_made_by {
                    writeln!(f, "\tVersion Made By: {}", ver)?;
                }
                if let Some(ver) = &header.version_req_to_extract {
                    writeln!(f, "\tVersion Required to Extract: {}", ver)?;
                }
                if let Some(bf) = header.bit_flag {
                    writeln!(f, "\tBit Flag: {}", bf)?;
                }

                if let Some(comp) = header.compression {
                    writeln!(f, "\tCompression: {}", comp)?;
                }

                if let Some(time) = &header.modify_time {
                    writeln!(f, "\tModify Time: {}", time)?;
                }
                if let Some(date) = &header.modify_date {
                    writeln!(f, "\tModify Date: {}", date)?;
                }

                if let Some(crc) = header.crc {
                    writeln!(f, "\tCRC 32: {}", crc)?;
                }

                if let Some(comp) = header.compressed_size {
                    writeln!(f, "\tCompressed Size: {}", comp)?;
                }

                if let Some(decomp) = header.uncompressed_size {
                    writeln!(f, "\tUncompressed Size: {}", decomp)?;
                }

                if let Some(name_length) = header.file_name_length {
                    writeln!(f, "\tFile Name Length: {}", name_length)?;
                }
                if let Some(name) = &header.file_name {
                    writeln!(f, "\tFile Name: {}", name)?;
                }

                if let Some(extra_length) = header.extra_field_length {
                    writeln!(f, "\tExtra Field Length: {}", extra_length)?;
                }
                if let Some(extra) = &header.extra_field {
                    writeln!(f, "\tExtra Field: {:?}", extra)?;
                }

                if let Some(disk_num) = &header.disk_number_start {
                    writeln!(f, "\tDisk Number Start: {}", disk_num)?;
                }

                if let Some(attrs) = &header.internal_file_attributes {
                    writeln!(f, "\tInternal File Attributes: {}", attrs)?;
                }

                if let Some(attrs) = &header.external_file_attributes {
                    writeln!(f, "\tInternal File Attributes: {}", attrs)?;
                }

            }

            writeln!(f, "]}}")?;
        }
        Ok(())
    }
}
