extern crate num_cpus;
extern crate threadpool;

//use std::thread;
use threadpool::ThreadPool;

fn main() {
    let ncpus = num_cpus::get();
    println!("CPU数量：{}", ncpus);
    let pool = ThreadPool::new(ncpus);
    for i in 0..ncpus {
        pool.execute(move || {
            println!("线程编号:{}", i);
        });
    }
    std::thread::sleep_ms(50);
}
