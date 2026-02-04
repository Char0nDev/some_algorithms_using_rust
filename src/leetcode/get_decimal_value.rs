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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut bin_val = String::new();
    let mut current = &head;

    while let Some(node) = current {
        bin_val.push_str(&format!("{}", node.val).to_string());
        current = &node.next;
    }

    i32::from_str_radix(&bin_val, 2).unwrap()
}
