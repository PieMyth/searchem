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

    pub fn addc(&mut self, c: contact::Contact) {
        self.contacts.push(c);
    }

    pub fn re_order(&mut self, current_index: usize, new_index: usize) {
        let c = self.contacts.remove(current_index);
        self.contacts.insert(new_index, c);
    }

    pub fn insert(&mut self, desired_index: usize, c: contact::Contact) {
        if self.contacts.len() < desired_index {
            self.addc(c);
        }
        else {
            self.contacts.insert(desired_index, c);
        }
    }

    pub fn remove(&mut self, desired_index: usize) -> contact::Contact{
        if self.contacts.len() <= desired_index {
            contact::Contact::create(0,0,0,'a')
        }
        else {
            self.contacts.remove(desired_index)
        }
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
    assert_eq!(x,y);
}

#[test]
fn addc_test() {
    let mut x = List::new();
    let c = contact::Contact::create(1,2,3,'a');
    x.addc(c.clone());
    assert_eq!(x.contacts.remove(0), c);
}

#[test]
fn re_order_test() {
    let mut x = List::new();
    let c = contact::Contact::create(1,2,3,'a');
    x.contacts.push(c.clone());
    x.add(1, 2, 3, 'b');
    x.add(1, 2, 3, 'c');
    x.add(1, 2, 3, 'd');
    x.re_order(0, 3);
    let c = x.contacts.remove(3);
    assert_eq!(c, contact::Contact::create(1, 2, 3, 'a'));
    x.contacts.insert(3, c);
    assert_eq!(x.contacts.remove(3), contact::Contact::create(1, 2, 3, 'a'));
}

#[test]
fn insert_test() {
    let mut x = List::new();
    let c = contact::Contact::create(1,2,3,'a');
    x.insert(100, c.clone());
    assert_eq!(x.contacts.remove(0), c);
    x.addc(c);
    let c = contact::Contact::create(1,2,3,'b');
    x.insert(0, c.clone());
    assert_eq!(x.contacts.remove(0), c);
}

#[test]
fn remove_test() {
    let mut x = List::new();
    assert_eq!(x.remove(0), contact::Contact::create(0, 0, 0, 'a'));
    let c = contact::Contact::create(1,2,3,'a');
    x.addc(c.clone());
    assert_eq!(x.remove(0), c);
    x.addc(c.clone());
    x.addc(c.clone());
    assert_eq!(x.remove(2), contact::Contact::create(0, 0, 0, 'a'));
    assert_eq!(x.remove(0), c);
}