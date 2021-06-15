use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

pub struct OpTable{
    table: HashMap<(String, String), char>,
    ts: HashSet<String>
}

impl OpTable {
    pub fn new(ts: HashSet<String>) -> OpTable {
        OpTable {
            table: HashMap::new(),
            ts: ts
        }
    }

    ///
    /// Insert to table
    ///
    pub fn insert(&mut self, ttuple: &(String, String), ch: char) {
        if self.table.contains_key(&ttuple) && self.table[&ttuple] != ch {
            println!("The grammar is ambiguous.");
            panic!("Ambiguous grammar detected.");
        }
        self.table.insert(ttuple.clone(), ch);
    }

    ///
    /// Convert table to string
    ///
    pub fn to_string(&self) -> String{
        let mut output:String = "".to_owned();
        output = output + " \t";
        for j in self.ts.iter() {
            output  = output + j + "\t";
        }
        output  = output + "\n";

        for i in self.ts.iter() {
            output  = output + i + "\t";
            for j in self.ts.iter() {
                let ttuple = (i.clone(), j.clone());
                if self.table.contains_key(&ttuple) {
                    output = output + &self.table[&ttuple].to_string() + "\t";
                } else {
                    output = output + " \t";
                }
            }
            output = output + "\n";
        }
        output
    }

}

impl Display for OpTable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let output = self.to_string();
        write!(f, "{}", output)
    }    
}
