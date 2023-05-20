pub(crate) struct WrapperMut<T> {
    val: std::cell::UnsafeCell<T>,
}

impl<T> WrapperMut<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: std::cell::UnsafeCell::new(val),
        }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { self.val.get().as_mut().unwrap() }
    }
}
