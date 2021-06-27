//! # table
//!
//! `table` provides `OpTable`
//! struct to make insert, output
//! operation on the operator table.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

///
/// A struct of Operation Table.
///
pub struct OpTable{
    /// The hashmap of relation.
    table: HashMap<(String, String), char>,
    /// The terminal set in the grammar.
    ts: HashSet<String>
}

impl OpTable {
    ///
    /// Create a new `opTable`.
    ///
    pub fn new(ts: HashSet<String>) -> OpTable {
        OpTable {
            table: HashMap::new(),
            ts: ts
        }
    }

    ///
    /// Insert to table.
    ///
    /// ## Principles
    /// Try to insert the a tuple with `ch` relation.
    /// If it is occupied and not equal to the relation
    /// to be inserted, then the grammar is ambiguous
    /// and the panic will be fired.
    ///
    pub fn insert(&mut self, ttuple: &(String, String), ch: char) {
        if self.table.contains_key(&ttuple) && self.table[&ttuple] != ch {
            println!("The grammar is ambiguous.");
            panic!("Ambiguous grammar detected.");
        }
        self.table.insert(ttuple.clone(), ch);
    }

    ///
    /// Convert table to string.
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
    ///
    /// Define the behavior of outputting
    /// an `opTable` struct.
    ///
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let output = self.to_string();
        write!(f, "{}", output)
    }    
}
