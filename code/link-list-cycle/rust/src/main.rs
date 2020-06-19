use std::collections::{HashSet, LinkedList};
use std::boxed::Box;
use std::ptr::NonNull;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);
    let has_cycle = does_linked_list_have_cycle(&list);
    println!("Has cycle: {:?}", has_cycle);

    let my_custom_list = make_custom_linked_list();
    let has_cycle_custom = does_my_linked_list_have_cycle(&my_custom_list);
    println!("Has cycle in custom: {:?}", has_cycle_custom);

    let bad_list = make_custom_linked_list_with_cycle();
    let has_cycle_bad = does_my_linked_list_have_cycle(&bad_list);
    println!("Has cycle in bad list: {:?}", has_cycle_bad);
}

fn does_linked_list_have_cycle<T>(linked_list: &LinkedList<T>) -> bool {
    let mut cursor = linked_list.iter();
    let mut count = 0;
    let list_length = linked_list.len();

    while count <= list_length {
        if cursor.next().is_none() {
            return false;
        }
        count = count + 1;
    }
    
    true
}

struct MyLinkedListNode {
    prev: Option<NonNull<MyLinkedListNode>>,
    next: Option<NonNull<MyLinkedListNode>>,
    value: String
}

struct MyLinkedList {
    first: Option<NonNull<MyLinkedListNode>>,
    last: Option<NonNull<MyLinkedListNode>>,
}

impl MyLinkedList {
    fn new() -> MyLinkedList {
        MyLinkedList{
            first: None,
            last: None
        }
    }
}

fn make_custom_linked_list() -> MyLinkedList {
    // we'll construct our linked list ourselves, prone to error!!!
    let mut list = MyLinkedList::new();

    // make nodes but have them be unlinked for now
    let node_a = MyLinkedListNode{
        value: "A".to_owned(),
        prev: None,
        next: None,
    };
    let node_b = MyLinkedListNode{
        value: "B".to_owned(),
        prev: None,
        next: None,
    };
    let node_c = MyLinkedListNode{
        value: "C".to_owned(),
        prev: None,
        next: None,
    };
    let node_d = MyLinkedListNode{
        value: "D".to_owned(),
        prev: None,
        next: None,
    };

    // now link the nodes correctly
    let node_a_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_a)));
    let node_b_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_b)));
    let node_c_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_c)));
    let node_d_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_d)));

    unsafe {
        (*node_a_nonnull_ptr.as_ptr()).next = Some(node_b_nonnull_ptr);
        (*node_b_nonnull_ptr.as_ptr()).next = Some(node_c_nonnull_ptr);
        (*node_c_nonnull_ptr.as_ptr()).next = Some(node_d_nonnull_ptr);

        (*node_d_nonnull_ptr.as_ptr()).prev = Some(node_c_nonnull_ptr);
        (*node_c_nonnull_ptr.as_ptr()).prev = Some(node_b_nonnull_ptr);
        (*node_b_nonnull_ptr.as_ptr()).prev = Some(node_a_nonnull_ptr);

        list.first = Some(node_a_nonnull_ptr);
        list.last = Some(node_d_nonnull_ptr);
    }

    list
}

fn make_custom_linked_list_with_cycle() -> MyLinkedList {
    // we'll construct our linked list ourselves, prone to error!!!
    let mut list = MyLinkedList::new();

    // make nodes but have them be unlinked for now
    let node_a = MyLinkedListNode{
        value: "A".to_owned(),
        prev: None,
        next: None,
    };
    let node_b = MyLinkedListNode{
        value: "B".to_owned(),
        prev: None,
        next: None,
    };
    let node_c = MyLinkedListNode{
        value: "C".to_owned(),
        prev: None,
        next: None,
    };
    let node_d = MyLinkedListNode{
        value: "D".to_owned(),
        prev: None,
        next: None,
    };

    // now link the nodes correctly
    let node_a_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_a)));
    let node_b_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_b)));
    let node_c_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_c)));
    let node_d_nonnull_ptr = NonNull::from(Box::leak(Box::new(node_d)));

    unsafe {
        (*node_a_nonnull_ptr.as_ptr()).next = Some(node_b_nonnull_ptr);
        (*node_b_nonnull_ptr.as_ptr()).next = Some(node_c_nonnull_ptr);

        // put a typo in here and set next to node_a
        (*node_c_nonnull_ptr.as_ptr()).next = Some(node_a_nonnull_ptr);

        (*node_d_nonnull_ptr.as_ptr()).prev = Some(node_c_nonnull_ptr);
        (*node_c_nonnull_ptr.as_ptr()).prev = Some(node_b_nonnull_ptr);
        (*node_b_nonnull_ptr.as_ptr()).prev = Some(node_a_nonnull_ptr);

        list.first = Some(node_a_nonnull_ptr);
        list.last = Some(node_d_nonnull_ptr);
    }

    list
}

fn does_my_linked_list_have_cycle(linked_list: &MyLinkedList) -> bool {
    if linked_list.first.is_none() {
        return false;
    }
    let mut observed_pointers = HashSet::new();
    let has_cycle: bool;
    
    unsafe {
        let mut node = linked_list.first.unwrap();
        loop {
            let node_ptr = node.as_ptr();
            let node_ref = node.as_ref();
            if observed_pointers.insert(node_ptr) == false {
                println!("found loop at point {:?}", node_ref.value);
                has_cycle = true;
                break;
            }
            
            if node_ref.next.is_none() {
                has_cycle = false;
                break;
            }

            node = node_ref.next.unwrap();
        }
    }

    has_cycle
}
