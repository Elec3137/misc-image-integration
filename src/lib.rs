#[cfg(feature = "kra")]
pub mod kra;
#[cfg(feature = "svg")]
pub mod svg;

pub fn register() -> bool {
    let kra_already_registered;
    #[cfg(feature = "kra")]
    {
        kra_already_registered = kra::register();
    }
    #[cfg(not(feature = "kra"))]
    {
        kra_already_registered = true;
    }

    let svg_already_registered;
    #[cfg(feature = "svg")]
    {
        svg_already_registered = svg::register();
    }
    #[cfg(not(feature = "svg"))]
    {
        svg_already_registered = true;
    }

    let is_already_registered = kra_already_registered && svg_already_registered;

    is_already_registered
}
