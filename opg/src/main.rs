use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;

enum RELATION {
    EQUAL,
    LESS,
    GREATER
}

struct Production {
    left: String,
    right: Vec<String>,
}

///
/// Generate FIRSTVT set for
/// every non-terminals.
///
fn gen_firstvt(productions: &Vec<Production>, nts: &HashSet<String>) -> HashMap<String, HashSet<String>>{
    let mut firstvtmap: HashMap<String, HashSet<String>> = HashMap::new();
    let mut firstvtcon: HashMap<String, Vec<String>> = HashMap::new();

    // Find mono terminal and
    // record the containing part
    for p in productions {
        if !nts.contains(p.right.first().unwrap()) {
            // Case 1: U => Ty
            let vts = firstvtmap.entry(p.left.to_string()).or_insert(HashSet::new());
            vts.insert(p.right.first().unwrap().to_string());
        } else {
            // Case 2: U => U_1Ty
            
        }
    }

    // Eliminate loop on 
    // the containing recursive tree
    // DFS, the element on the loop
    // shares the same set.

    // Compose all the sets
    // by recursing.

    firstvtmap
}

///
/// Generate LASTVT set for
/// every non-terminals.
///
fn gen_lastvt(productions: &Vec<Production>, nts: &HashSet<String>) -> HashMap<String, HashSet<String>>{
    let mut lastvtmap: HashMap<String, HashSet<String>> = HashMap::new();
    lastvtmap
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
    // add the $E$ for the starting non-terminal
    let startnt = p[0].left.clone();
    p.push(Production {
        left: startnt.to_string(),
        right: vec!["$".to_string(), startnt, "$".to_string()]
    });
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
/// Find the equal operators
///
fn find_eq(table: &mut HashMap<(String,String),RELATION>, productions: &Vec<Production>) {

}

///
/// Find the less relations
///
fn find_less(table: &mut HashMap<(String,String),RELATION>, productions: &Vec<Production>, firstvt: &HashMap<String,HashSet<String>>) {

}

///
/// Find the greater relations
///
fn find_greater(table: &mut HashMap<(String,String),RELATION>, productions: &Vec<Production>, lastvt: &HashMap<String,HashSet<String>>) {

}


///
/// Generate Operator Precedence Table
/// for context-free grammar contents.
///
fn opg_generate(contents: &String) -> String {
    let productions: Vec<Production> = gen_productions(&contents);
    let nts = get_non_terminals(&productions);
    let firstvt = gen_firstvt(&productions, &nts);
    let lastvt = gen_lastvt(&productions, &nts);
    let mut table: HashMap<(String,String),RELATION> = HashMap::new();

    // if there is conflict on operator precedence,
    // then the grammar is ambiguous.
    // TODO: panic error
    find_eq(&mut table, &productions);
    find_less(&mut table, &productions, &firstvt);
    find_greater(&mut table, &productions, &lastvt);

    for v in nts{
        println!("{:?}",v);
    }
    contents.clone()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // File for input
    let filename = &args[1];
    // Contents of the file
    let contents = fs::read_to_string(filename).expect("No such file.");
    // Get the table
    let table = opg_generate(&contents);
    // Output the table
    println!("{}", table);
}
