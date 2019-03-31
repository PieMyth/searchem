#[derive(Debug)]
pub struct Contact{
    fname: u8,
    mname: u8,
    lname: u8,
    symbols: char, 
}

impl PartialEq for Contact {
    fn eq(&self, other: &Contact) -> bool {
        self.fname == other.fname && self.mname == other.mname && self.lname == other.lname && self.symbols == other.symbols
    }
}

impl Contact{
    pub fn new() -> Contact{
        Contact {fname: 0, mname: 0, lname: 0, symbols: ' '}
    }

    pub fn changeformat(&mut self, fname: u8, mname: u8, lname: u8, symbols: char) {
        self.fname = fname;
        self.mname = mname;
        self.lname = lname;
        self.symbols = symbols;
    }

}

    #[test]
    fn new_test(){
        let c = Contact::new();
        assert_eq!(c, Contact{fname: 0, mname: 0, lname: 0, symbols: ' '});
        assert_eq!(c, Contact::new());
        assert_eq!(Contact::new(), Contact::new());
    }

    #[test]
    fn change_format_test() {
        let mut x = Contact::new();
        let mut y = Contact::new();
        x.changeformat(1,2,3,'a');
        assert_eq!(x, Contact{fname: 1, mname: 2, lname: 3, symbols: 'a'});
        assert_ne!(x,y);
        y.changeformat(1,2,3,'a');
        assert_eq!(y, Contact{fname: 1, mname: 2, lname: 3, symbols: 'a'});
        assert_eq!(y,y);
    }