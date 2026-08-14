#[cfg(feature = "kra")]
pub mod kra;
#[cfg(feature = "svg")]
pub mod svg;

pub fn register() -> bool {
    let is_already_registered = if cfg!(feature = "kra") {
        kra::register()
    } else {
        true
    } && if cfg!(feature = "svg") {
        svg::register()
    } else {
        true
    };

    is_already_registered
}
