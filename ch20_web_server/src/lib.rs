use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// 创建一个线程池结构，并让ThreadPool充当通道的发送端
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    // new方法用于配置线程池中的线程数量
    pub fn new(size: usize) -> ThreadPool {
        // 确保size大于0，否则panic
        assert!(size > 0);

        // 新建一个通道
        let (sender, receiver) = mpsc::channel();
        // 实现多个worker共享接收端
        let receiver = Arc::new(Mutex::new(receiver));
        // 设置Worker的容量
        let mut workers = Vec::with_capacity(size);

        // 创建线程并存储在vector中
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // 执行线程中的代码
    pub fn execute<F>(&self, f:F)
        where 
            F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/*
要实现的行为是创建线程并稍后发送代码
- 每一个 Worker 储存一个单独的 JoinHandle<()> 实例
- 在 Worker上实现一个方法来获取需要允许代码的闭包并将其发送给已经运行的线程执行
- 赋予每一个 worker id，这样就可以在日志和调试中区别线程池中的不同 worker。
*/
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}