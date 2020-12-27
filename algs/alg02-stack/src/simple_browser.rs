use crate::linkedlist_stack::LinkedListStack;

// 模拟实现一个浏览器的前进、后退功能
#[derive(Debug, Default, Clone)]
pub struct Browser {
    forward_stack: LinkedListStack,
    back_stack: LinkedListStack,
    current_page: String,
}

impl Browser {
    pub fn new() -> Self {
        Default::default()
    }
    
    fn show_url(&mut self, url: &String, prefix: String) {
        self.current_page = url.to_string();
        println!("{} page == {}", prefix, url);
    }

    fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }

    fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    pub fn open(&mut self, url: &String) {
        if !self.current_page.is_empty() {
            self.back_stack.push(self.current_page.clone());
            self.forward_stack.clear();
        }
        self.show_url(url, "Open".to_string());
    }

    pub fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.forward_stack.push(self.current_page.clone());
            let back_url = self.back_stack.pop().unwrap();
            self.show_url(&back_url, "Back".to_string());
            return Some(back_url);
        }

        println!("Can not go back, there is no page behind");
        None
    }

    pub fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.back_stack.push(self.current_page.clone());
            let forward_url = self.forward_stack.pop().unwrap();
            self.show_url(&forward_url, "Forward".to_string());
            return Some(forward_url);
        }

        println!("Can not go forward, there is no page forward");
        None
    }

    pub fn check_current_page(&self) {
        println!("current page: {}", self.current_page);
    }
}