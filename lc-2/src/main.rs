impl Solution {
    /// this is better, but it seems to pay to keep the match as small as possible
    /// 3ms and 2.16 MB
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // get mut references to the lists
        let mut l1 = &l1;
        let mut l2 = &l2;

        // there is at least one node in each of these lists
        let mut num1 = l1.as_ref().unwrap().val;
        let mut num2 = l2.as_ref().unwrap().val;

        let mut sum = num1 + num2;
        let mut carry = sum / 10;

        // start the result list
        let mut result_head = Some(Box::from(ListNode::new(sum % 10)));
        let mut result_tail = &mut result_head;

        loop {
            // advance the lists conditionally
            match (&l1, &l2) {
                (Some(n1), Some(n2)) => {
                    l1 = &n1.next;
                    num1 = if let Some(node) = l1 { node.val } else { 0 };

                    l2 = &n2.next;
                    num2 = if let Some(node) = l2 { node.val } else { 0 };

                    if l1.is_none() && l2.is_none() && carry == 0 {
                        break;
                    }
                }
                (Some(n1), None) => {
                    l1 = &n1.next;
                    num1 = if let Some(node) = l1 { node.val } else { 0 };

                    num2 = 0;

                    if l1.is_none() && carry == 0 {
                        break;
                    }
                }
                (None, Some(n2)) => {
                    num1 = 0;

                    l2 = &n2.next;
                    num2 = if let Some(node) = l2 { node.val } else { 0 };

                    if l2.is_none() && carry == 0 {
                        break;
                    }
                }
                (None, None) => {
                    break;
                }
            }

            sum = num1 + num2 + carry;
            carry = sum / 10;

            // the result tail is always Some
            result_tail.as_mut().unwrap().next = Some(Box::from(ListNode::new(sum % 10)));
            result_tail = &mut result_tail.as_mut().unwrap().next;
        }

        result_head
    }
    /// a giant match is much worse
    /// 6 ms and 2.26 MB
    pub fn add_two_numbers3(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // get mut references to the lists
        let mut l1 = &l1;
        let mut l2 = &l2;

        // there is at least one node in each of these lists
        let mut num1 = l1.as_ref().unwrap().val;
        let mut num2 = l2.as_ref().unwrap().val;

        let mut sum = num1 + num2;
        let mut carry = sum / 10;

        // start the result list
        let mut result_head = Some(Box::from(ListNode::new(sum % 10)));
        let mut result_tail = &mut result_head;

        loop {
            // advance the lists conditionally
            match (l1.as_deref(), l2.as_deref()) {
                // both are nodes and have next nodes
                (Some(ListNode { next: Some(n1), .. }), Some(ListNode { next: Some(n2), .. })) => {
                    l1 = &l1.as_ref().unwrap().next;
                    l2 = &l2.as_ref().unwrap().next;
                    num1 = n1.val;
                    num2 = n2.val;
                }
                // both are nodes but l2 is the last one
                (Some(ListNode { next: Some(n1), .. }), Some(ListNode { next: None, .. })) => {
                    l1 = &l1.as_ref().unwrap().next;
                    l2 = &None;
                    num1 = n1.val;
                    num2 = 0;
                }
                // both are nodes but l1 is the last one
                (Some(ListNode { next: None, .. }), Some(ListNode { next: Some(n2), .. })) => {
                    l1 = &None;
                    l2 = &l2.as_ref().unwrap().next;
                    num1 = 0;
                    num2 = n2.val;
                }
                // both are nodes, but both are the last ones
                (Some(ListNode { next: None, .. }), Some(ListNode { next: None, .. })) => {
                    l1 = &None;
                    l2 = &None;
                    num1 = 0;
                    num2 = 0;
                }
                // l1 is a node with a next, l2 is done
                (Some(ListNode { next: Some(n1), .. }), None) => {
                    l1 = &l1.as_ref().unwrap().next;
                    num1 = n1.val;
                }
                (Some(ListNode { next: None, .. }), None) => {
                    l1 = &None;
                    num1 = 0;
                }
                (None, Some(ListNode { next: Some(n2), .. })) => {
                    l2 = &l2.as_ref().unwrap().next;
                    num2 = n2.val;
                }
                (None, Some(ListNode { next: None, .. })) => {
                    l2 = &None;
                    num2 = 0;
                }
                (None, None) => {
                    break;
                }
            }

            sum = num1 + num2 + carry;
            carry = sum / 10;

            if sum == 0 && l1.is_none() && l2.is_none() {
                break;
            }

            // the result tail is always Some
            result_tail.as_mut().unwrap().next = Some(Box::from(ListNode::new(sum % 10)));
            result_tail = &mut result_tail.as_mut().unwrap().next;
        }

        result_head
    }

    /// this is way better
    /// 0 ms and 2.03 MB
    pub fn add_two_numbers2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // get mut references to the lists
        let mut l1 = &l1;
        let mut l2 = &l2;

        // there is at least one node in each of these lists
        let mut num1 = l1.as_ref().unwrap().val;
        let mut num2 = l2.as_ref().unwrap().val;

        let mut sum = num1 + num2;
        let mut carry = sum / 10;

        // start the result list
        let mut result_head = Some(Box::from(ListNode::new(sum % 10)));
        let mut result_tail = &mut result_head;

        loop {
            // advance the lists conditionally
            match (&l1, &l2) {
                (Some(n1), Some(n2)) => {
                    l1 = &n1.next;
                    num1 = if let Some(node) = l1 { node.val } else { 0 };

                    l2 = &n2.next;
                    num2 = if let Some(node) = l2 { node.val } else { 0 };
                }
                (Some(n1), None) => {
                    l1 = &n1.next;
                    num1 = if let Some(node) = l1 { node.val } else { 0 };

                    num2 = 0;
                }
                (None, Some(n2)) => {
                    num1 = 0;

                    l2 = &n2.next;
                    num2 = if let Some(node) = l2 { node.val } else { 0 };
                }
                (None, None) => {
                    break;
                }
            }

            sum = num1 + num2 + carry;
            carry = sum / 10;

            if sum == 0 && l1.is_none() && l2.is_none() {
                break;
            }

            // the result tail is always Some
            result_tail.as_mut().unwrap().next = Some(Box::from(ListNode::new(sum % 10)));
            result_tail = &mut result_tail.as_mut().unwrap().next;
        }

        result_head
    }
    /// this is already really good
    /// 3 ms and 2.18MB
    pub fn add_two_numbers1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;

        // the carry for the addtion
        let mut carry = 0;
        let mut result_tail: &mut Option<Box<ListNode>> = &mut None;
        let mut result_head = None;

        loop {
            let num1 = if let Some(node) = l1 { node.val } else { 0 };
            let num2 = if let Some(node) = l2 { node.val } else { 0 };

            let sum = num1 + num2 + carry;

            if sum == 0 && l1.is_none() && l2.is_none() {
                break;
            }
            // println!("{:?} + {:?} + ({:?}) = {:?}", num1, num2, carry, sum % 10);
            carry = sum / 10;

            let new_node = Some(Box::from(ListNode::new(sum % 10)));
            if let Some(ref mut node) = result_tail {
                node.next = new_node;
                result_tail = &mut node.next;
            } else {
                result_head = new_node;
                result_tail = &mut result_head;
            }

            match (&l1, &l2) {
                (Some(n1), Some(n2)) => {
                    l1 = &n1.next;
                    l2 = &n2.next;
                }
                (Some(n1), None) => {
                    l1 = &n1.next;
                }
                (None, Some(n2)) => {
                    l2 = &n2.next;
                }
                (None, None) => {
                    break;
                }
            }
        }

        result_head
    }
}

#[test]
fn test_342_plus_465() {
    let mut list_1 = ListNode::new(2);
    let mut n2 = ListNode::new(4);
    let n3 = ListNode::new(3);
    n2.next = Some(Box::from(n3));
    list_1.next = Some(Box::from(n2));
    let list_1 = Some(Box::from(list_1));

    let mut list_2 = ListNode::new(5);
    let mut n2 = ListNode::new(6);
    let n3 = ListNode::new(4);
    n2.next = Some(Box::from(n3));
    list_2.next = Some(Box::from(n2));
    let list_2 = Some(Box::from(list_2));

    let mut result = ListNode::new(7);
    let mut n2 = ListNode::new(0);
    let n3 = ListNode::new(8);
    n2.next = Some(Box::from(n3));
    result.next = Some(Box::from(n2));
    let result = Some(Box::from(result));

    println!("Explanation: 342 + 465 = 807.");
    assert_eq!(result, Solution::add_two_numbers(list_1, list_2));
}

#[test]
fn test_0_plus_0() {
    let list_1 = Some(Box::from(ListNode::new(0)));

    let list_2 = Some(Box::from(ListNode::new(0)));

    let result = Some(Box::from(ListNode::new(0)));

    println!("Explanation: 0 + 0 = 0.");
    assert_eq!(result, Solution::add_two_numbers(list_1, list_2));
}

#[test]
fn test_9999999_plus_9999() {
    let mut list_1 = ListNode::new(9);
    let mut n2 = ListNode::new(9);
    let mut n3 = ListNode::new(9);
    let mut n4 = ListNode::new(9);
    let mut n5 = ListNode::new(9);
    let mut n6 = ListNode::new(9);
    let n7 = ListNode::new(9);
    n6.next = Some(Box::from(n7));
    n5.next = Some(Box::from(n6));
    n4.next = Some(Box::from(n5));
    n3.next = Some(Box::from(n4));
    n2.next = Some(Box::from(n3));
    list_1.next = Some(Box::from(n2));
    let list_1 = Some(Box::from(list_1));

    let mut list_2 = ListNode::new(9);
    let mut n2 = ListNode::new(9);
    let mut n3 = ListNode::new(9);
    let n4 = ListNode::new(9);
    n3.next = Some(Box::from(n4));
    n2.next = Some(Box::from(n3));
    list_2.next = Some(Box::from(n2));
    let list_2 = Some(Box::from(list_2));

    let mut result = ListNode::new(8);
    let mut n2 = ListNode::new(9);
    let mut n3 = ListNode::new(9);
    let mut n4 = ListNode::new(9);
    let mut n5 = ListNode::new(0);
    let mut n6 = ListNode::new(0);
    let mut n7 = ListNode::new(0);
    let n8 = ListNode::new(1);
    n7.next = Some(Box::from(n8));
    n6.next = Some(Box::from(n7));
    n5.next = Some(Box::from(n6));
    n4.next = Some(Box::from(n5));
    n3.next = Some(Box::from(n4));
    n2.next = Some(Box::from(n3));
    result.next = Some(Box::from(n2));
    let result = Some(Box::from(result));

    println!("Explanation: 9999999 + 9999 = 10009998.");
    assert_eq!(result, Solution::add_two_numbers(list_1, list_2));
}

// Definition for singly-linked list.
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

struct Solution {}

fn main() {}
