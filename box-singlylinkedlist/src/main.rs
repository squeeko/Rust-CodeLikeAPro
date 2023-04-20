#![allow(dead_code)]
#![allow(unused_variables)]

/*
In Rust, self and Self mean two difference things.

self refers to the current module or object.
Self refers to the type of the current module or object.

One of the examples that clarifies this difference is the implementation of PartialEq. Let’s imagine that we want to implement PartialEq for an struct.

struct Employee {
  name: String,
  next_meeting: MeetingType
}

impl PartialEq for Employee {
  fn eq(&self, other: &Self) -> bool { // &self = refers to the current module or object
                                          other: &Self = refers to the type of the current module or object, this type is an "Employee" type
        self.name == other.name
    }
}

https://hannydevelop.hashnode.dev/the-ultimate-guide-to-self-self-in-rust-ckr0rctcp058aqus1hriabnoe

 */
struct ListItem<T> {
    data: Box<T>, // Data is "Box'd" and the data field cannot be empty or null
    next: Option<Box<ListItem<T>>>, // This field is Optional as we are unsure of the number of elements and so we "Box'd" it as well...These are the two use cases for Box!
}

struct SinglyLinkedList<T> {
    head: ListItem<T>, // Since this only is going to contain "head" we DO NOT Box because it is ALWAYS present, NEVER empty NOR null.
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data), // The new data is moved into the new ListItem by pacing it in a Box, whihc allocates it on the Heap. The compiler will sort out the details regarding getting the data into the "target location" as it may be have to be moved from the Stack to the Heap.
            next: None, // Initialized as None as we are not sure where the items are located in the list yet. Also there is no "insert" operation only "append"
        }
    }

    fn next(&self) -> Option<&Self> {
        // This function returns an "Optional" reference to the next item if it exists. This function helps to unwrap the nested references to simplify the code.
        if let Some(next) = &self.next {
            // The "if let" construct to cheek if the "next" pointer, points to anything BEFORE trying to dereference it.
            // We then deference the "next" Box pointer, turn it into an ordinary reference and return that inside an Option.
            // This is merely to make the code a little easier to use, as the alternative is to return a reference to a Box, rather than the object directly.
            Some(&*next)
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() { // 
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
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
}
