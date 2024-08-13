use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use winresource::WindowsResource;

const ICON_PATH: &str = "./icons/icon.png";
const ICON_L_PATH: &str = "./icons/iconL.ico";
const ICON_RAW_PATH: &str = "icon.rgba8";
const WIDTH_RAW_PATH: &str = "icon-w";
const HEIGHT_RAW_PATH: &str = "icon-h";

fn main() {
    // Get icon from png
    let icon = image::open(ICON_PATH).expect("Failed to open icon path").into_rgba8();
    let (width, height) = icon.dimensions();

    let build_dir = std::env::var("OUT_DIR").unwrap();

    let mut opts = OpenOptions::new();
    opts.read(true).write(true).truncate(true).create(true);

    let mut f = opts.open(PathBuf::from(&build_dir).join(ICON_RAW_PATH)).unwrap();
    f.write_all(&icon.into_raw()).unwrap();

    let mut f = opts.open(PathBuf::from(&build_dir).join(WIDTH_RAW_PATH)).unwrap();
    f.write_all(&width.to_be_bytes()).unwrap();

    let mut f = opts.open(PathBuf::from(&build_dir).join(HEIGHT_RAW_PATH)).unwrap();
    f.write_all(&height.to_be_bytes()).unwrap();

    // Use icon for executable
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        WindowsResource::new().set_icon(ICON_L_PATH).compile().unwrap();
    }
}
