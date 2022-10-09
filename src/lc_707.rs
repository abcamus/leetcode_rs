struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {}

    fn get(&self, index: i32) -> i32 {
        if index as usize >= self.q.len() {
            return -1;
        }
        return self.q[index as usize];
    }

    fn add_at_head(&mut self, val: i32) {
        self.q.push_front(val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.q.push_back(val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index as usize > self.q.len() {
            return;
        } else if index as usize == self.q.len() {
            self.add_at_tail(val);
        } else if index < 0 {
            self.add_at_head(val);
        } else {
            self.q[index as usize] = val;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index as usize >= self.q.len() {
            return;
        }
        self.q.remove(index as usize);
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
#[test]
fn test_my_linked_list() {}
