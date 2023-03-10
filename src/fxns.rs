use std::{ops::{Deref, DerefMut}, marker::PhantomData};

pub unsafe fn mem_copy<T>(obj: &T) -> T {
    let len = std::mem::size_of::<T>();

    let src = obj as *const T as *const u8;

    unsafe {
        let dst = std::alloc::alloc(std::alloc::Layout::new::<T>());
        std::ptr::copy_nonoverlapping(src, dst, len);
        std::ptr::read(dst as *const T)
    }
}

pub struct HeapRef<T> {
    item: *mut u8,
    phanthom: PhantomData<T>,
}

impl<T> HeapRef<T> {
    pub fn new(self) {

    }
}

impl<T> Deref for HeapRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for HeapRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl<T> Drop for HeapRef<T> {
    fn drop(&mut self) {
        todo!()
    }
}
