use super::{Allocator, Index, OptionFlags, Retained, Type, TypeId};

use crate::define_cf_type;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct StringEncoding(u32);

impl StringEncoding {
    pub const UTF8: Self = Self(0x08000100);
}

pub type UniChar = u16;

#[repr(transparent)]
pub struct StringCompareFlags(OptionFlags);

impl StringCompareFlags {
    pub const NONE: Self = Self(0);
    pub const CASE_INSENSITIVE: Self = Self(1);
    pub const BACKWARDS: Self = Self(4);
    pub const ANCHORED: Self = Self(8);
    pub const NON_LITERAL: Self = Self(16);
    pub const LOCALIZED: Self = Self(32);
    pub const NUMERACALLY: Self = Self(64);
    pub const DIACRITIC_INSENITIVE: Self = Self(128);
    pub const WIDTH_INSENSITIVE: Self = Self(256);
    pub const FORCED_ORDERING: Self = Self(512);
}

define_cf_type!(String(Type));

impl String {
    ///```
    /// use cidre::cf;
    ///
    /// assert_eq!(cf::String::type_id(), 7);
    ///```
    pub fn type_id() -> TypeId {
        unsafe { CFStringGetTypeID() }
    }
    #[inline]
    pub fn show_str(&self) {
        unsafe { CFShowStr(self) }
    }

    #[inline]
    pub fn get_length(&self) -> Index {
        unsafe { CFStringGetLength(self) }
    }

    #[inline]
    pub fn has_suffix(&self, suffix: &String) -> bool {
        unsafe { CFStringHasSuffix(self, suffix) }
    }

    #[inline]
    pub fn has_prefix(&self, prefix: &String) -> bool {
        unsafe { CFStringHasPrefix(self, prefix) }
    }

    #[inline]
    pub fn get_character_at_index(&self, idx: Index) -> UniChar {
        unsafe { CFStringGetCharacterAtIndex(self, idx) }
    }

    #[inline]
    pub fn create_copy(&self, alloc: Option<&Allocator>) -> Option<Retained<String>> {
        unsafe { CFStringCreateCopy(alloc, self) }
    }

    #[inline]
    pub fn create_with_bytes_no_copy<'a>(
        alloc: Option<&Allocator>,
        bytes: &'a [u8],
        num_bytes: Index,
        encoding: StringEncoding,
        is_external_representation: bool,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<'a, String>> {
        unsafe {
            let bytes = bytes.as_ptr();
            CFStringCreateWithBytesNoCopy(
                alloc,
                bytes,
                num_bytes,
                encoding,
                is_external_representation,
                contents_deallocator,
            )
        }
    }

    #[inline]
    pub fn create_mutable_copy(
        &self,
        alloc: Option<&Allocator>,
        max_length: Index,
    ) -> Option<Retained<MutableString>> {
        unsafe { CFStringCreateMutableCopy(alloc, max_length, self) }
    }
}

define_cf_type!(MutableString(String));

impl MutableString {
    #[inline]
    pub fn append(&mut self, appended_string: &String) {
        unsafe { CFStringAppend(self, appended_string) }
    }

    #[inline]
    pub fn trim(&mut self, trim_string: &String) {
        unsafe { CFStringTrim(self, trim_string) }
    }

    #[inline]
    pub fn trim_whitespace(&mut self) {
        unsafe { CFStringTrimWhitespace(self) }
    }

    #[inline]
    pub fn create(alloc: Option<&Allocator>, max_length: Index) -> Option<Retained<Self>> {
        unsafe { CFStringCreateMutable(alloc, max_length) }
    }
}

extern "C" {
    fn CFStringGetTypeID() -> TypeId;
    fn CFStringGetLength(the_string: &String) -> Index;
    fn CFStringCreateMutable(
        alloc: Option<&Allocator>,
        max_length: Index,
    ) -> Option<Retained<MutableString>>;
    fn CFStringCreateCopy<'a>(
        alloc: Option<&Allocator>,
        the_string: &String,
    ) -> Option<Retained<'a, String>>;
    fn CFStringHasPrefix(the_string: &String, prefix: &String) -> bool;
    fn CFStringHasSuffix(the_string: &String, prefix: &String) -> bool;
    fn CFStringCreateMutableCopy<'a>(
        alloc: Option<&Allocator>,
        max_length: Index,
        the_string: &String,
    ) -> Option<Retained<'a, MutableString>>;
    fn CFStringGetCharacterAtIndex(the_string: &String, idx: Index) -> UniChar;
    fn CFStringAppend(the_string: &mut MutableString, appended_string: &String);

    fn CFStringTrim(the_string: &MutableString, trim_string: &String);
    fn CFStringTrimWhitespace(the_string: &mut MutableString);

    fn CFStringCreateWithBytesNoCopy<'a>(
        alloc: Option<&Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: StringEncoding,
        is_external_representation: bool,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<'a, String>>;

    fn CFShowStr(str: &String);
}
