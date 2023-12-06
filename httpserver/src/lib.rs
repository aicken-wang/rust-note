use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Creates a new thread pool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the pool size is zero.
    pub fn new(size:usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            //创建工作任务
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // 保存到线程池
        ThreadPool { workers, sender}
    }
    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
struct Worker {
    id :usize,
    // 无返回值()
    thread:thread::JoinHandle<()>,
}
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self:Box<F>) {
        (*self)()
    }
}
// 任务类型Job;
type Job = Box<dyn FnBox + Send + 'static>;
// 工作线程
impl Worker {
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
      
        });
        Worker{id, thread}
    }
}