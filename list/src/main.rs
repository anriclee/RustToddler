struct ListNode<T> {
    element: T,
    next: Box<Option<ListNode<T>>>,
}

use std::ops::Deref;

fn print_node_val(root: &ListNode<u32>) {
    println!("the value:{}", root.element);
    match root.next.deref() {
        Some(node) => {
            print_node_val(&node);
        }
        None => {
            return;
        }
    }
}

fn main() {
    // build a list
    let n1 = ListNode {
        element: 1,
        next: Box::new(None),
    };

    let n2 = ListNode {
        element: 2,
        next: Box::new(Some(n1)),
    };

    let n3 = ListNode {
        element: 3,
        next: Box::new(Some(n2)),
    };

    print_node_val(&n3);
}
