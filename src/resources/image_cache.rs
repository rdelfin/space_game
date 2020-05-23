use anyhow::Result;
use ggez::graphics::Image;
use ggez::Context;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Default)]
pub struct ImageCache {
    images: HashMap<String, Image>,
}

#[derive(Error, Debug)]
pub enum ImageCacheError {
    #[error("Image at path {0} has not been loaded")]
    ImageNotLoaded(String),
}

impl ImageCache {
    pub fn fetch_image(&mut self, ctx: &mut Context, path: &str) -> Result<&Image> {
        Ok(self
            .images
            .entry(path.to_string())
            .or_insert(Image::new(ctx, path)?))
    }

    pub fn get_image_res(&self, path: &str) -> Result<&Image> {
        Ok(self
            .get_image(path)
            .ok_or_else(|| ImageCacheError::ImageNotLoaded(path.to_string()))?)
    }

    pub fn get_image(&self, path: &str) -> Option<&Image> {
        self.images.get(path)
    }
}
