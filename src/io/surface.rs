use crate::{
    cf::{self, Retained, Type},
    define_cf_type,
};

pub type SurfaceId = u32;

#[repr(i32)]
pub enum SurfaceComponentName {
    Unkown = 0,
    Alpha = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Luma = 5,
    ChromaRed = 6,
    ChromeBlue = 7,
}

#[repr(i32)]
pub enum SurfaceComponentType {
    Unknown = 0,
    UnsignedInteger = 1,
    SignedInteger = 2,
    Float = 3,
    SignedNormalized = 4,
}

#[repr(i32)]
pub enum SurfaceComponentRange {
    Unknown = 0,
    Full = 1,
    Video = 2,
    Wide = 3,
}

#[repr(i32)]
pub enum SurfaceSubsampling {
    Unknown = 0,
    None = 1, // Includes "4:4:4"
    _422 = 2, // Chroma downsampled by 2x1
    _420 = 3, // Chroma downsampled by 2x2
    _411 = 4, // Chroma downsampled by 4x1
}

#[repr(transparent)]
pub struct SurfaceLockOptions(pub cf::OptionFlags);

impl SurfaceLockOptions {
    pub const READ_ONLY: Self = Self(1);
    pub const AVOID_SYNC: Self = Self(2);
}

define_cf_type!(Surface(Type));

impl Surface {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { IOSurfaceGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    /// use cidre::io;
    ///
    ///
    /// let width = cf::Number::from_i32(100);
    /// let height = cf::Number::from_i32(200);
    ///
    /// let properties = cf::Dictionary::from_pairs(
    ///   &[
    ///     io::surface_keys::width(),
    ///     io::surface_keys::height()
    ///   ],
    ///   &[
    ///     &width,
    ///     &height
    ///   ]
    /// ).unwrap();
    ///
    /// let surf = io::Surface::create(&properties).unwrap();
    ///
    /// assert_eq!(100, surf.get_width());
    /// assert_eq!(200, surf.get_height());
    /// assert_eq!(0, surf.get_plane_count());
    /// assert_ne!(0, surf.get_id());
    ///
    /// let props = surf.copy_all_values().unwrap();
    /// props.show();
    /// assert_eq!(1, props.len());
    /// ```
    pub fn create<'a>(properties: &cf::Dictionary) -> Option<Retained<'a, Surface>> {
        unsafe { IOSurfaceCreate(properties) }
    }

    #[inline]
    pub fn get_id(&self) -> SurfaceId {
        unsafe { IOSurfaceGetID(&self) }
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        unsafe { IOSurfaceGetWidth(self) }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        unsafe { IOSurfaceGetHeight(self) }
    }

    #[inline]
    pub fn get_plane_count(&self) -> usize {
        unsafe { IOSurfaceGetPlaneCount(self) }
    }

    #[inline]
    pub fn get_plane_width(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetWidthOfPlane(self, plane_index) }
    }

    #[inline]
    pub fn get_plane_height(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::io;
    ///
    /// let surf = io::Surface::lookup(0);
    ///
    /// assert!(surf.is_none());
    /// ```
    pub fn lookup<'a>(csid: SurfaceId) -> Option<Retained<'a, Surface>> {
        unsafe { IOSurfaceLookup(csid) }
    }

    #[inline]
    pub fn copy_all_values<'a>(&self) -> Option<Retained<'a, cf::Dictionary>> {
        unsafe { IOSurfaceCopyAllValues(self) }
    }
}

extern "C" {
    fn IOSurfaceGetTypeID() -> cf::TypeId;
    fn IOSurfaceCreate<'a>(properties: &cf::Dictionary) -> Option<Retained<'a, Surface>>;
    fn IOSurfaceLookup<'a>(csid: SurfaceId) -> Option<Retained<'a, Surface>>;
    fn IOSurfaceGetID(buffer: &Surface) -> SurfaceId;
    fn IOSurfaceGetWidth(buffer: &Surface) -> usize;
    fn IOSurfaceGetHeight(buffer: &Surface) -> usize;
    fn IOSurfaceGetPlaneCount(buffer: &Surface) -> usize;
    fn IOSurfaceGetWidthOfPlane(buffer: &Surface, plane_index: usize) -> usize;
    fn IOSurfaceGetHeightOfPlane(buffer: &Surface, plane_index: usize) -> usize;

    fn IOSurfaceCopyAllValues<'a>(buffer: &Surface) -> Option<Retained<'a, cf::Dictionary>>;
}

/// The following list of properties are used with the cf::Dictionary passed to io::Surface::create
pub mod keys {
    use crate::cf::String;

    /// cf::Number of the total allocation size of the buffer including all planes.    
    /// Defaults to BufferHeight * BytesPerRow if not specified. Must be specified for
    /// dimensionless buffers.
    #[inline]
    pub fn alloc_size() -> &'static String {
        unsafe { kIOSurfaceAllocSize }
    }

    /// cf::Number for the width of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[inline]
    pub fn width() -> &'static String {
        unsafe { kIOSurfaceWidth }
    }

    /// cf::Number for the height of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[inline]
    pub fn height() -> &'static String {
        unsafe { kIOSurfaceHeight }
    }
    #[inline]
    pub fn bytes_per_row() -> &'static String {
        unsafe { kIOSurfaceBytesPerRow }
    }
    #[inline]
    pub fn bytes_per_element() -> &'static String {
        unsafe { kIOSurfaceBytesPerElement }
    }
    #[inline]
    pub fn element_width() -> &'static String {
        unsafe { kIOSurfaceElementWidth }
    }
    #[inline]
    pub fn element_height() -> &'static String {
        unsafe { kIOSurfaceElementHeight }
    }
    #[inline]
    pub fn offset() -> &'static String {
        unsafe { kIOSurfaceOffset }
    }
    #[inline]
    pub fn plane_info() -> &'static String {
        unsafe { kIOSurfacePlaneInfo }
    }
    #[inline]
    pub fn plane_width() -> &'static String {
        unsafe { kIOSurfacePlaneWidth }
    }
    #[inline]
    pub fn plane_height() -> &'static String {
        unsafe { kIOSurfacePlaneHeight }
    }
    #[inline]
    pub fn plane_bytes_per_row() -> &'static String {
        unsafe { kIOSurfacePlaneBytesPerRow }
    }

    extern "C" {
        static kIOSurfaceAllocSize: &'static String;
        static kIOSurfaceWidth: &'static String;
        static kIOSurfaceHeight: &'static String;
        static kIOSurfaceBytesPerRow: &'static String;
        static kIOSurfaceBytesPerElement: &'static String;
        static kIOSurfaceElementWidth: &'static String;
        static kIOSurfaceElementHeight: &'static String;
        static kIOSurfaceOffset: &'static String;
        static kIOSurfacePlaneInfo: &'static String;
        static kIOSurfacePlaneWidth: &'static String;
        static kIOSurfacePlaneHeight: &'static String;
        static kIOSurfacePlaneBytesPerRow: &'static String;
        // static kIOSurfacePlaneOffset: &'static String;
        // static kIOSurfacePlaneSize: &'static String;
        // static kIOSurfacePlaneBase: &'static String;
        // static kIOSurfacePlaneBitsPerElement: &'static String;
        // static kIOSurfacePlaneBytesPerElement: &'static String;
        // static kIOSurfacePlaneElementWidth: &'static String;
        // static kIOSurfacePlaneElementHeight: &'static String;
        // static kIOSurfaceCacheMode: &'static String;
        // static kIOSurfacePixelFormat: &'static String;
        // static kIOSurfacePixelSizeCastingAllowed: &'static String;
        // static kIOSurfacePlaneComponentBitDepths: &'static String;
        // static kIOSurfacePlaneComponentBitOffsets: &'static String;
    }
}