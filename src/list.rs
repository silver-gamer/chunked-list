use crate::chunk::Chunk;

struct ChunkedList<T, const ChunkSize: usize> {
    front: *mut Chunk<T, ChunkSize>,
    back: *mut Chunk<T, ChunkSize>,
}

