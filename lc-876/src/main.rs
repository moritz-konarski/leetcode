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
    // removing the counter may reduce memory even further
    // directly operating on the Option may also help
    /// 0 ms and 1.99 MB
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // we know there is at least one node
        let mut first_node = &head;
        // for one node its its own middle
        let mut middle_node = first_node;

        // we take two steps with each loop iteration
        while let Some(ListNode {
            next: Some(next_node),
            ..
        }) = first_node.as_deref()
        {
            first_node = &next_node.next;
            // this always works because the first node has already traversed this
            middle_node = &middle_node.as_ref().unwrap().next;
        }

        middle_node.to_owned()
    }

    /// 0 ms and 2.01 MB
    pub fn middle_node2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head.expect("there is at least one node");
        let mut middle = current;
        let mut node_counter = 1;

        while let Some(next) = &current.next {
            current = &next;
            node_counter += 1;

            // by moving the middle pointer every two nodes we keep track of the middle
            if node_counter % 2 == 0 {
                // this always works because middle follows the current pointer
                if let Some(next_middle) = &middle.next {
                    middle = next_middle;
                }
            }
        }

        Some(middle.to_owned())
    }
    /// this beats the pants off other solutions
    /// 0 ms and 2.13 MB
    pub fn middle_node1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head.expect("there is at least one node");
        let mut middle = current;
        let mut node_counter = 1;

        while let Some(next) = &current.next {
            current = &next;
            node_counter += 1;

            // by moving the middle pointer every two nodes we keep track of the middle
            if node_counter % 2 == 0 {
                // this always works because middle follows the current pointer
                if let Some(next_middle) = &middle.next {
                    middle = next_middle;
                }
            }
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
