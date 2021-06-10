use std::env;
use std::fs;

struct Production {
    left: String,
    right: Vec<String>
}

struct Opg {
    contents: String,
    productions: Vec<Production>
}

impl Opg {
    pub fn new(_contents: String) -> Opg {
        let mut prod:Vec<Production> = Vec::new();
        for line in _contents.lines(){
            let ps:Vec<_> = line.split("->").collect();
            let vs:Vec<_> = ps[1].split_whitespace().collect();
            prod.push(Production {
                left: ps[0].trim().to_string(),
                right: vs.iter().map(|s| s.clone().to_string()).collect()
            });
        }
        Opg{
            contents: _contents,
            productions: prod
        }
    }

    pub fn generate(&self) -> String {
        let prod = &self.productions;
        for prod in prod{
            println!("{}->{:?}",prod.left,prod.right);
        }
        let table = self.contents.clone();
        table
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // File for input
    let filename = &args[1];
    // Contents of the file
    let _contents = fs::read_to_string(filename).expect("No such file."); 
    // Define the struct
    let opg = Opg::new(_contents);
    // Get the table
    let table = opg.generate();
    // Output the table
    println!("{}", table);
}
