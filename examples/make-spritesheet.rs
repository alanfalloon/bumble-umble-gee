//! Prepare the sprite sheet and generate the import code.
use convert_case::{Case, Casing};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbaImage,
};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

type SpriteFrames = BTreeMap<String, BTreeMap<usize, PathBuf>>;

fn main() {
    let out_dir = PathBuf::from("src");
    let spritesheet_rs_path = out_dir.join("spritesheet.rs");
    let spritesheet_png_path = out_dir.join("spritesheet.png");
    let frames = find_all_sprite_frames();
    println!("Found frames {:#?}", &frames);
    let rows: BTreeMap<String, Row> = frames
        .iter()
        .map(|(k, v)| (k.to_owned(), Row::from_frames(v.values())))
        .collect();
    let spritesheet_x = rows
        .values()
        .map(|r| r.frame_size.0 * r.frames.len() as u32)
        .max()
        .unwrap();
    let spritesheet_y: u32 = rows.values().map(|r| r.frame_size.1).sum();
    println!(
        "Creating empty spritesheet image buffer {}x{}",
        spritesheet_x, spritesheet_y
    );
    let mut spritesheet: RgbaImage = ImageBuffer::new(spritesheet_x, spritesheet_y);
    println!("Done creating empty spritesheet");
    let mut spritesheet_rs = io::BufWriter::new(fs::File::create(&spritesheet_rs_path).unwrap());
    spritesheet_rs
        .write_all(
            br##"
#![allow(dead_code)]
// Autogenerated using: cargo run --example make-spritesheet
use macroquad::math::{Rect, XY};

pub struct FrameRect {
    pub pixel_offset: XY<u32>,
    pub xy: Rect,
    pub uv: Rect,
}

pub const SPRITESHEET_PNG_BYTES: &[u8] = include_bytes!("spritesheet.png");
"##,
        )
        .unwrap();
    let mut y_offset = 0;
    for (name, row) in rows.into_iter() {
        let var_prefix = name.to_case(Case::ScreamingSnake);
        let (frame_x, frame_y) = row.frame_size;
        let frame_u = frame_x as f32 / spritesheet_x as f32;
        let frame_v = frame_y as f32 / spritesheet_y as f32;
        let v = y_offset as f32 / spritesheet_y as f32;
        spritesheet_rs
            .write_all(
                format!(
                    r##"
pub const {var}_FRAME_SIZE: XY<u32> = XY {{
    x: {}u32,
    y: {}u32,
}};
pub const {var}_FRAME_UV: XY<f32> = XY {{
    x: {}f32,
    y: {}f32,
}};
pub const {var}_SPRITE_Y_OFFSET: u32 = {}u32;
pub const {var}_FRAMES: [FrameRect; {}] = [
"##,
                    frame_x,
                    frame_y,
                    frame_u,
                    frame_v,
                    y_offset,
                    row.frames.len(),
                    var = &var_prefix,
                )
                .as_bytes(),
            )
            .unwrap();
        println!(
            "Frames for {} ({}) {} at {}x{} ({:.3}x{:.3}uv); y-offset:{} (v:{:.3})",
            &name,
            &var_prefix,
            row.frames.len(),
            frame_x,
            frame_y,
            frame_u,
            frame_v,
            y_offset,
            v
        );
        for (frame_num, frame_src_image) in row.frames.into_iter().enumerate() {
            let x_offset = frame_num as u32 * frame_x;
            let u = x_offset as f32 / spritesheet_x as f32;
            println!("Frame {} x-offset: {} ({:.3}u)", frame_num, x_offset, u);
            println!("Resizing frame {}", frame_num);
            let frame_img = resize(&frame_src_image, frame_x, frame_y, FilterType::Lanczos3);
            println!(
                "Copying frame {} to spritesheet at {}x{} ({}x{}uv)",
                frame_num, x_offset, y_offset, u, v
            );
            spritesheet
                .copy_from(&frame_img, x_offset, y_offset)
                .unwrap();
            spritesheet_rs
                .write_all(
                    format!(
                        r##"    FrameRect {{
        pixel_offset: XY {{ x: {x}u32, y: {y}u32 }},
        xy: Rect {{ x: {x}f32, y: {y}f32, w: {w}f32, h: {h}f32 }},
        uv: Rect {{ x: {u}f32, y: {v}f32, w: {fu}f32, h: {fv}f32 }},
    }},
"##,
                        x = x_offset,
                        y = y_offset,
                        u = u,
                        v = v,
                        w = frame_x,
                        h = frame_y,
                        fu = frame_u,
                        fv = frame_v,
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        spritesheet_rs.write_all(b"];\n\n").unwrap();
        y_offset += frame_y;
    }
    spritesheet
        .save_with_format(&spritesheet_png_path, image::ImageFormat::Png)
        .unwrap();
}

/// Search the resources/ dir for all the .png and assume they are in
/// "foo-bar-1.png", "foo-bar-2.png", etc. Return a map of the series name
/// ("foo-bar") to maps of index to the path for that frame.
fn find_all_sprite_frames() -> SpriteFrames {
    let mut frames: BTreeMap<String, BTreeMap<usize, PathBuf>> = BTreeMap::new();
    for dirent in fs::read_dir("resources").unwrap() {
        let dirent = dirent.unwrap();
        println!("Looking at dirent {:?}", &dirent);
        let path = dirent.path();
        if path.extension() == Some(OsStr::new("png")) {
            println!("Found a PNG: {}", path.display());
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let (series, index) = file_stem.rsplit_once('-').unwrap();
            let index = usize::from_str_radix(index, 10).unwrap();
            println!("Found {} frame {} at {}", &series, &index, path.display());
            let series = frames
                .entry(series.to_owned())
                .or_insert_with(BTreeMap::new);
            let _: Option<()> = series.insert(index, path.clone()).map(|old_path| {
                panic!(
                    "Index {} found at path {} and path {}",
                    index,
                    old_path.display(),
                    path.display()
                );
            });
        }
    }
    frames
}

/// Tracks the size of each row of sprites, and the location in the row of each
/// frame.
#[derive(Clone)]
struct Row {
    frame_size: (u32, u32),
    frames: Vec<DynamicImage>,
}
impl Row {
    fn from_frames<I, P>(frame_paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut frame_paths = frame_paths.into_iter();
        let frame_img = get_image(frame_paths.next().unwrap().as_ref());
        let original_size = frame_img.dimensions();
        let mut frames = vec![frame_img];
        for frame_path in frame_paths {
            let frame_img = get_image(frame_path.as_ref());
            let this_size = frame_img.dimensions();
            assert_eq!(
                this_size,
                original_size,
                "Frame {} has wrong size",
                frame_path.as_ref().display()
            );
            frames.push(frame_img);
        }
        // Sprites are scaled down until the largest dimension is MAX
        const MAX: u32 = 256;
        let frame_size = if original_size.0 > original_size.1 {
            let scaled = original_size.1 as f32 * MAX as f32 / original_size.0 as f32;
            (MAX, scaled as u32)
        } else {
            let scaled = original_size.0 as f32 * MAX as f32 / original_size.1 as f32;
            (scaled as u32, MAX)
        };
        Self { frame_size, frames }
    }
}

fn get_image<P: AsRef<Path>>(path: P) -> DynamicImage {
    let path = path.as_ref();
    println!("Loading {}", path.display());
    let img = image::load(
        io::BufReader::new(fs::File::open(path).unwrap()),
        image::ImageFormat::Png,
    )
    .unwrap();
    println!("Done Loading {}", path.display());
    img
}
