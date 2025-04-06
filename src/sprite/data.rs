use std::{collections::HashMap, path::Path, time::Duration};

use alpacker::{
    Pack,
    data::{aseprite::Sprite as AlpackerSprite, raylib::RaylibAsset},
};
use aseprite::SpritesheetData;
use raylib::{
    RaylibHandle, RaylibThread,
    math::{Rectangle, Vector2},
    texture::{Image, Texture2D},
};

use crate::Raylib;

#[derive(Debug)]
pub struct SpriteData {
    pub texture: Texture2D,
    pub frames: Vec<Frame>,
    pub tags: HashMap<String, FrameTag>,
}

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub duration: Duration,
    pub rect: Rectangle,
    pub offset: Vector2,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameTag {
    pub from: usize,
    pub to: usize,
}

impl FrameTag {
    pub const fn from_start(to: usize) -> Self {
        Self { from: 0, to }
    }
}

impl<'r> RaylibAsset<'r> for SpriteData {
    type System = Raylib;
    type Error = anyhow::Error;

    fn load(
        pack: &mut impl Pack,
        Raylib { rl, thread }: &'r mut Self::System,
        path: impl AsRef<Path>,
    ) -> Result<Self, Self::Error> {
        let AlpackerSprite { image, meta } = pack.get::<AlpackerSprite<Image>>(path)?;
        let image = image.unwrap();

        let texture = rl.load_texture_from_image(&thread, &image)?;

        let SpritesheetData { frames, meta } = meta;

        let frames = frames.into_iter().map(Frame::from).collect::<Vec<Frame>>();
        let tags = meta
            .frame_tags
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(frametag_from_aseprite)
            .collect::<HashMap<String, FrameTag>>();

        Ok(Self {
            texture,
            frames,
            tags,
        })
    }
}

const fn aseprite_rect_to_raylib(rect: aseprite::Rect) -> Rectangle {
    Rectangle::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.h as f32)
}

fn frametag_from_aseprite(tag: aseprite::Frametag) -> (String, FrameTag) {
    let aseprite::Frametag { name, from, to, .. } = tag;
    let tag = FrameTag {
        from: from as usize,
        to: to as usize,
    };
    (name, tag)
}

impl From<aseprite::Frame> for Frame {
    fn from(value: aseprite::Frame) -> Self {
        Self {
            offset: Vector2 {
                x: value.sprite_source_size.x as f32,
                y: value.sprite_source_size.y as f32,
            },
            rect: aseprite_rect_to_raylib(value.frame),
            duration: Duration::from_millis(value.duration as u64),
        }
    }
}
