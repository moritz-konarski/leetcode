// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
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

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head.expect("there is at least one node");
        let mut middle = current;
        let mut counter = 1;

        while let Some(next) = &current.next {
            if counter % 2 == 0 {
                middle = &current;
            }
            current = &next;
            counter += 1;
        }

        Some(middle.clone())
    }
}

#[test]
fn test1() {
    // Input: head = [1,2,3,4,5]
    // Output: [3,4,5]
    // Explanation: The middle node of the list is node 3.
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(2);
    let mut n3 = ListNode::new(3);
    let mut n4 = ListNode::new(4);
    let n5 = ListNode::new(5);
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4));
    n2.next = Some(Box::new(n3.clone()));
    n1.next = Some(Box::new(n2));
    assert_eq!(
        Some(Box::new(n3)),
        Solution::middle_node(Some(Box::new(n1)))
    );
}

#[test]
fn test2() {
    // Input: head = [1,2,3,4,5,6]
    // Output: [4,5,6]
    // Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(2);
    let mut n3 = ListNode::new(3);
    let mut n4 = ListNode::new(4);
    let mut n5 = ListNode::new(5);
    let n6 = ListNode::new(6);
    n5.next = Some(Box::new(n6));
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4.clone()));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));
    assert_eq!(
        Some(Box::new(n4)),
        Solution::middle_node(Some(Box::new(n1)))
    );
}

#[test]
fn test3() {
    let n1 = ListNode::new(1);
    assert_eq!(
        Some(Box::new(n1.clone())),
        Solution::middle_node(Some(Box::new(n1)))
    );
}

fn main() {}
