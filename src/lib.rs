
use std::{fmt::{Debug, Display}, mem::{MaybeUninit}, usize};

const fn mask_modulo(index: usize, size: usize) -> usize {
    index % size
}

pub struct RingBuffer<T: Copy, const S: usize> {
    buffer: [MaybeUninit<T>; S],
    len: usize,
    writeptr: usize,
    readptr: usize, 
}

impl<T, const S: usize> Debug for RingBuffer<T, S>
where
    T: Copy
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RingBuffer {{ len: {}, writeptr: {}, readptr: {} }}", self.len, self.writeptr, self.readptr)
    }
}

impl<T, const S: usize> RingBuffer<T, S> 
where T: Copy + Display
{
    pub fn new() -> RingBuffer<T, S> {
        let buffer: [MaybeUninit<T>; S] = unsafe {MaybeUninit::uninit().assume_init()};
        RingBuffer { buffer, len: 0, writeptr: 0, readptr: 0}
    }

    pub fn write(&mut self, elem: T) {
        //let mem_addr = &self.buffer[self.writeptr] as *const _;

        if self.is_full() {
            self.readptr = mask_modulo(self.readptr, S)
        } else {
            self.len += 1;
        }

        self.buffer[self.writeptr].write(elem);
        //println!("Writing data to: {:p}, at index: {}", mem_addr, self.writeptr);

        self.writeptr += 1;
        self.writeptr = mask_modulo(self.writeptr, S);
    }

    pub fn read(&mut self) {

        if self.is_empty() {
            println!("Buffer is emtpy! Cannot read");
            return;
        }
        let mem_addr = &self.buffer[self.readptr] as *const _;

        //println!("Reading data from: {:p}, at index: {}", mem_addr, self.readptr);
        let elem = unsafe { self.buffer[self.readptr].assume_init_read()};
        //println!("Element read: {}", elem);

        self.readptr += 1;
        self.readptr = mask_modulo(self.readptr, S);
        self.len -= 1;
    } 

    pub fn is_full(&self) -> bool {
        self.len == S
    }

    pub fn is_empty(&self) -> bool {
        self.writeptr == self.readptr && self.len == 0
    }
} 


