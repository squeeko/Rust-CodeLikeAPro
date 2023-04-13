#![allow(dead_code)]
#![allow(unused_variables)]
struct ListItem<T> {
    data: Box<T>, // Data is "Box'd" and the data field cannot be empty or null
    next: Option<Box<ListItem<T>>> // This field is Optional as we are unsure of the number of elements and so we "Box'd" it as well...These are the two use cases for Box!
}

struct SinglyLinkedList<T> {
    head: ListItem<T>, // Since this only is going to contain "head" we DO NOT Box because it is ALWAYS present, NEVER empty NOR null.
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data), // The new data is 
            next: None
    }
}

fn next(&self) -> Option<&Self> {
    if let Some(next) = &self.next {
        Some(&*next)
    } else {
        None
    }
}

fn mut_tail(&mut self) -> &mut Self {
    if self.next.is_some() {
        self.next.as_mut().unwrap().mut_tail()
    } else {
        self
    }
}

fn data(&self) -> &T {
    self.data.as_ref()
}
}

impl<T> SinglyLinkedList<T> {
    fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }

    fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }

    fn head(&self) -> &ListItem<T> {
        &self.head
    }
}


fn main() {
    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("tail");
    let mut item= list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;

        } else {
            break;
        }
    }
}
