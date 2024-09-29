// https://leetcode.com/problems/merge-k-sorted-lists/

mod merge_k_sorted_lists {
    #[derive(PartialEq, Eq, Clone, Debug)]
    struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    #[allow(unused)]
    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }

        fn cons(val: i32, tail: ListNode) -> Self {
            ListNode {
                next: Some(Box::new(tail)),
                val,
            }
        }
    }

    use std::collections::BinaryHeap;

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct HeadsQueueItem<'a> {
        pub list: &'a ListNode,
    }

    impl Ord for HeadsQueueItem<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.list.val.cmp(&other.list.val) {
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            }
        }
    }

    impl PartialOrd for HeadsQueueItem<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }

    struct HeadsQueue<'a> {
        heads: BinaryHeap<HeadsQueueItem<'a>>,
    }

    impl<'a> HeadsQueue<'a> {
        fn new(lists: &'a Vec<Option<Box<ListNode>>>) -> Self {
            let mut queue = HeadsQueue {
                heads: BinaryHeap::new(),
            };

            for list in lists {
                if let Some(node) = list.as_deref() {
                    queue.heads.push(HeadsQueueItem {
                        list: &node,
                    });
                }
            }
            queue
        }

        fn pop(&mut self) -> Option<i32> {
            let item = self.heads.pop();
            if let Some(item) = item {
                if let Some(tail) = item.list.next.as_deref() {
                    self.heads.push(HeadsQueueItem {
                        list: &tail,
                    });
                }
                Some(item.list.val)
            } else {
                None
            }
        }
    }

    #[allow(unused)]
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heads: HeadsQueue = HeadsQueue::new(&lists);
        let mut result: Option<Box<ListNode>> = None;
        let mut result_tail: &mut Option<Box<ListNode>> = &mut result;

        while let Some(val) = heads.pop() {
            *result_tail = Some(Box::new(ListNode::new(val)));
            result_tail = &mut result_tail.as_deref_mut().unwrap().next;
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let onefourfive = ListNode::cons(1, ListNode::cons(4, ListNode::new(5)));
            let onethreefour = ListNode::cons(1, ListNode::cons(3, ListNode::new(4)));
            let twosix = ListNode::cons(2, ListNode::new(6));
            let input: Vec<Option<Box<ListNode>>> = vec![
                Some(Box::new(onefourfive)),
                Some(Box::new(onethreefour)),
                Some(Box::new(twosix)),
            ];

            let expected = ListNode::cons(
                1,
                ListNode::cons(
                    1,
                    ListNode::cons(
                        2,
                        ListNode::cons(
                            3,
                            ListNode::cons(
                                4,
                                ListNode::cons(4, ListNode::cons(5, ListNode::new(6))),
                            ),
                        ),
                    ),
                ),
            );

            assert_eq!(merge_k_lists(input), Some(Box::new(expected)));
        }

        #[test]
        fn test_2() {
            assert_eq!(merge_k_lists(vec![None]), None);
        }

        #[test]
        fn test_3() {
            assert_eq!(merge_k_lists(vec![]), None);
        }
    }
}
