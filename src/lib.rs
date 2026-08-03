use std::{
    ffi::OsString,
    io::{self, Read},
};

use image::{
    ImageDecoder, ImageError, ImageReader, ImageResult,
    error::DecodingError,
    hooks::{self, GenericReader},
};
use zip::ZipArchive;

pub fn register() -> bool {
    hooks::register_decoding_hook(OsString::from("kra"), Box::new(hook))
}

fn hook<'a>(reader: GenericReader<'a>) -> ImageResult<Box<dyn ImageDecoder + 'a>> {
    let mut zip = ZipArchive::new(reader).map_err(to_image_error)?;

    let mut reader = zip.by_name("mergedimage.png").map_err(to_image_error)?;

    // reading it all and wrapping it with a Cursor
    // is the only way I know to give it Seek
    // (which is required by ImageReader)
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let image_reader = ImageReader::with_format(io::Cursor::new(buf), image::ImageFormat::Png);

    Ok(Box::new(image_reader.into_decoder()?))
}

fn to_image_error(
    e: impl Into<Box<dyn std::error::Error + std::marker::Send + Sync + 'static>>,
) -> ImageError {
    ImageError::Decoding(DecodingError::new(
        image::error::ImageFormatHint::PathExtension("kra".into()),
        e,
    ))
}
