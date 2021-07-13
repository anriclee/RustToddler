enum List<T> {
    Empty,
    NonEmpty(Box<ListNode<T>>),
}

struct ListNode<T> {
    element: T,
    next: List<T>,
}

fn print_node_val(root: &List<i32>) {
    match root {
        Empty => return,
        NonEmpty(node) => {
            println!("the value:{}", node.element);
            print_node_val(&node.next);
        }
    }
}

use self::List::*;

fn main() {
    // build a list
    let n1 = NonEmpty(Box::new(ListNode {
        element: 1,
        next: Empty,
    }));

    let n2 = NonEmpty(Box::new(ListNode {
        element: 2,
        next: n1,
    }));

    let n3 = NonEmpty(Box::new(ListNode {
        element: 3,
        next: n2,
    }));

    print_node_val(&n3);
}
