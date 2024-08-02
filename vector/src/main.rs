use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{mem, ptr};

#[derive(Debug)]
struct MRawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> MRawVec<T> {
    fn new() -> Self {
        let cap = (mem::size_of::<T>() == 0) as usize;
        Self {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    fn grow(&mut self) {
        // ??
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            // Layout::array<T>(n) may cause a error
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = match self.cap {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for MRawVec<T> {
    fn drop(&mut self) {
        // zero sized type or zero capcity
        if self.cap == 0 || mem::size_of::<T>() == 0 {
            return;
        }

        let layout = Layout::array::<T>(self.cap).unwrap();
        unsafe {
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[derive(Debug)]
pub struct MVec<T> {
    buf: MRawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for MVec<T> {}

unsafe impl<T: Sync> Sync for MVec<T> {}

impl<T> MVec<T> {
    pub fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        // ??
        assert!(mem::size_of::<T>() != 0);
        MVec {
            buf: MRawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
            // mem::drop()
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index >= 0 && index < self.len, "index out of bounds");

        unsafe {
            self.len -= 1;
            let removed_elem = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );

            removed_elem
        }
    }
}

impl<T> MVec<T> {
    fn grow(&mut self) {
        self.buf.grow();
    }
}

//
impl<T> Drop for MVec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr(), self.len));
            // for i in 0..self.len {
            //     unsafe {

            //         ptr::drop_in_place(self.ptr().add(i));
            //     }
            // }
        }
        // MRawVec handles deallocation
    }
}

impl<T> Deref for MVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for MVec<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe { std::slice::from_raw_parts_mut(self.buf.ptr.as_mut(), self.len()) }
    }
}

struct MRawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> MRawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        MRawValIter {
            start: slice.as_ptr(),
            // ????
            end: if mem::size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> Iterator for MRawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for MRawValIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

pub struct MIntoIter<T> {
    _buf: MRawVec<T>,
    iter: MRawValIter<T>,
}

impl<T> Iterator for MIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> IntoIterator for MVec<T> {
    type Item = T;
    type IntoIter = MIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = MRawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);

            MIntoIter {
                iter: iter,
                _buf: buf,
            }
        }
    }
}

impl<T> DoubleEndedIterator for MIntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for MIntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct MDrain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: MRawValIter<T>,
}

impl<'a, T> Iterator for MDrain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for MDrain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for MDrain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> MVec<T> {
    pub fn drain(&mut self) -> MDrain<T> {
        unsafe {
            let iter = MRawValIter::new(&self);
            self.len = 0;
            MDrain {
                iter: iter,
                vec: PhantomData,
            }
        }
    }
}

fn main() {
    println!("{}", std::mem::size_of::<MRawVec<i32>>());
}
