use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;

struct Production {
    left: String,
    right: Vec<String>,
}

///
/// Generate FIRSTVT set for
/// every non-terminals.
///
fn gen_firstvt(productions: &Vec<Production>, nt: &HashSet<String>) -> HashMap<String, HashSet<String>>{
    let mut firstvtmap: HashMap<String, HashSet<String>> = HashMap::new();
    firstvtmap
}

///
/// Generate LASTVT set for
/// every non-terminals.
///
fn gen_lastvt(productions: &Vec<Production>, nt: &HashSet<String>) -> HashMap<String, HashSet<String>>{
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
/// Generate Operator Precedence Table
/// for context-free grammar contents.
///
fn opg_generate(contents: &String) -> String {
    let productions: Vec<Production> = gen_productions(&contents);
    let nt = get_non_terminals(&productions);
    let firstvt = gen_firstvt(&productions, &nt);
    let lastvt = gen_lastvt(&productions, &nt);

    for v in nt{
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
