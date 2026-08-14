use std::io::Read;

use image::{
    ImageDecoder, ImageError, ImageResult,
    error::DecodingError,
    hooks::{self, GenericReader},
};
use resvg::usvg::{Options, Transform, Tree};

pub fn register() -> bool {
    hooks::register_decoding_hook("svg".into(), Box::new(hook))
        && hooks::register_decoding_hook("svgz".into(), Box::new(hook))
}

fn hook<'a>(mut reader: GenericReader<'a>) -> ImageResult<Box<dyn ImageDecoder + 'a>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let tree = Tree::from_data(&buf, &Options::default()).map_err(to_image_error)?;
    Ok(Box::new(SvgDecoder::new(tree)))
}

struct SvgDecoder {
    tree: Tree,
}
impl SvgDecoder {
    fn new(tree: Tree) -> Self {
        Self { tree }
    }
}
impl ImageDecoder for SvgDecoder {
    fn color_type(&self) -> image::ColorType {
        image::ColorType::Rgba8
    }
    fn dimensions(&self) -> (u32, u32) {
        self.tree.size().to_int_size().dimensions()
    }
    fn read_image(self, buf: &mut [u8]) -> ImageResult<()>
    where
        Self: Sized,
    {
        let (width, height) = self.dimensions();
        let mut pixmap = resvg::tiny_skia::PixmapMut::from_bytes(buf, width, height).unwrap();
        resvg::render(&self.tree, Transform::default(), &mut pixmap);

        Ok(())
    }
    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        (*self).read_image(buf)
    }
}

fn to_image_error(
    e: impl Into<Box<dyn std::error::Error + std::marker::Send + Sync + 'static>>,
) -> ImageError {
    ImageError::Decoding(DecodingError::new(
        image::error::ImageFormatHint::Name("SVG".into()),
        e,
    ))
}
