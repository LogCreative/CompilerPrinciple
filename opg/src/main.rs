use std::env;
use std::fs;

struct Production {
    left: String,
    right: Vec<String>,
}

fn gen_productions(contents: &String) -> Vec<Production> {
    let mut p: Vec<Production> = Vec::new();
    for line in contents.lines() {
        let ps: Vec<_> = line.split("->").collect();
        let rs: Vec<_> = ps[1].split("|").collect();
        for rsp in rs {
            let vs: Vec<_> = rsp.split_whitespace().collect();
            p.push(Production {
                left: ps[0].trim().to_string(),
                right: vs.iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    p
}

fn opg_generate(contents: &String) -> String {
    let productions: Vec<Production> = gen_productions(&contents);
    for prod in productions {
        println!("{}->{:?}", prod.left, prod.right);
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
