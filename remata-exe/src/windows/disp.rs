use std::fmt;

impl fmt::Display for super::Windows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Windows PE Info ===")?;

        write_opt(f, "Machine Type", &self.machine_type)?;
        write_opt(f, "Timestamp", &self.time_stamp)?;
        write_opt_vec(f, "Characteristics", &self.image_file_characteristics)?;
        write_opt(f, "PE Type", &self.portable_type)?;
        write_opt(f, "Linker Version", &self.linker_version)?;
        write_opt(f, "Code Size", &self.code_size)?;
        write_opt(f, "Initialized Data Size", &self.initialized_data_size)?;
        write_opt(f, "Uninitialized Data Size", &self.uninitialized_data_size)?;
        write_opt(f, "Entry Point", &self.entry_point)?;
        write_opt(f, "OS Version", &self.os_version)?;
        write_opt(f, "Image Version", &self.image_version)?;
        write_opt(f, "Subsystem Version", &self.subsystem_version)?;
        write_opt(f, "Subsystem", &self.subsystem)?;

        Ok(())
    }
}

fn write_opt<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    label: &str,
    val: &Option<T>,
) -> fmt::Result {
    match val {
        Some(v) => writeln!(f, "{:<28}: {}", label, v),
        None => writeln!(f, "{:<28}: N/A", label),
    }
}

fn write_opt_vec<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    label: &str,
    val: &Option<Vec<T>>,
) -> fmt::Result {
    match val {
        Some(vec) if !vec.is_empty() => {
            let joined = vec.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "{:<28}: {}", label, joined)
        }
        _ => writeln!(f, "{:<28}: N/A", label),
    }
}