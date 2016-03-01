use std::marker::PhantomData;

pub struct ContainerOffsets<A, T> {
    arr: *mut T,
    _phantom: PhantomData<A>,
}

impl <A, T> ContainerOffsets<A, T> where T: AsRef<[A]> + AsMut<[A]> {
    pub fn new(arr: *mut T) -> ContainerOffsets<A, T> {
        ContainerOffsets {
            arr: arr,
            _phantom: PhantomData
        }
    }

    #[inline(always)]
    pub unsafe fn get(&self, idx: usize) -> &A {
        (*self.arr).as_ref().get_unchecked(idx)
    }

    #[inline(always)]
    pub unsafe fn get_mut(&self, idx: usize) -> &mut A {
        (*self.arr).as_mut().get_unchecked_mut(idx)
    }
}
