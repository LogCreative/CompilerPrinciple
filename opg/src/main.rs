use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

enum RELATION {
    EQUAL,
    LESS,
    GREATER,
}

struct Production {
    left: String,
    right: Vec<String>,
}

struct Dfs {
    con: HashMap<String, HashSet<String>>,
    visited: HashSet<String>,
    element: HashMap<String, i64>,
    category: HashMap<i64, HashSet<String>>,
    top: i64,
}

impl Dfs {
    fn new(con: HashMap<String, HashSet<String>>) -> Dfs {
        Dfs {
            con: con,
            visited: HashSet::new(),
            element: HashMap::new(),
            category: HashMap::new(),
            top: -1,
        }
    }

    ///
    /// DFS(Depth-First Search)
    /// on the structure of
    /// Dfs
    ///
    fn dfs(&mut self, node: String) {
        // stack-based DFS
        // path is in the stack
        let mut dfs_stack: Vec<String> = Vec::new();
        dfs_stack.push(node);

        while dfs_stack.len() > 0 {
            let father = dfs_stack.pop().unwrap();

            let mut first = dfs_stack.len();
            for (pos, el) in dfs_stack.iter().enumerate() {
                if el.eq(&father) {
                    first = pos;
                }
            }

            println!("{},{:?}",father,dfs_stack);
            
            self.top += 1;
            let cate: i64 = self.top;
            if first < dfs_stack.len() {
                // if father is in the stack
                // then there is a loop in the path
                for i in first..dfs_stack.len() - 1 {
                    if self.element.contains_key(&dfs_stack[i]) {
                        let mono = self.category.entry(self.element[&dfs_stack[i]]).or_default();
                        // move to the same new category.
                        for el in mono.to_owned() {
                            self.element.insert(el.clone(), cate);
                        }
                        // clear the original category.
                        mono.clear();
                    }
                }
            } else if !self.element.contains_key(&father){
                // otherwise, it is a new category if it is not recorded.
                self.element.insert(father.clone(), cate);
                let newcate = self.category.entry(cate).or_insert(HashSet::new());
                newcate.insert(father.clone());
            } else {
                self.top -= 1;
            }
            
            // DFS pre visited -- do not visit again
            // if it is a loop, it won't pass the condition test.
            if !self.visited.contains(&father) {
                // pre visited
                self.visited.insert(father.clone());
                if self.con.contains_key(&father) {
                    for child in self.con[&father].clone() {
                        dfs_stack.push(child);
                    }
                }
            }
        }
    }
}

fn compose_elements(
    mono: &HashMap<String, HashSet<String>>,
    con: &HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    // Eliminate loop on
    // the containing recursive tree
    // by using division method.
    // DFS, the element on the loop
    // shares the same set.
    println!("{:?}", mono);
    println!("{:?}", con);

    let mut dfs_div = Dfs::new(con.clone());
    for nt in con.keys() {
        dfs_div.dfs(nt.clone());
    }
    println!("{:?}",dfs_div.element);
    println!("{:?}",dfs_div.category);

    // Compose all the sets
    // by recursing.
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    map
}

///
/// Generate FIRSTVT set for
/// every non-terminals.
///
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
    let firstvt = compose_elements(&firstvtmono, &firstvtcon);
    firstvt
}

///
/// Generate LASTVT set for
/// every non-terminals.
///
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
    let lastvt = compose_elements(&lastvtmono, &lastvtcon);
    lastvt
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
        right: vec!["$".to_string(), startnt, "$".to_string()],
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
fn find_eq(table: &mut HashMap<(String, String), RELATION>, productions: &Vec<Production>) {}

///
/// Find the less relations
///
fn find_less(
    table: &mut HashMap<(String, String), RELATION>,
    productions: &Vec<Production>,
    firstvt: &HashMap<String, HashSet<String>>,
) {
}

///
/// Find the greater relations
///
fn find_greater(
    table: &mut HashMap<(String, String), RELATION>,
    productions: &Vec<Production>,
    lastvt: &HashMap<String, HashSet<String>>,
) {
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
    let mut table: HashMap<(String, String), RELATION> = HashMap::new();

    // if there is conflict on operator precedence,
    // then the grammar is ambiguous.
    // TODO: panic error
    find_eq(&mut table, &productions);
    find_less(&mut table, &productions, &firstvt);
    find_greater(&mut table, &productions, &lastvt);

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
