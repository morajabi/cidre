pub mod geometry;
pub use geometry::Float;
pub use geometry::Point;
pub use geometry::Rect;
pub use geometry::Size;

pub mod color_space;
pub use color_space::ColorRenderingIntent;
pub use color_space::ColorSpace;
pub use color_space::ColorSpaceModel;

pub mod color;
pub use color::Color;

pub mod window;
pub use window::ID as WindowID;

pub mod direct_display;
pub use direct_display::ID as DirectDisplayID;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}