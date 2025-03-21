use crate::llk::isa::interface::memory::address::VirtualAddress;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VAddr(usize);

impl VirtualAddress for VAddr {
    fn from_ptr<T>(ptr: *const T) -> Self {
        VAddr(ptr as usize)
    }

    fn from_mut<T>(ptr: *mut T) -> Self {
        VAddr(ptr as usize)
    }

    fn into_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    fn into_mut<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

impl From<usize> for VAddr {
    fn from(val: usize) -> Self {
        VAddr(val)
    }
}

impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.0
    }
}
