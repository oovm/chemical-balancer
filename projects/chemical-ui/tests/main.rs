use image::{io::Reader as ImageReader, ImageFormat};
use std::{
    fs::File,
    io::{Cursor, Write},
    path::PathBuf,
};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn prepare_default_icon() -> std::io::Result<()> {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let png: &[u8] = include_bytes!("default_icon.png");
    let mut reader = ImageReader::new(Cursor::new(png));
    reader.set_format(ImageFormat::Png);
    let icon = reader.decode().unwrap();
    let bin = here.join("default_icon.bin");
    println!("{:?}", bin);
    let mut file = File::create(bin).unwrap();
    file.write_all(icon.as_bytes()).unwrap();
    println!("({}, {})", icon.width(), icon.height());
    Ok(())
}
