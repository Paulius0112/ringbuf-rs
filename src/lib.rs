use std::{
    fmt::{Debug, Display},
    mem::MaybeUninit,
    usize,
};

const fn mask_modulo(index: usize, size: usize) -> usize {
    index % size
}

// Other buffer implementations (stack/heap) should reuse this trait
// for standard API
pub trait Buffer<T>
where
    T: Copy,
{
    fn push(&mut self, elem: T) -> Option<T>;
    fn get(&mut self) -> Option<T>;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
}

pub struct RingBuffer<T: Copy, const S: usize> {
    buffer: [MaybeUninit<T>; S],
    len: usize,
    writeptr: usize,
    readptr: usize,
}

impl<T, const S: usize> Buffer<T> for RingBuffer<T, S>
where
    T: Copy,
{
    fn is_full(&self) -> bool {
        self.len == S
    }

    fn is_empty(&self) -> bool {
        self.writeptr == self.readptr && self.len == 0
    }

    fn push(&mut self, elem: T) -> Option<T> {
        let overwritten = if self.is_full() {
            Some(unsafe { self.buffer[self.readptr].assume_init_read() })
        } else {
            self.len += 1;
            None
        };

        self.buffer[self.writeptr].write(elem);

        self.writeptr = mask_modulo(self.writeptr + 1, S);

        if overwritten.is_some() {
            self.readptr = mask_modulo(self.readptr + 1, S);
        }

        overwritten
    }

    fn get(&mut self) -> Option<T> {
        if self.is_empty() {
            println!("Buffer is emtpy! Cannot read");
            return None;
        }

        let elem = unsafe { self.buffer[self.readptr].assume_init_read() };
        self.readptr = mask_modulo(self.readptr + 1, S);
        self.len -= 1;

        Some(elem)
    }
}

impl<T, const S: usize> Debug for RingBuffer<T, S>
where
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RingBuffer {{ len: {}, writeptr: {}, readptr: {} }}",
            self.len, self.writeptr, self.readptr
        )
    }
}

impl<T, const S: usize> RingBuffer<T, S>
where
    T: Copy + Display,
{
    pub fn new() -> RingBuffer<T, S> {
        let buffer: [MaybeUninit<T>; S] = unsafe { MaybeUninit::uninit().assume_init() };
        RingBuffer {
            buffer,
            len: 0,
            writeptr: 0,
            readptr: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Buffer, RingBuffer};

    #[test]
    fn test_full_and_empty() {
        let mut buffer: RingBuffer<u32, 10> = RingBuffer::new();

        for i in 1..12 {
            buffer.push(i);
        }

        assert!(buffer.is_full());

        for _ in 1..11 {
            buffer.get();
        }

        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buffer: RingBuffer<u32, 5> = RingBuffer::new();

        for i in 1..7 {
            buffer.push(i);
        }

        assert_eq!(
            buffer.get(),
            Some(2),
            "Buffer's latest read ptr should be at 2"
        )
    }
}
