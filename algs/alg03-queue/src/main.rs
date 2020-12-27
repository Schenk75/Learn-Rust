mod array_queue;
// mod linkedlist_queue;
mod circle_queue;

use array_queue::ArrayQueue;
// use linkedlist_queue::LinkedListQueue;
use circle_queue::CircleQueue;

fn main() {
    println!("Array Queue:");
    let mut queue = ArrayQueue::new(3);
    queue.enqueue(2);
    queue.enqueue(2);
    queue.enqueue(2);
    queue.enqueue(2);
    println!("{}", queue);
    queue.dequeue();
    queue.dequeue();
    queue.enqueue(4);
    queue.dequeue();
    println!("{}\n", queue);

    // println!("LinkedList Queue:");
    // let mut m = LinkedListQueue::new();
    // m.enqueue(4);
    // m.enqueue(4);
    // m.enqueue(4);
    // m.dequeue();
    // m.dequeue();
    // println!("{:?}\n", m);

    println!("Circle Queue:");
    let mut queue = CircleQueue::new(3);
    queue.enqueue(2);
    println!("{}", queue);
    queue.enqueue(2);
    println!("{}", queue);
    queue.enqueue(2);
    println!("{}", queue);
    queue.enqueue(2);
    println!("{}", queue);
    queue.dequeue();
    println!("{}", queue);
    queue.dequeue();
    println!("{}", queue);
    queue.enqueue(4);
    println!("{}", queue);
    queue.dequeue();
    println!("{}", queue);
}
