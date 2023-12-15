struct MinStack {
    stack: Vec<i32>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: 0,
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.stack.len() == 1 {
            self.min = val;
        } else if val < self.min {
            self.min = val;
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        if self.stack.len() == 0 {
            self.min = 0;
        } else {
            self.min = *self.stack.iter().min().unwrap();
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}
