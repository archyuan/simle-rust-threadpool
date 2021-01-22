
mod threadpool;

fn handle(id:usize) {
    
    println!("id is {}",id);
}


fn main() {
    
    let thread_size = 5;
    let pool = threadpool::ThreadPool::new(thread_size);
    
    for i in 0..thread_size {
        pool.execute(move || {
           handle(i);
        });
    }

}
