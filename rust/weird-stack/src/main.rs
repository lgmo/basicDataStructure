/// This is not a good way to make a stack in rust, but it works

enum StackNode {
    Last,
    Middle(i32, Box<StackNode>),
    
}

struct MyStack {
    head: StackNode,
}

impl MyStack {
    fn new() -> MyStack {
        MyStack {
            head: StackNode::Last
        }
    }

    fn push(mut self, val: i32) {
        self.head = StackNode::Middle(val, Box::new(self.head))
    }

    fn pop(mut self) {
        self.head = match self.head {
            StackNode::Last => StackNode::Last,
            StackNode::Middle(_, n) => *n,
        }
    }

    fn top(&self) -> Option<i32> {
        match self.head {
            StackNode::Last => None,
            StackNode::Middle(x, _) => Some(x),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
