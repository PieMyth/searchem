#[path = "contact.rs"]
pub mod contact;

#[derive(Debug, Clone)]
pub struct List {
    contacts: Vec<contact::Contact>,
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        let mut b = self.contacts.len() == other.contacts.len();
        if b != false {
            for i in 0..self.contacts.len() {
                if b != false {
                    b = self.contacts[i] == self.contacts[i];
                }
            }
        }
        b
    }
}

impl List {
    pub fn new() -> List {
        List{contacts: Vec::new()}
    }

    pub fn create(new_list: &List) -> List {
        let mut new_l = List::new();
        for l in new_list.contacts.clone() {
            let format = l.give_format();
            let c = contact::Contact::create(format.0, format.1, format.2, format.3);
            new_l.contacts.push(c);
        };
        new_l
    }

    pub fn add(&mut self, fname: u8, mname: u8, lname: u8, symbol: char) {
        let c = contact::Contact::create(fname, mname, lname, symbol);
        self.contacts.push(c);
    }
}

#[test]
fn new_test() {
    let l = List::new();
    assert_eq!(List::new(), List{contacts: Vec::new()});
    assert_eq!(l, List::new());
    assert_eq!(l, List{contacts: Vec::new()});
}

#[test]
fn create_test() {
    let mut x = List::new();
    let c = contact::Contact::create(1,2,3,'a');
    x.contacts.push(c.clone());
    let y = List::create(&x);
    assert_eq!(x,y);
    x.contacts.push(c);
    assert_ne!(x,y);
}

#[test]
fn add_test() {
    let mut x = List::new();
    let c = contact::Contact::create(1,2,3,'a');
    x.contacts.push(c.clone());
    let mut y = List::new();
    y.add(1, 2, 3, 'a');
    assert_eq!(x,y);
    y.add(1, 2, 3, 'a');
    assert_ne!(x,y);
    x.add(1, 2, 3, 'a');
    assert_eq!(x,y);c
}