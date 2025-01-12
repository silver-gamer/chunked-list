use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::ops::{Index, IndexMut};

pub struct Chunk<T, const ChunkSize: usize> {
    next: Option<NonNull<Self>>,
    prev: Option<NonNull<Self>>,
    array: [MaybeUninit<T>; ChunkSize],
    next_index: usize,
}

impl<T, const ChunkSize: usize> Chunk<T, ChunkSize> {
    fn new() -> Self {
        Chunk {
            array: [const { MaybeUninit::uninit() }; ChunkSize],
            next: None,
            prev: None,
            next_index: 0,
        }
    }

    fn with_next(next: *mut Self) -> Self {
        Chunk {
            array: [const { MaybeUninit::uninit() }; ChunkSize],
            next: Some(next.into()),
            prev: None,
            next_index: 0,
        }
    }

    fn with_prev(prev: *mut Self) -> Self {
        Chunk {
            array: [const { MaybeUninit::uninit() }; ChunkSize],
            next: None,
            prev: Some(prev.into()),
            next_index: 0,
        }
    }

    fn len(&self) -> usize {
        self.next_index
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.next_index {
            None
        } else {
            Some(self.get_unchecked(index))
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.next_index {
            None
        } else {
            Some(self.get_unchecked_mut(index))
        }
    }

    fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.array[index].assume_init_ref() }
    }

    fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.array[index].assume_init_mut() }
    }


    fn push(&mut self, value: T) {
        self.array[self.next_index] = MaybeUninit::new(value);
        self.next_index += 1;
    }

    fn pop(&mut self) {
        self.next_index -= 1;
    }
}

impl<T, const ChunkSize: usize> Index<usize> for Chunk<T, ChunkSize> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T, const ChunkSize: usize> IndexMut<usize> for Chunk<T, ChunkSize> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut chunk = Chunk::<u32, 4>::new();
        chunk.push(1);
        chunk.push(2);

        assert_eq!(*chunk.get(0).unwrap(), 1);
    }
}
