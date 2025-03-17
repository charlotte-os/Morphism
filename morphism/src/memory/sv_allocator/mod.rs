//! # Supervisor Allocator
//! This module provides a supervisor allocator for managing memory in the supervisor context.
//! The allocator uses the as of yet unstable Allocator trait to allocate memory without ever panicking or aborting.
//! Internally the allocator uses the talc crate to provide the necessary functionality however we do use a wrapper
//! around it to provide the ability to automatically grow and shrink the heap and thus free up page frames for
//! userspace whenever possible.

use alloc::Allocator;

use lazy_static::lazy_static;
use talc::{ErrOnOom, Talc};

use crate::llk::environment::boot_protocol::limine::KERNEL_ADDRESS_REQUEST;
use crate::llk::isa::current_isa::system_info::CpuInfo;
use crate::llk::isa::interface::system_info::CpuInfoIfce;

lazy_static! {
    static ref HIGHER_HALF_START: VAddr = VAddr::from(1usize << (CpuInfo::get_vaddr_sig_bits() as usize - 1usize));
    static ref SV_VADDR: VAddr = VAddr::from(KERNEL_ADDRESS_REQUEST.get_response().unwrap().virtual_base());
}

pub struct SvAllocator {
    talc: Talc<ErrOnOom>,
    full_heap_range: (VAddr, VAddr),
}

impl SvAllocator {
    pub fn new() -> Self {
        let allocator = SvAllocator {
            talc: Talc::new(ErrOnOom),
            full_heap_range: (*HIGHER_HALF_START, *SV_VADDR),
        };
        todo!("Give talc a mapped heap area to work with.");
        allocator
    }
}

impl alloc::Allocator for SvAllocator {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
        if layout.size() == 0 {
            return Err(AllocError);
        }
        self.talc.lock().allocate(layout)
    }

    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        self.talc.lock().deallocate(ptr, layout)
    }
}
