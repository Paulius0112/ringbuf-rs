use ringbuf_rs::Buffer;
use ringbuf_rs::RingBuffer;

fn main() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);

    let item = buffer.get();
    println!("Got {}", item.unwrap());
    buffer.get();
}
