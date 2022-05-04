pub mod surface;

pub use surface::Surface;
pub use surface::SurfaceComponentName;
pub use surface::SurfaceComponentRange;
pub use surface::SurfaceId;
pub use surface::SurfaceLockOptions;
pub use surface::SurfaceSubsampling;

#[link(name = "IOSurface", kind = "framework")]
extern "C" {}