//! # Operator Precedence Grammar Parser
//!
//! `opg` reads an context-free grammar input 
//! and outputs the precedence of the operators.

mod dfs;
mod table;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

///
/// A struct to 
/// represent a production.
///
struct Production {
    /// the left side of the production.
    left: String,
    /// the right side of the production.
    right: Vec<String>,
}

///
/// Generate FIRSTVT set for
/// every non-terminals.
///
/// ## Input
/// `productions` the vector of struct `Production`
///
/// `nts` the non-terminal set, which could be generated 
/// from the function `get_non_terminals()`. 
///
/// ## Return
/// `firstvt` the set of FIRSTVT
/// for all terminals.
///
/// ## Example
/// ```
///  let firstvt:HashMap<String, HashSet<String>> = gen_firstvt(&productions, &nts);
/// ```
///
/// ## Principles
/// Go through every production
/// and search the following patterns:
/// 1. U => U_1y
/// 2. U => U_1Ty
/// 3. U => Ty
///
/// call `dfs::compose_element()` 
/// to get the final result.
fn gen_firstvt(
    productions: &Vec<Production>,
    nts: &HashSet<String>,
) -> HashMap<String, HashSet<String>> {
    let mut firstvtmono: HashMap<String, HashSet<String>> = HashMap::new();
    let mut firstvtcon: HashMap<String, HashSet<String>> = HashMap::new();

    // Find mono terminal and
    // record the containing part
    for p in productions {
        if nts.contains(p.right.first().unwrap()) {
            // Case 1: U => U_1y
            let ntc = firstvtcon
                .entry(p.left.to_string())
                .or_insert(HashSet::new());
            ntc.insert(p.right.first().unwrap().to_string());
            // Case 1*: U => U_1Ty
            if p.right.len() > 1 && !nts.contains(&p.right[1]) {
                let vts = firstvtmono
                    .entry(p.left.to_string())
                    .or_insert(HashSet::new());
                vts.insert(p.right[1].clone());
            }
        } else {
            // Case 2: U => Ty
            let vts = firstvtmono
                .entry(p.left.to_string())
                .or_insert(HashSet::new());
            vts.insert(p.right.first().unwrap().to_string());
        }
    }
    let firstvt = dfs::compose_elements(&firstvtmono, &firstvtcon);
    firstvt
}

///
/// Generate LASTVT set for
/// every non-terminals.
///
/// ## Input
/// `productions` the vector of struct `Production`
///
/// `nts` the non-terminal set, which could be generated 
/// from the function `get_non_terminals()`. 
///
/// ## Return
/// `lastvt` the set of LASTVT
/// for all terminals.
///
/// ## Example
/// ```
///  let lastvt:HashMap<String, HashSet<String>> = gen_lastvt(&productions, &nts);
/// ```
///
/// ## Principles
/// Go through every production
/// and search the following patterns:
/// 1. U => xU_1
/// 2. U => xTU_1
/// 3. U => xT
///
/// call `dfs::compose_element()` 
/// to get the final result.
fn gen_lastvt(
    productions: &Vec<Production>,
    nts: &HashSet<String>,
) -> HashMap<String, HashSet<String>> {
    let mut lastvtmono: HashMap<String, HashSet<String>> = HashMap::new();
    let mut lastvtcon: HashMap<String, HashSet<String>> = HashMap::new();

    // Find mono terminal and
    // record the containing part
    for p in productions {
        if nts.contains(p.right.last().unwrap()) {
            // Case 1: U => xU_1
            let ntc = lastvtcon
                .entry(p.left.to_string())
                .or_insert(HashSet::new());
            ntc.insert(p.right.last().unwrap().to_string());
            // Case 1*: U => xTU_1
            if p.right.len() > 1 && !nts.contains(&p.right[p.right.len() - 2]) {
                let vts = lastvtmono
                    .entry(p.left.to_string())
                    .or_insert(HashSet::new());
                vts.insert(p.right[p.right.len() - 2].clone());
            }
        } else {
            // Case 2: U => xT
            let vts = lastvtmono
                .entry(p.left.to_string())
                .or_insert(HashSet::new());
            vts.insert(p.right.last().unwrap().to_string());
        }
    }
    let lastvt = dfs::compose_elements(&lastvtmono, &lastvtcon);
    lastvt
}

///
/// Find the equal operators
///
fn find_eq(
    table: &mut table::OpTable,
    productions: &Vec<Production>,
    nts: &HashSet<String>,
) {
    for p in productions {
        let mut pe = p.right.clone();
        pe.retain(|x| !nts.contains(x));
        for i in 0..pe.len() {
            for j in i + 1..pe.len() {
                table.insert(&(pe[i].clone(), pe[j].clone()), '=');
            }
        }
    }
}

///
/// Find the less relations
///
fn find_less(
    table: &mut table::OpTable,
    productions: &Vec<Production>,
    nts: &HashSet<String>,
    firstvt: &HashMap<String, HashSet<String>>,
) {
    for p in productions {
        if p.right.len() <= 1 {
            continue;
        }
        for i in 0..p.right.len() - 1 {
            if !nts.contains(&p.right[i]) && nts.contains(&p.right[i + 1]) {
                for t in firstvt[&p.right[i + 1]].iter() {
                    table.insert(&(p.right[i].to_owned(), t.to_owned()), '<');
                }
            }
        }
    }
}

///
/// Find the greater relations
///
fn find_greater(
    table: &mut table::OpTable,
    productions: &Vec<Production>,
    nts: &HashSet<String>,
    lastvt: &HashMap<String, HashSet<String>>,
) {
    for p in productions {
        if p.right.len() <= 1 {
            continue;
        }
        for i in 0..p.right.len() - 1 {
            if nts.contains(&p.right[i]) && !nts.contains(&p.right[i + 1]) {
                for t in lastvt[&p.right[i]].iter() {
                    table.insert( &(t.to_owned(), p.right[i + 1].to_owned()), '>');
                }
            }
        }
    }
}

///
/// Generate production list for
/// the grammar contents.
///
fn gen_productions(contents: &String) -> Vec<Production> {
    let mut p: Vec<Production> = Vec::new();
    for line in contents.lines() {
        let ps: Vec<_> = line.split("->").collect();
        let ls = ps[0].trim();
        let rs: Vec<_> = ps[1].split("|").collect();
        for rsp in rs {
            let vs: Vec<_> = rsp.split_whitespace().collect();
            p.push(Production {
                left: ls.to_string(),
                right: vs.iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    p
}

///
/// Get all the non terminals from
/// the generated production.
///
fn get_non_terminals(productions: &Vec<Production>) -> HashSet<String> {
    productions.iter().map(|s| s.left.clone()).collect()
}

///
/// Get terminals
///
fn get_terminals(productions: &Vec<Production>, nts: &HashSet<String>) -> HashSet<String> {
    let mut ts: HashSet<String> = HashSet::new();
    for p in productions {
        for v in p.right.iter() {
            if !nts.contains(v) {
                ts.insert(v.clone());
            }
        }
    }
    ts
}

///
/// Generate Operator Precedence Table
/// for context-free grammar contents.
///
fn opg_generate(contents: &String) {
    let mut productions: Vec<Production> = gen_productions(&contents);
    let nts = get_non_terminals(&productions);
    let firstvt = gen_firstvt(&productions, &nts);
    let lastvt = gen_lastvt(&productions, &nts);

    // add the $E$ for the starting non-terminal
    let startnt = productions[0].left.clone();
    productions.push(Production {
        left: startnt.to_string(),
        right: vec!["$".to_string(), startnt, "$".to_string()],
    });

    let ts = get_terminals(&productions, &nts);
    let mut table = table::OpTable::new(ts.clone());

    // if there is conflict on operator precedence,
    // then the grammar is ambiguous.
    find_eq(&mut table, &productions, &nts);
    find_less(&mut table, &productions, &nts, &firstvt);
    find_greater(&mut table, &productions, &nts, &lastvt);

    print!("{}", table);
    fs::write("output.txt", &table.to_string()).expect("Cannot output file!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // File for input
    if args.len() < 2 {
        panic!("Please follow a file name!");
    }
    let filename = &args[1];
    // Contents of the file
    let contents = fs::read_to_string(filename).expect("No such file.");
    // Get the table
    opg_generate(&contents);
}
