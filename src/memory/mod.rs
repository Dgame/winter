pub struct Slice<T> {
    ptr: *const T,
    len: usize,
}

pub struct MutSlice<T> {
    ptr: *mut T,
    len: usize,
}

pub trait Lenght {
    fn len(&self) -> usize;
}

impl<T> Lenght for Slice<T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Lenght for MutSlice<T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub trait SetAt<T> {
    fn set_at(&mut self, index: usize, val: T);
}

impl<T> SetAt<T> for MutSlice<T> {
    fn set_at(&mut self, index: usize, value: T) {
        let slice: &mut [T] = self.as_mut();
        slice[index] = value;
    }
}

pub trait GetAt<T: Clone> {
    fn get_at(&self, index: usize) -> T;
}

impl<T> GetAt<T> for Slice<T>
where
    T: Clone,
{
    fn get_at(&self, index: usize) -> T {
        let slice: &[T] = self.as_ref();

        slice[index].clone()
    }
}

impl<T> GetAt<T> for MutSlice<T>
where
    T: Clone,
{
    fn get_at(&self, index: usize) -> T {
        let slice: &[T] = self.as_ref();

        slice[index].clone()
    }
}

impl<'a, T> From<&'a Vec<T>> for Slice<T> {
    fn from(vec: &'a Vec<T>) -> Self {
        Self {
            ptr: vec.as_ptr(),
            len: vec.len(),
        }
    }
}

impl<'a, T> From<&'a mut Vec<T>> for MutSlice<T> {
    fn from(vec: &'a mut Vec<T>) -> Self {
        Self {
            ptr: vec.as_mut_ptr(),
            len: vec.len(),
        }
    }
}

impl<'a, T> From<&'a [T]> for Slice<T> {
    fn from(slice: &'a [T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
        }
    }
}

impl<'a, T> From<&'a mut [T]> for MutSlice<T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
        }
    }
}

impl<'a, T> Into<&'a [T]> for Slice<T> {
    fn into(self) -> &'a [T] {
        use std::slice;

        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> Into<&'a [T]> for MutSlice<T> {
    fn into(self) -> &'a [T] {
        use std::slice;

        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> Into<&'a mut [T]> for MutSlice<T> {
    fn into(self) -> &'a mut [T] {
        use std::slice;

        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> AsRef<[T]> for Slice<T> {
    fn as_ref(&self) -> &[T] {
        use std::slice;

        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> AsRef<[T]> for MutSlice<T> {
    fn as_ref(&self) -> &[T] {
        use std::slice;

        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> AsMut<[T]> for MutSlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        use std::slice;

        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
