use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ringbuf_rs::RingBuffer;

fn benchmark_ring_buffer_write(c: &mut Criterion) {
    c.bench_function("ring buffer write", |b| {
        b.iter(|| {
            let mut ring: RingBuffer<u32, 3> = RingBuffer::new();
            for i in 0..1_000_000 {
                ring.write(i);
                black_box(());
            }
        })
    });
}

// fn benchmark_ring_buffer_read(c: &mut Criterion) {
//     c.bench_function("ring buffer read", |b| {
//         b.iter(|| {
//             let mut ring: RingBuffer<u16, 3> = RingBuffer::new();
//             ring.write(1);
//             ring.write(2);
//             ring.write(3);
//             ring.read();
//         })
//     });
// }

criterion_group!(benches, benchmark_ring_buffer_write);
criterion_main!(benches);
