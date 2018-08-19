pub struct MutSlice<T: Clone> {
    ptr: *mut T,
    length: usize,
}

impl<T> MutSlice<T>
where
    T: Clone,
{
    pub fn new(ptr: *mut T, length: usize) -> Self {
        Self { ptr, length }
    }

    pub fn from(vec: &mut Vec<T>) -> Self {
        Self::new(vec.as_mut_ptr(), vec.len())
    }

    pub fn from_slice(slice: &mut [T], length: usize) -> Self {
        Self::new(slice.as_mut_ptr(), length)
    }

    pub fn to_slice(&self) -> &mut [T] {
        use std::slice;

        unsafe { slice::from_raw_parts_mut(self.ptr, self.length) }
    }

    pub fn set(&self, index: usize, value: T) {
        self.to_slice()[index] = value;
    }

    pub fn get(&self, index: usize) -> T {
        self.to_slice()[index].clone()
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
