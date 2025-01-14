// TODO:
// - [ ] Pure fn block is global _NSConcreteGlobalBlock
// - [ ] Pure fn block with fields - _NSConcreteMallocBlock
// - [x] NoEscaping block - _NSConcreteStackBlock

// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt
// https://developer.apple.com/documentation/swift/calling-objective-c-apis-asynchronously
// https://github.com/apple/swift-corelibs-foundation/blob/main/Sources/BlocksRuntime/runtime.c

use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::{self, transmute},
    ops,
};

use crate::{arc, define_opts, ns, objc::Class};

#[repr(transparent)]
pub struct Block<F>(c_void, std::marker::PhantomData<F>);

impl<F> Block<F> {
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        unsafe { transmute(self) }
    }
}

pub fn fn0<R>(f: extern "C" fn(*const c_void) -> R) -> bl<extern "C" fn(*const c_void) -> R> {
    bl::with(f)
}

pub fn fn1<A, R>(
    f: extern "C" fn(*const c_void, a: A) -> R,
) -> bl<extern "C" fn(*const c_void, a: A) -> R> {
    bl::with(f)
}

pub fn fn2<A, B, R>(
    f: extern "C" fn(*const c_void, a: A, b: B) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B) -> R> {
    bl::with(f)
}

pub fn fn3<A, B, C, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C) -> R> {
    bl::with(f)
}

pub fn fn4<A, B, C, D, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R> {
    bl::with(f)
}

pub fn fn5<A, B, C, D, E, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R> {
    bl::with(f)
}

pub fn once0<'a, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce() -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke0 as _, f)
}

pub fn once1<'a, A, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce(A) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke1 as _, f)
}

pub fn once2<'a, A, B, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce(A, B) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke2 as _, f)
}

pub fn once3<'a, A, B, C, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce(A, B, C) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke3 as _, f)
}

pub fn once4<'a, A, B, C, D, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce(A, B, C, D) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke4 as _, f)
}

pub fn once5<'a, A, B, C, D, E, R, F: 'a>(f: F) -> BlOnce<'a, F>
where
    F: FnOnce(A, B, C, D, E) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke5 as _, f)
}

pub fn mut0<'a, R, F: 'a>(f: F) -> BlMut<'a, F>
where
    F: FnMut() -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke0 as _, f)
}

pub fn mut1<'a, A, R, F: 'a>(f: F) -> BlMut<'a, F>
where
    F: FnMut(A) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke1 as _, f)
}

pub fn mut2<'a, A, B, R, F: 'a>(f: F) -> BlMut<'a, F>
where
    F: FnMut(A, B) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke2 as _, f)
}

pub fn mut3<'a, A, B, C, R, F: 'a>(f: F) -> BlMut<'a, F>
where
    F: FnMut(A, B, C) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke3 as _, f)
}

pub fn mut4<'a, A, B, C, D, R, F>(f: F) -> BlMut<'a, F>
where
    F: FnMut(A, B, C, D) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke4 as _, f)
}

pub fn mut5<'a, A, B, C, D, E, R, F: 'a>(f: F) -> BlMut<'a, F>
where
    F: FnMut(A, B, C, D, E) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke5 as _, f)
}

pub fn no_esc0<R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut() -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke0 as _, f)
}

pub fn no_esc1<A, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke1 as _, f)
}

pub fn no_esc2<A, B, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke2 as _, f)
}

pub fn no_esc3<A, B, C, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke3 as _, f)
}

pub fn no_esc4<A, B, C, D, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C, D) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke4 as _, f)
}

pub fn no_esc5<A, B, C, D, E, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C, D, E) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke5 as _, f)
}

define_opts!(pub Flags(i32));

impl Flags {
    pub const NONE: Self = Self(0);

    // runtime
    pub const DEALLOCATING: Self = Self(1);

    // runtime
    pub const REFCOUNT_MASK: Self = Self(0xfffei32);

    // compiler
    // Set to true on blocks that have captures (and thus are not true
    // global blocks) but are known not to escape for various other
    // reasons. For backward compatibility with old runtimes, whenever
    // IS_NOESCAPE is set, IS_GLOBAL is set too. Copying a
    // non-escaping block returns the original block and releasing such a
    // block is a no-op, which is exactly how global blocks are handled.
    pub const IS_NOESCAPE: Self = Self(1 << 23);

    // runtime
    pub const NEEDS_FREE: Self = Self(1 << 24);
    // compiler
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    pub const HAS_CTOR: Self = Self(1 << 26);
    pub const IS_GC: Self = Self(1 << 27);
    pub const IS_GLOBAL: Self = Self(1 << 28);
    pub const USE_STRET: Self = Self(1 << 29);
    pub const HAS_SIGNATURE: Self = Self(1 << 30);
    pub const HAS_EXTENDED_LAYOUT: Self = Self(1 << 31);

    const RETAINED_NEEDS_FREE: Self = Self(2 | Self::NEEDS_FREE.0);
    const RETAINED_NEEDS_DROP: Self = Self(2 | Self::NEEDS_FREE.0 | Self::HAS_COPY_DISPOSE.0);
}

#[repr(C)]
pub struct Desc1 {
    reserved: usize,
    size: usize,
}

#[repr(C)]
pub struct Desc2<T: Sized> {
    descriptor1: Desc1,
    copy: extern "C" fn(dest: &mut T, src: &mut T),
    dispose: extern "C" fn(literal: &mut T),
}

// for completion handlers
#[repr(C)]
struct Layout2Once<'a, F: Sized + 'a> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'a Desc2<Self>,
    closure: Option<F>,
}

#[repr(C)]
struct Layout2Mut<'a, F: 'a + Sized> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'a Desc2<Self>,
    closure: mem::ManuallyDrop<F>,
}

#[repr(C)]
pub struct Layout1Mut<'a, F> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'a Desc1,
    closure: &'a mut F,
}

#[repr(C)]
pub struct LayoutArg {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
}

#[repr(transparent)]
pub struct Arg<F>(LayoutArg, PhantomData<F>);

impl<R> Arg<fn() -> R> {
    pub fn call(&mut self) -> R {
        let f: extern "C" fn(literal: &mut Self) -> R =
            unsafe { std::mem::transmute(self.0.invoke) };
        f(self)
    }
}

impl<A, R> Arg<fn(a: A) -> R> {
    pub fn call(&mut self, a: A) -> R {
        let f: extern "C" fn(literal: &mut Self, a: A) -> R =
            unsafe { std::mem::transmute(self.0.invoke) };
        f(self, a)
    }
}

impl<A, B, R> Arg<fn(a: A, b: B) -> R> {
    pub fn call(&mut self, a: A, b: B) -> R {
        let f: extern "C" fn(literal: &mut Self, a: A, b: B) -> R =
            unsafe { std::mem::transmute(self.0.invoke) };
        f(self, a, b)
    }
}

pub type NoEscBlMut<'a, F> = Layout1Mut<'a, F>;

impl<'a, F> std::ops::Deref for Layout1Mut<'a, F> {
    type Target = Block<F>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, F> std::ops::DerefMut for Layout1Mut<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, F> Layout1Mut<'a, F> {
    const DESCRIPTOR_1: Desc1 = Desc1 {
        reserved: 0,
        size: std::mem::size_of::<&'static Class<ns::Id>>()
            + std::mem::size_of::<Flags>()
            + std::mem::size_of::<i32>()
            + std::mem::size_of::<*const c_void>()
            + std::mem::size_of::<&'static Desc1>()
            + std::mem::size_of::<&'static c_void>(), // emulating &mut F
    };

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.closure)()
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.closure)(a)
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.closure)(a, b)
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.closure)(a, b, c)
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnMut(A, B, C, D) -> R,
    {
        (self.closure)(a, b, c, d)
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (self.closure)(a, b, c, d, e)
    }

    fn new(invoke: *const c_void, f: &'a mut F) -> Layout1Mut<'a, F> {
        Self {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Default::default(),
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_1,
            closure: f,
        }
    }
}

/// block with static fn
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct bl<F: Sized> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: F,
    descriptor: &'static Desc1,
}

impl<F: Sized> bl<F> {
    #[inline]
    pub fn escape(&mut self) -> &'static mut Block<F> {
        unsafe { mem::transmute(self) }
    }
}

#[repr(transparent)]
pub struct BlOnce<'a, F: 'a>(&'a mut Block<F>);

impl<F> BlOnce<'static, F> {
    #[inline]
    pub fn escape(self) -> &'static mut Block<F> {
        unsafe { transmute(self) }
    }
}

#[repr(transparent)]
pub struct BlMut<'a, F: 'a>(&'a mut Block<F>);

impl<F> BlMut<'static, F> {
    #[inline]
    pub fn escape(&mut self) -> &'static mut Block<F> {
        let ptr = self.0 as *mut Block<F>;
        unsafe { &mut *ptr }
    }
}

impl<'a, F> Drop for BlOnce<'a, F> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            _Block_release(self.0 as *mut _ as *const _);
        };
    }
}

impl<'a, F> Drop for BlMut<'a, F> {
    #[inline]
    fn drop(&mut self) {
        unsafe { _Block_release(self.0 as *mut _ as *const _) };
    }
}

impl<F> Clone for BlMut<'static, F> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ptr = _Block_copy(self as *const Self as *const _);
            Self(&mut *(ptr as *mut Self))
        }
    }
}

impl<'a, F> ops::Deref for BlOnce<'a, F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, F> ops::DerefMut for BlOnce<'a, F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, F> ops::Deref for BlMut<'a, F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, F> ops::DerefMut for BlMut<'a, F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<F> ops::Deref for bl<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<F> ops::DerefMut for bl<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<'a, F: 'a + Sized> Layout2Once<'a, F> {
    const DESCRIPTOR_2: Desc2<Self> = Desc2 {
        descriptor1: Desc1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        debug_assert!(mem::needs_drop::<F>());
        block.closure.take();
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnOnce() -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)()
        } else {
            panic!()
        }
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnOnce(A) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnOnce(A, B) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnOnce(A, B, C) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnOnce(A, B, C, D) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnOnce(A, B, C, D, E) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d, e)
        } else {
            panic!()
        }
    }

    fn new(invoke: *const c_void, f: F) -> BlOnce<'a, F> {
        let flags = if mem::needs_drop::<F>() {
            Flags::RETAINED_NEEDS_DROP
        } else {
            Flags::RETAINED_NEEDS_FREE
        };

        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: Some(f),
        });
        let block_ref = Box::leak(block);
        BlOnce(unsafe { mem::transmute(block_ref) })
    }
}

impl<'a, F: 'a + Sized> Layout2Mut<'a, F> {
    const DESCRIPTOR_2: Desc2<Self> = Desc2 {
        descriptor1: Desc1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        debug_assert!(mem::needs_drop::<F>());
        unsafe {
            mem::ManuallyDrop::drop(&mut block.closure);
        }
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.closure)()
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.closure)(a)
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.closure)(a, b)
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.closure)(a, b, c)
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnMut(A, B, C, D) -> R,
    {
        (self.closure)(a, b, c, d)
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (self.closure)(a, b, c, d, e)
    }

    fn new(invoke: *const c_void, f: F) -> BlMut<'a, F> {
        let flags = if mem::needs_drop::<F>() {
            Flags::RETAINED_NEEDS_DROP
        } else {
            Flags::RETAINED_NEEDS_FREE
        };

        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: mem::ManuallyDrop::new(f),
        });
        let block_ref = Box::leak(block);
        BlMut(unsafe { mem::transmute(block_ref) })
    }
}

impl<F> bl<F> {
    const DESCRIPTOR: Desc1 = Desc1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    pub fn with(invoke: F) -> bl<F> {
        bl {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR,
        }
    }
}

impl<R> bl<extern "C" fn(b: *const c_void) -> R> {
    pub fn call(&self) -> R {
        (self.invoke)(self as *const Self as _)
    }
}

impl<A, R> bl<extern "C" fn(b: *const c_void, a: A) -> R> {
    pub fn call(&self, a: A) -> R {
        (self.invoke)(self as *const Self as _, a)
    }
}

impl<A, B, R> bl<extern "C" fn(b: *const c_void, a: A, b: B) -> R> {
    pub fn call(&self, a: A, b: B) -> R {
        (self.invoke)(self as *const Self as _, a, b)
    }
}

impl<A, B, C, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C) -> R> {
    pub fn call(&self, a: A, b: B, c: C) -> R {
        (self.invoke)(self as *const Self as _, a, b, c)
    }
}

impl<A, B, C, D, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C, d: D) -> R> {
    pub fn call(&self, a: A, b: B, c: C, d: D) -> R {
        (self.invoke)(self as *const Self as _, a, b, c, d)
    }
}

impl<A, B, C, D, E, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C, d: D, e: E) -> R> {
    pub fn call(&self, a: A, b: B, c: C, d: D, e: E) -> R {
        (self.invoke)(self as *const Self as _, a, b, c, d, e)
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    static _NSConcreteGlobalBlock: Class<ns::Id>;
    static _NSConcreteStackBlock: Class<ns::Id>;
    static _NSConcreteMallocBlock: Class<ns::Id>;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::blocks;

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped foo");
        }
    }

    #[test]
    fn simple_block() {
        let foo = Foo;
        let _rc = Rc::new(10);
        let mut b = blocks::mut0(move || println!("nice {foo:?}"));

        let q = crate::dispatch::Queue::new();
        q.async_b(b.escape());
        q.async_b(b.escape());
        q.async_mut(|| println!("nice"));
        q.sync_mut(|| println!("fuck"));
        // q.async_once(move || println!("nice {rc:?}"));

        println!("finished");
    }
}

#[cfg(feature = "async")]
use parking_lot::Mutex;
#[cfg(feature = "async")]
use std::sync::Arc;

#[cfg(feature = "async")]
struct Shared<T> {
    ready: Option<T>,
    pending: Option<std::task::Waker>,
}

#[cfg(feature = "async")]
impl<T> Shared<T> {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            ready: None,
            pending: None,
        }))
    }

    pub fn ready(&mut self, result: T) {
        self.ready = Some(result);

        if let Some(waker) = self.pending.take() {
            waker.wake();
        }
    }
}

#[cfg(feature = "async")]
pub struct Completion<R>(Arc<Mutex<Shared<R>>>);

#[cfg(feature = "async")]
impl<T> std::future::Future for Completion<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut lock = self.0.lock();

        if let Some(r) = lock.ready.take() {
            std::task::Poll::Ready(r)
        } else {
            lock.pending = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

#[cfg(feature = "async")]
pub fn comp0() -> (Completion<()>, BlOnce<'static, impl FnOnce()>) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        once0(move || shared.lock().ready(())),
    )
}

#[cfg(feature = "async")]
pub fn comp1<R: Send + 'static>() -> (Completion<R>, BlOnce<'static, impl FnOnce(R)>) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        once1(move |v: R| shared.lock().ready(v)),
    )
}

#[cfg(feature = "async")]
pub fn ok() -> (
    Completion<Result<(), arc::R<ns::Error>>>,
    BlOnce<'static, impl FnOnce(Option<&'static ns::Error>)>,
) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        once1(move |error: Option<&'static ns::Error>| {
            shared.lock().ready(match error {
                None => Ok(()),
                Some(err) => Err(err.retained()),
            });
        }),
    )
}

#[cfg(feature = "async")]
pub fn result<T: arc::Retain>() -> (
    Completion<Result<arc::R<T>, arc::R<ns::Error>>>,
    BlOnce<'static, impl FnOnce(Option<&'static T>, Option<&'static ns::Error>)>,
) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        once2(
            move |value: Option<&'static T>, error: Option<&'static ns::Error>| {
                let res = match error {
                    None => Ok(unsafe { value.unwrap_unchecked().retained() }),
                    Some(err) => Err(err.retained()),
                };

                shared.lock().ready(res);
            },
        ),
    )
}
