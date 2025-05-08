use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_heap() {
    // Initialize heap
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
}
