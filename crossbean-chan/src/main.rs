use crossbeam;

// 用于send,rcv
use crossbeam_channel;
use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let arr = &[
        12, 890, 98, 67, 37, 986, 908, 896, 123, 23, 89, 456, 2345, 909, 1123, 678,
    ];
    let max = find_max(arr);
    println!("max:{:?}", max);

    // 创建并发的数据管道
    message_send_consume();

    // 在两个线程间传递数据
    println!("exec sender received data.");
    sender_rcv();
}

const THRESHOLD: usize = 2;
fn find_max(arr: &[i32]) -> Option<i32> {
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        Some(max_l.max(max_r))
    })
    .unwrap()
}

// 一个生产者，2个消费者处理消息
// 创建并发的数据管道
fn message_send_consume() {
    let (sender1, rcv1) = bounded(1);
    let (sender2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // sender
        s.spawn(|_| {
            for i in 0..n_msgs {
                sender1.send(i).unwrap();
                println!("source sent {}", i);
            }

            // 发送完毕后，关闭sender1
            drop(sender1);
        });

        // 2个接收者消费
        // 对于消费者rcv1进行消费
        for _ in 0..n_workers {
            let (sender, rcv) = (sender2.clone(), rcv1.clone());
            // 创建子线程进行消费
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in rcv.iter() {
                    println!("worker {:?} received {}.", thread::current().id(), msg);
                    sender.send(msg * 2).unwrap();
                }
            });
        }

        // 关闭sender2 否则接收器 rcv2 不会关闭
        drop(sender2);
        for msg in rcv2.iter() {
            println!("sink received {}", msg);
        }
    })
    .unwrap();
}

// 这个实例示范了在单生产者、单消费者（SPSC）环境中使用 crossbeam-channel。
// 我们构建的生成短期线程实例中，使用 crossbeam::scope 和 Scope::spawn来管理生产者线程。
// 在两个线程之间，使用 crossbeam_channel::unbounded
// 信道交换数据，这意味着可存储消息的数量没有限制。生产者线程在消息之间休眠半秒。
fn sender_rcv() {
    let (sender, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                sender.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    // 关闭发送者通道
    drop(sender);

    // 消费
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("received data:{}", msg);
    }
}
