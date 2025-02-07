use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use threadpool::ThreadPool;

fn main() {
    // 1. 创建基本线程
    let handle = thread::spawn(|| {
        println!("你好，我是一个新线程！");
        thread::sleep(Duration::from_millis(1000));
        println!("新线程执行完毕！");
    });

    println!("主线程在继续执行");

    // 等待新线程完成
    handle.join().unwrap();

    // 2. 使用 move 闭包传递数据
    let data = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("在新线程中处理数据: {:?}", data);
    });
    handle.join().unwrap();

    // 3. 使用通道进行线程间通信
    let (tx, rx) = mpsc::channel();
    
    let sender_handle = thread::spawn(move || {
        let messages = vec!["你好", "世界", "从", "线程"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let receiver_handle = thread::spawn(move || {
        for received in rx {
            println!("收到消息: {}", received);
        }
    });

    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();

    // 4. 使用线程池
    println!("\n=== 线程池示例 ===");
    let pool = ThreadPool::new(3); // 创建一个有3个线程的线程池
    
    for i in 0..5 {
        pool.execute(move || {
            println!("线程池中的任务 {} 开始执行", i);
            thread::sleep(Duration::from_millis(500));
            println!("线程池中的任务 {} 执行完成", i);
        });
    }
    
    pool.join(); // 等待所有任务完成

    // 5. 线程同步 (使用 Mutex)
    println!("\n=== 线程同步示例 ===");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock();
            *num += 1;
            println!("线程 {} 将计数器增加到: {}", i, *num);
            thread::sleep(Duration::from_millis(300));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数器的值: {}", *counter.lock());

    // 6. 线程锁示例 (使用 RwLock)
    println!("\n=== 线程锁示例 ===");
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // 创建多个读取线程
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let numbers = data_clone.read();
            println!("读取线程 {} 读取到数据: {:?}", i, *numbers);
            thread::sleep(Duration::from_millis(200));
        });
        handles.push(handle);
    }

    // 创建写入线程
    let data_clone = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut numbers = data_clone.write();
        numbers.push(6);
        println!("写入线程添加了新数据，现在数据是: {:?}", *numbers);
    });
    handles.push(write_handle);

    // 再创建一些读取线程
    for i in 3..6 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(300));
            let numbers = data_clone.read();
            println!("读取线程 {} 读取到数据: {:?}", i, *numbers);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
} 