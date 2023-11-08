impl Solution {
    /// this is already really good
    /// 3 ms and 2.18MB
    pub fn add_two_numbers(
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
