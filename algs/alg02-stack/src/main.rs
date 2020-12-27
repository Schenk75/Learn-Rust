mod array_stack;
mod linkedlist_stack;
mod simple_browser;

use array_stack::ArrayStack;
use linkedlist_stack::LinkedListStack;
use simple_browser::Browser;

fn main() {
    let mut array_stack = ArrayStack::new();
    array_stack.push(1);
    array_stack.push(2);
    array_stack.push(3);
    println!("Array Stack:\n{:?}", array_stack);
    println!("pop: {:?}", array_stack.pop());
    println!("{:?}", array_stack);
    println!("top: {:?}\n", array_stack.top());

    let mut link_stack = LinkedListStack::new();
    link_stack.push("hello".to_string());
    link_stack.push("world".to_string());
    link_stack.push("!".to_string());
    println!("Linked List Stack:\n{}", link_stack);
    println!("pop: {:?}", link_stack.pop());
    println!("{}\n", link_stack);

    println!("Simple Browser:");
    let mut browser = Browser::new();
    browser.open(&"http://www.baidu.com".to_string());
    browser.open(&"http://news.baidu.com/".to_string());
    browser.open(&"http://news.baidu.com/ent".to_string());
    browser.go_back();
    browser.go_back();
    browser.go_forward();
    browser.open(&"http://www.qq.com".to_string());
    browser.go_forward();
    browser.go_back();
    browser.go_forward();
    browser.go_back();
    browser.go_back();
    browser.go_back();
    browser.go_back();
    browser.check_current_page();
}