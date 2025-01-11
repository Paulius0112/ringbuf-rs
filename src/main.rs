
use std::{fmt::Display, mem::{self, MaybeUninit}, usize};



pub struct RingBuffer<T: Copy, const S: usize> {
    buffer: [MaybeUninit<T>; S],
    len: usize,
    writeptr: usize,
    readptr: usize, 
}

impl<T, const S: usize> RingBuffer<T, S> 
where T: Copy + Display
{
    pub fn new() -> RingBuffer<T, S> {
        let mut buffer: [MaybeUninit<T>; S] = unsafe {MaybeUninit::uninit().assume_init()};
        
        RingBuffer { buffer, len: 0, writeptr: 0, readptr: 0}
    }

    pub fn write(&mut self, elem: T) {
        // Check if ring is full
        if self.is_full() {
            println!("Buffer is full!");
            return

            // TODO: Overwrite first element
        }

        let mem_addr = &self.buffer[self.writeptr] as *const _;

        self.buffer[self.writeptr].write(elem);
        println!("Writing data to: {:p}, at index: {}", mem_addr, self.writeptr);

        self.writeptr += 1;
        self.writeptr = self.writeptr % S;
        self.len += 1;
    }

    pub fn read(&mut self) {

        if self.is_empty() {
            println!("Buffer is emtpy! Cannot read");
            return;
        }
        let mem_addr = &self.buffer[self.readptr] as *const _;

        println!("Reading data from: {:p}, at index: {}", mem_addr, self.readptr);
        let elem = unsafe { self.buffer[self.readptr].assume_init_read()};
        println!("Element read: {}", elem);

        self.readptr += 1;
        self.readptr = self.readptr % S;
        self.len -= 1;
    } 

    pub fn is_full(&self) -> bool {
        self.len == S
    }

    pub fn is_empty(&self) -> bool {
        self.writeptr == self.readptr
    }
} 

fn main() {
    println!("Hello, world!");

    let mut ring: RingBuffer<u16, 3> = RingBuffer::new();

    ring.write(5);
    ring.read();
    ring.write(6);
    ring.read();
    ring.write(7);
    ring.read();
    ring.write(58);
    ring.read();
    ring.read();
    //ring.write(6);

    // ring.read();
    // ring.read();
    


    


}
