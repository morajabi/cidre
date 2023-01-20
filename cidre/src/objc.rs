use std::{
    arch::asm,
    borrow::Cow,
    ffi::c_void,
    intrinsics::transmute,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{arc, cf::Type};
#[derive(Debug)]
#[repr(transparent)]
pub struct Class<T: Obj>(Type, PhantomData<T>);

impl<T: Obj> Class<T> {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[msg_send(alloc)]
    pub unsafe fn alloc(&self) -> &'static T;

    // in general alloc_init is faster
    #[msg_send(new)]
    pub unsafe fn new(&self) -> arc::R<T>;
}

impl<T: Obj> Obj for Class<T> {}

impl<T: Obj> arc::Release for T {
    #[inline]
    unsafe fn release(&mut self) {
        objc_release(transmute(self))
    }
}

impl<T: Obj> arc::Retain for T {
    fn retained(&self) -> arc::R<Self> {
        unsafe { Self::retain(self) }
    }
}

pub trait Obj: arc::Retain {
    #[inline]
    unsafe fn retain(id: &Self) -> arc::R<Self> {
        transmute(objc_retain(transmute(id)))
    }

    #[msg_send(description)]
    fn description_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain()]
    fn description(&self) -> arc::R<crate::ns::String>;

    #[msg_send(debugDescription)]
    fn debug_description_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain()]
    fn debug_description(&self) -> arc::R<crate::ns::String>;

    #[msg_send(respondsToSelector:)]
    fn responds_to_sel(&self, sel: &Sel) -> bool;

    #[inline]
    fn is_tagged_ptr(&self) -> bool {
        ((self as *const Self as usize) >> 63) == 1
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel0<R>(&self, selector: &Sel) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel1<R, A>(&self, selector: &Sel, a: A) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel2<R, A, B>(&self, selector: &Sel, a: A, b: B) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel3<R, A, B, C>(&self, selector: &Sel, a: A, b: B, c: C) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B, C) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel4<R, A, B, C, D>(&self, selector: &Sel, a: A, b: B, c: C, d: D) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B, C, D) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel5<R, A, B, C, D, E>(&self, selector: &Sel, a: A, b: B, c: C, d: D, e: E) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B, C, D, E) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel6<R, A, B, C, D, E, F>(
        &self,
        selector: &Sel,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
    ) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B, C, D, E, F) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e, f)
    }

    /// # Safety
    /// use `msg_send!`
    #[inline]
    unsafe fn sel7<R, A, B, C, D, E, F, G>(
        &self,
        selector: &Sel,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
    ) -> R {
        let imp: unsafe extern "C" fn(&Self, &Sel, A, B, C, D, E, F, G) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e, f, g)
    }

    #[inline]
    unsafe fn call0<R>(&self, send: unsafe extern "C" fn()) -> R {
        let imp: unsafe extern "C" fn(&Self) -> R = transmute(send as *const c_void);
        imp(self)
    }

    #[inline]
    unsafe fn call1<R, A>(&self, send: unsafe extern "C" fn(), a: A) -> R {
        let imp: unsafe extern "C" fn(&Self, *const c_void, A) -> R =
            transmute(send as *const c_void);
        imp(self, std::ptr::null(), a)
    }

    #[inline]
    unsafe fn call2<R, A, B>(&self, send: unsafe extern "C" fn(), a: A, b: B) -> R {
        let imp: unsafe extern "C" fn(&Self, *const c_void, A, B) -> R =
            transmute(send as *const c_void);
        imp(self, std::ptr::null(), a, b)
    }

    #[inline]
    unsafe fn call3<R, A, B, C>(&self, send: unsafe extern "C" fn(), a: A, b: B, c: C) -> R {
        let imp: unsafe extern "C" fn(&Self, *const c_void, A, B, C) -> R =
            transmute(send as *const c_void);
        imp(self, std::ptr::null(), a, b, c)
    }

    #[inline]
    unsafe fn call4<R, A, B, C, D>(
        &self,
        send: unsafe extern "C" fn(),
        a: A,
        b: B,
        c: C,
        d: D,
    ) -> R {
        let imp: unsafe extern "C" fn(&Self, *const c_void, A, B, C, D) -> R =
            transmute(send as *const c_void);
        imp(self, std::ptr::null(), a, b, c, d)
    }

    #[inline]
    unsafe fn call5<R, A, B, C, D, E>(
        &self,
        send: unsafe extern "C" fn(),
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
    ) -> R {
        let imp: unsafe extern "C" fn(&Self, *const c_void, A, B, C, D, E) -> R =
            transmute(send as *const c_void);
        imp(self, std::ptr::null(), a, b, c, d, e)
    }
}

/// Use it as NSObject or id
#[repr(transparent)]
pub struct Id(Type);

impl Id {
    // #[inline]
    // pub unsafe fn retain<T: arc::Release>(id: &Id) -> arc::R<T> {
    //     transmute(objc_retain(id))
    // }

    // #[inline]
    // pub unsafe fn release(id: &mut Id) {
    //     objc_release(id)
    // }

    #[inline]
    pub unsafe fn autorelease<'ar>(id: &mut Id) -> &mut Id {
        objc_autorelease(id)
    }

    #[inline]
    pub unsafe fn retain_autoreleased<'ar>(id: Option<&Id>) -> Option<&Id> {
        objc_retainAutoreleasedReturnValue(id)
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[msg_send(isEqual:)]
    pub fn is_equal(&self, other: &Self) -> bool;

    #[inline]
    pub fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl Obj for Id {}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.description();
        f.debug_tuple("NS").field(&Cow::from(desc.deref())).finish()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Sel(NonNull<c_void>);

pub mod autorelease_pool;
pub mod ns;
pub use autorelease_pool::AutoreleasePoolPage;

pub fn autoreleasepool<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Clone, // Autoreleased doesn't implement Clone
{
    let _page = AutoreleasePoolPage::push();
    f()
}

#[macro_export]
macro_rules! msg_send {
    // TODO: we should pass name and kind
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel7($self, $sel, $a, $b, $c, $d, $e, $f, $g) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        //unsafe { $self.sel6($sel, $a, $b, $c, $d, $e, $f) }
        unsafe { $crate::objc::Obj::sel6($self, $sel, $a, $b, $c, $d, $e, $f) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel5($self, $sel, $a, $b, $c, $d, $e) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel4($self, $sel, $a, $b, $c, $d) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel3($self, $sel, $a, $b, $c) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel2($self, $sel, $a, $b) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel1($self, $sel, $a) }
    }};

    ($lib:literal, $self:ident, $sel:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static $crate::objc::Sel;
        }

        unsafe { $crate::objc::Obj::sel0($self, $sel) }
    }};
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);

    fn objc_msgSend(id: &Id, sel: &Sel, args: ...) -> *const c_void;
    fn objc_autorelease<'a>(id: &mut Id) -> &'a mut Id;
    fn objc_retainAutoreleasedReturnValue<'a>(obj: Option<&Id>) -> Option<&'a Id>;
}

#[macro_export]
macro_rules! define_obj_type {
    ($NewType:ident($BaseType:path)) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $NewType($BaseType);

        impl $crate::objc::Obj for $NewType {}

        impl std::ops::Deref for $NewType {
            type Target = $BaseType;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $NewType {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl $NewType {
            #[inline]
            pub fn retained(&self) -> $crate::arc::R<Self> {
                unsafe { $crate::objc::Obj::retain(self) }
            }
        }

        impl $crate::arc::R<$NewType> {
            #[must_use]
            pub fn autoreleased<'ar>(self) -> &'ar mut $NewType {
                unsafe {
                    let res = $crate::objc::Id::autorelease(std::mem::transmute(self));
                    return std::mem::transmute(res);
                }
            }
        }
    };
}

impl PartialEq for Id {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

#[repr(C)]
pub struct Delegate<T: Sized> {
    pub delegate: Box<T>,
    pub obj: crate::arc::R<Id>,
}

#[cfg(test)]
mod tests {

    use super::autoreleasepool;
    use crate::{cf, dispatch};
    use std;

    fn autorelease_example<'ar>() -> &'ar mut dispatch::Queue {
        let q = dispatch::Queue::new();
        q.autoreleased()
    }

    #[test]
    fn autorelease() {
        let ptr = autoreleasepool(|| {
            let q = autorelease_example();
            assert_eq!(1, q.as_type_ref().retain_count());
            unsafe { q.as_type_ref().as_type_ptr() }
        });

        let _ptr: &cf::Type = unsafe { std::mem::transmute(ptr) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ReturnedAutoReleased<T: Obj + 'static>(&'static mut T);

impl<T: Obj> ReturnedAutoReleased<T> {
    #[inline]
    pub fn retain(self) -> arc::R<T> {
        unsafe {
            asm!("mov x29, x29");
            transmute(Id::retain_autoreleased(transmute(self)))
        }
    }

    #[inline]
    pub fn option_retain(value: Option<Self>) -> Option<arc::R<T>> {
        unsafe {
            asm!("mov x29, x29");
            transmute(Id::retain_autoreleased(transmute(value)))
        }
    }
}

impl<T: Obj> Deref for ReturnedAutoReleased<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Obj> DerefMut for ReturnedAutoReleased<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub use cidre_macros::cls_msg_send;
pub use cidre_macros::msg_send;
pub use cidre_macros::rar_retain;

// global_asm!(
//     "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
//     "    .long    0",
//     "    .long    0",
//     "    .popsection",
// );