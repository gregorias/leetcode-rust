// https://leetcode.com/problems/add-two-numbers/
//
#![allow(unused)]
mod add_two_numbers {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    fn add_carry(a: i32, b: i32, carry: i32) -> (i32, i32) {
        let sum = a + b + carry;
        (sum % 10, sum / 10)
    }

    fn get_next(node: Option<&ListNode>) -> Option<&ListNode> {
        node.and_then(|node| node.next.as_deref())
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l_cursor, mut r_cursor): (Option<&ListNode>, Option<&ListNode>) =
            (l1.as_deref(), l2.as_deref());

        let mut carry: i32 = 0;
        let mut output_head: Option<Box<ListNode>> = None;
        // Now the mutable tail
        let mut output_tail: &mut Option<Box<ListNode>> = &mut output_head;

        while carry > 0 || l_cursor.is_some() || r_cursor.is_some() {
            let (l_val, r_val) = (
                l_cursor.map_or(0, |node| node.val),
                r_cursor.map_or(0, |node| node.val),
            );
            let (sum, carry) = add_carry(l_val, r_val, carry);
            *output_tail = Some(Box::new(ListNode::new(sum)));
            // I couldn’t figure out how to do this without the unwrap or another kind of
            // unwrapping. The (overly strict) borrow checker is in the way.
            output_tail = &mut output_tail.as_mut().unwrap().next;

            (l_cursor, r_cursor) = (get_next(l_cursor), get_next(r_cursor));
        }
        output_head
    }
}
