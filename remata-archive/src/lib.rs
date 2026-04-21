//! Remata Archive
#![deny(missing_docs)]
pub mod iso;
pub use iso::Iso;
pub mod torrent;
pub use torrent::Torrent;
pub mod zip;
pub use zip::Zip;
pub mod gzip;
pub use gzip::Gzip;
pub mod rar;
pub use rar::{Rar, Rar4, Rar5};
pub mod xz;
pub use xz::Xz;
pub mod ar;
pub use ar::Ar;
pub mod tar;
pub use tar::Tar;
// pub mod sevenz;
// pub use sevenz::SevenZ;
// pub mod zst;
// pub use zst::Zstd;
