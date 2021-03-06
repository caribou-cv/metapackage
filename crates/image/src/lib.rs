extern crate image as piston_image;
extern crate imageproc as piston_imageproc;

pub use piston_image::{ConvertBuffer, GrayImage, Luma, Rgba, RgbaImage, SubImage};
pub use piston_imageproc::{filter, math, definitions};

pub use self::integral::IntegralImage;
pub use self::pyramid::Pyramid;

mod integral;
mod pyramid;

pub type GrayPyramid = Pyramid<GrayImage>;