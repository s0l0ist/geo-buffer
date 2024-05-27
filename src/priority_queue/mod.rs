#![allow(dead_code)]
pub(crate) struct PriorityQueue<T: std::cmp::PartialOrd> {
    content: Vec<T>,
}

impl<T: std::cmp::PartialOrd> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.content.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn insert(&mut self, item: T) {
        self.content.push(item);
        let mut cur = self.content.len() - 1;
        while cur > 0 {
            let par = (cur - 1) / 2;
            if self.content[cur] < self.content[par] {
                self.content.swap(cur, par);
                cur = par;
            } else {
                break;
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.content.get(0)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ret = self.content.swap_remove(0);
        let mut cur = 0;
        while cur < self.content.len() {
            let lc = cur * 2 + 1;
            let rc = cur * 2 + 2;
            let nc;
            if lc >= self.content.len() {
                break;
            } else if rc >= self.content.len() {
                nc = lc;
            } else {
                nc = if self.content[lc] < self.content[rc] {
                    lc
                } else {
                    rc
                };
            }
            if self.content[nc] < self.content[cur] {
                self.content.swap(cur, nc);
                cur = nc;
            } else {
                break;
            }
        }
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        assert!(pq.is_empty());

        pq.insert(3.0);
        pq.insert(1.0);
        pq.insert(2.0);

        assert_eq!(pq.peek(), Some(&1.0));
        assert_eq!(pq.pop(), Some(1.0));
        assert_eq!(pq.pop(), Some(2.0));
        assert_eq!(pq.pop(), Some(3.0));
        assert!(pq.is_empty());
    }
}
