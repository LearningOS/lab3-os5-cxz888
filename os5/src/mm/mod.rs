mod frame_allocator;
mod heap_allocator;

pub mod address;
pub mod memory_set;
pub mod page_table;

use self::memory_set::KERNEL_SPACE;

pub use self::memory_set::remap_test;

pub fn init() {
    log::trace!("init heap");
    heap_allocator::init_heap();
    log::trace!("init frame_allocator");
    frame_allocator::init_frame_allocator();
    log::trace!("activate page table");
    KERNEL_SPACE.exclusive_access().activate();
}
