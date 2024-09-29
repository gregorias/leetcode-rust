// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//
// Some Leetcode commenters tried using slow and fast pointers. That’s nonsense or at least faulty
// pattern matching. The problem just requires a scout pointer that’s n steps ahead.

mod remove_nth_node_from_end_of_list {
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

    #[allow(unused)]
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut scout: &Option<Box<ListNode>> = &head;

        // Move the scout n steps ahead.
        for _ in 0..n {
            scout = &scout.as_ref().unwrap().next;
        }

        let mut lagging_ptr: &Option<Box<ListNode>> = &head;
        let mut result: Option<Box<ListNode>> = None;
        let mut result_tail = &mut result;

        // Copy the nodes from the head to the node before the one to be removed.
        while let Some(scout_node) = scout.as_deref() {
            if let Some(lagging_node) = lagging_ptr.as_deref() {
                *result_tail = Some(Box::new(ListNode::new(lagging_node.val)));
                result_tail = &mut result_tail.as_deref_mut().unwrap().next;

                scout = &scout_node.next;
                lagging_ptr = &lagging_node.next;
            }
        }

        // Skip the node to be removed.
        lagging_ptr = &lagging_ptr.as_deref().unwrap().next;

        // Copy the remaining tail.
        while let Some(laggging_node) = lagging_ptr.as_deref() {
            *result_tail = Some(Box::new(ListNode::new(laggging_node.val)));
            result_tail = &mut result_tail.as_deref_mut().unwrap().next;
            lagging_ptr = &lagging_ptr.as_deref().unwrap().next;
        }

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let orig = ListNode::cons(
                1,
                ListNode::cons(2, ListNode::cons(3, ListNode::cons(4, ListNode::new(5)))),
            );
            let expected =
                ListNode::cons(1, ListNode::cons(2, ListNode::cons(3, ListNode::new(5))));
            assert_eq!(
                remove_nth_from_end(Some(Box::new(orig)), 2),
                Some(Box::new(expected))
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                remove_nth_from_end(Some(Box::new(ListNode::new(1))), 1),
                None
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                remove_nth_from_end(Some(Box::new(ListNode::cons(1, ListNode::new(2)))), 1),
                Some(Box::new(ListNode::new(1)))
            );
        }
    }
}
