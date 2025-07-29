pub use quickvariant_macros as macros;

use std::alloc::Layout;
use std::any::TypeId;
use std::collections::HashSet;

/// ### `InvalidParameters`
/// If the layout created from the specified type is invalid, this error will be returned.
/// 
/// ### `AllocationFailed`
/// This error is returned when memory allocation fails. Normally, this error should not occur.
///
/// ### `DisallowedType`
/// This error is returned when attempting to set a type that was not used to create the Variant.
pub enum ErrorKind {
    InvalidParameters,
    AllocationFailed,
    DisallowedType
}

/// This struct provides a C++-like variant. If you create an instance without using the `make_variant` macro, calling `set` may cause undefined behavior. Also, only types with a `'static` lifetime are allowed.
pub struct Variant {
    ptr: *mut u8,
    layout: Layout,
    id: Option<TypeId>,
    allowed: HashSet<TypeId>,
    drop_fn: Option<unsafe fn(*mut u8)>
}

impl Variant {
    unsafe fn _drop<T>(ptr: *mut u8) {
        unsafe {
            std::ptr::drop_in_place(ptr as *mut T);
        }
    }

    #[doc(hidden)]
    pub fn __new(size: usize, align: usize, allowed: HashSet<TypeId>) -> Result<Self, ErrorKind> {
        let layout = match Layout::from_size_align(size, align) {
            Ok(layout) => layout,
            Err(_) => return Err(ErrorKind::InvalidParameters)
        };
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(ErrorKind::AllocationFailed);
        }
        Ok(Self {
            ptr,
            layout,
            id: None,
            allowed,
            drop_fn: None
        })
    }

    /// This function is unsafe. The argument value accepts a value of type T. T must have a 'static lifetime and must be one of the types specified when creating the variant with the make_variant macro.
    pub unsafe fn set<T>(&mut self, value: T) -> Result<(), ErrorKind> where T: 'static {
        let id = TypeId::of::<T>();
        if !self.allowed.contains(&id) {
            return Err(ErrorKind::DisallowedType);
        }
        if let Some(drop) = self.drop_fn {
            unsafe {
                drop(self.ptr);
            }
        }
        unsafe {
            std::ptr::write(self.ptr as *mut T, value);
        }
        self.id = Some(id);
        self.drop_fn = Some(Self::_drop::<T>);
        Ok(())
    }

    /// Resets the variant, returning it to its initial state.
    pub fn reset(&mut self) {
        if let Some(drop) = self.drop_fn {
            unsafe {
                drop(self.ptr);
            }
        }
        self.id = None;
        self.drop_fn = None;
    }

    /// Returns a bool indicating whether the variant currently holds the type T. Returns false if it is in the initial state or holds a different type.
    pub fn holds<T>(&self) -> bool where T: 'static {
        if let Some(id) = self.id {
            if id == TypeId::of::<T>() {
                return true;
            }
        }
        false
    }

    /// Returns the internal value as an Option<&T>. Returns None if the stored type is not T.
    pub fn get<T>(&self) -> Option<&T> where T: 'static {
        if let Some(id) = self.id {
            if id == TypeId::of::<T>() {
                return unsafe { Some(&*(self.ptr as *mut T)) }
            }
        }
        None
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if let Some(drop) = self.drop_fn {
                unsafe {
                    drop(self.ptr);
                }
            }
            unsafe {
                std::alloc::dealloc(self.ptr, self.layout);
            }
        }
    }
}