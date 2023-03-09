fn mem_copy<T>(obj: &T) -> T {
    let len = std::mem::size_of::<T>();

    let src = obj as *const T as *const u8;

    unsafe {
        let dst = std::alloc::alloc(std::alloc::Layout::new::<T>());
        std::ptr::copy_nonoverlapping(src, dst, len);
        std::ptr::read(dst as *const T)
    }
}