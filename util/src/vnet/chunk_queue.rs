#[cfg(test)]
pub mod chunk_queue_test;

use std::collections::VecDeque;

use tokio::sync::RwLock;

use super::chunk::*;

#[derive(Default)]
pub struct ChunkQueue {
    chunks: RwLock<VecDeque<Box<dyn Chunk + Send + Sync>>>,
    max_size: usize, // 0 or negative value: unlimited
}

impl ChunkQueue {
    pub fn new(max_size: usize) -> Self {
        ChunkQueue {
            chunks: RwLock::new(VecDeque::new()),
            max_size,
        }
    }

    pub async fn push(&self, c: Box<dyn Chunk + Send + Sync>) -> bool {
        let mut chunks = self.chunks.write().await;

        if self.max_size > 0 && chunks.len() >= self.max_size {
            false // dropped
        } else {
            chunks.push_back(c);
            true
        }
    }

    pub async fn pop(&self) -> Option<Box<dyn Chunk + Send + Sync>> {
        let mut chunks = self.chunks.write().await;
        chunks.pop_front()
    }

    pub async fn peek(&self) -> Option<Box<dyn Chunk + Send + Sync>> {
        let chunks = self.chunks.read().await;
        chunks.front().map(|chunk| chunk.clone_to())
    }
}
