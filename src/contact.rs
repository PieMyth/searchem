#[derive(Debug, Clone)]
pub struct Contact{
    fname: u8,
    mname: u8,
    lname: u8,
    symbol: char, 
}

impl PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.fname == other.fname && self.mname == other.mname && self.lname == other.lname && self.symbol == other.symbol
    }
}

impl Contact{
    pub fn new() -> Contact{
        Contact {fname: 0, mname: 0, lname: 0, symbol: ' '}
    }

    pub fn create(f: u8, m: u8, l: u8, s: char) -> Contact {
        Contact {fname: f, mname: m, lname: l, symbol: s}
    }

    pub fn changeformat(&mut self, fname: u8, mname: u8, lname: u8, symbol: char) {
        self.fname = fname;
        self.mname = mname;
        self.lname = lname;
        self.symbol = symbol;
    }

    pub fn give_format(&self) -> (u8, u8, u8, char) {
        (self.fname, self.mname, self.lname, self.symbol)
    }

}

#[test]
fn new_test(){
    let c = Contact::new();
    assert_eq!(c, Contact{fname: 0, mname: 0, lname: 0, symbol: ' '});
    assert_eq!(c, Contact::new());
    assert_eq!(Contact::new(), Contact::new());
}

#[test]
fn create_test() {
    let c = Contact::create(1, 2, 3, 'a');
    assert_eq!(c, Contact{fname: 1, mname: 2, lname: 3, symbol: 'a'});
}

#[test]
fn change_format_test() {
    let mut x = Contact::new();
    let mut y = Contact::new();
    x.changeformat(1,2,3,'a');
    assert_eq!(x, Contact{fname: 1, mname: 2, lname: 3, symbol: 'a'});
    assert_ne!(x,y);
    y.changeformat(1,2,3,'a');
    assert_eq!(y, Contact{fname: 1, mname: 2, lname: 3, symbol: 'a'});
    assert_eq!(x,y);
}

#[test]
fn give_format_test() {
    let mut c = Contact::new();
    assert_eq!(c.give_format(), (0,0,0,' '));
    c.changeformat(1, 2, 3, 'a');
    assert_eq!(c.give_format(), (1,2,3,'a'));

}