
use ringbuf_rs::RingBuffer;

fn main() {
    let mut buffer: RingBuffer<u32, 10> = RingBuffer::new();

    buffer.write(1);
    buffer.write(1);
    buffer.write(1);
    buffer.write(1);

    buffer.read();
    buffer.read();
}