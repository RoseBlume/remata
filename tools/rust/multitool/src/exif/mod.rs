pub mod ifd;
pub mod tags;
pub mod gps;
pub mod starts;
pub mod helpers;

#[derive(Clone, Copy)]
pub enum ParseMode {
    Strict,
    Lenient,
}
