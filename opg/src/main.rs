use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

struct Production {
    left: String,
    right: Vec<String>,
}

struct Dfs {
    mono: HashMap<String, HashSet<String>>,
    con: HashMap<String, HashSet<String>>,
    visited: HashSet<String>,
    path: Vec<String>,
    element: HashMap<String, i64>,
    category: HashMap<i64, HashSet<String>>,
    tree: HashMap<i64, HashSet<i64>>,
    top: i64,
}

impl Dfs {
    fn new(mono: HashMap<String, HashSet<String>>, con: HashMap<String, HashSet<String>>) -> Dfs {
        Dfs {
            mono: mono,
            con: con,
            visited: HashSet::new(),
            path: Vec::new(),
            element: HashMap::new(),
            category: HashMap::new(),
            tree: HashMap::new(),
            top: -1,
        }
    }

    fn dfs(&mut self) -> HashMap<String, HashSet<String>> {
        // The first DFS: Merge category
        for nt in self.mono.to_owned().keys() {
            self.dfs_merge(nt.clone());
        }
        // The second DFS: Establish the connection between categories.
        self.visited.clear();
        self.path.clear();
        for nt in self.mono.to_owned().keys() {
            self.dfs_conn(nt.clone(), None);
        }
        // The third DFS: get the map from the root node.
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for nt in self.mono.to_owned().keys() {
            self.visited.clear();
            self.path.clear();
            let mapnt = map.entry(nt.to_owned()).or_insert(HashSet::new());
            self.dfs_map(nt.clone(), mapnt);
        }
        map
    }

    ///
    /// Merge DFS
    ///
    fn dfs_merge(&mut self, node: String) {
        self.merge(node.clone());

        // DFS pre visited -- do not visit again
        // if it is a loop, it won't pass the condition test.
        if !self.visited.contains(&node) {
            // pre visited
            self.visited.insert(node.clone());
            self.path.push(node.clone());
            if self.con.contains_key(&node) {
                for child in self.con[&node].clone() {
                    self.dfs_merge(child);
                }
            }
            // post visited
            self.path.pop();
        }
    }

    ///
    /// connection DFS
    ///
    fn dfs_conn(&mut self, node: String, parent: Option<String>) {
        // DFS pre visited -- do not visit again
        // no looped element is added in the next time.
        if !self.path.contains(&node) && parent.is_some() {
            let nodecate = self.element[&parent.unwrap()].clone();
            let children = self.tree.entry(nodecate).or_insert(HashSet::new());
            children.insert(self.element[&node]);
        }
        if !self.visited.contains(&node) {
            // pre visited
            self.visited.insert(node.clone());
            self.path.push(node.clone());
            if self.con.contains_key(&node) {
                for child in self.con[&node].clone() {
                    self.dfs_conn(child, Some(node.clone()));
                }
            }
            self.path.pop();
        }
    }

    ///
    /// map DFS
    ///
    fn dfs_map(&mut self, node: String, map: &mut HashSet<String>) {
        if !self.path.contains(&node) {
            for v in self.mono[&node].clone() {
                map.insert(v);
            }
        }
        if !self.visited.contains(&node) {
            // pre visited
            self.visited.insert(node.clone());
            self.path.push(node.clone());
            if self.con.contains_key(&node) {
                for child in self.con[&node].clone() {
                    self.dfs_map(child, map);
                }
            }
            self.path.pop();
        }
    }

    ///
    /// Merge New Category
    ///
    fn merge(&mut self, node: String) {
        let mut first = self.path.len();
        for (pos, el) in self.path.iter().enumerate() {
            if el.eq(&node) {
                first = pos;
            }
        }
        self.top += 1;
        let cate: i64 = self.top;
        if first < self.path.len() {
            // there is a loop in the path
            for i in first..self.path.len() {
                // if self.element.contains_key(&self.path[i]) {
                // move to the same new category.
                let oldnum = self.element[&self.path[i]];
                let oldcate = self.category[&oldnum].clone();
                let newcate = self.category.entry(cate).or_insert(HashSet::new());
                for el in oldcate {
                    self.element.insert(el.clone(), cate);
                    newcate.insert(el.clone());
                }
                // clear the original category.
                self.category.remove(&oldnum);
                // }
            }
        } else if !self.element.contains_key(&node) {
            // otherwise, it is a new category if it is not recorded.
            self.element.insert(node.clone(), cate);
            let newcate = self.category.entry(cate).or_insert(HashSet::new());
            newcate.insert(node.clone());
        } else {
            self.top -= 1;
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
    let mut dfs_div = Dfs::new(mono.clone(), con.clone());
    dfs_div.dfs()
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
/// Insert to table
///
fn insert_table(table: &mut HashMap<(String, String), char>, ttuple: &(String, String), ch: char) {
    if table.contains_key(&ttuple) && table[&ttuple] != ch {
        println!("The grammar is ambiguous.");
        panic!("Ambiguous grammar detected.");
    }
    table.insert(ttuple.clone(), ch);
}

///
/// Find the equal operators
///
fn find_eq(
    table: &mut HashMap<(String, String), char>,
    productions: &Vec<Production>,
    nts: &HashSet<String>,
) {
    for p in productions {
        let mut pe = p.right.clone();
        pe.retain(|x| !nts.contains(x));
        for i in 0..pe.len() {
            for j in i + 1..pe.len() {
                insert_table(table, &(pe[i].clone(), pe[j].clone()), '=');
            }
        }
    }
}

///
/// Find the less relations
///
fn find_less(
    table: &mut HashMap<(String, String), char>,
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
                    insert_table(table, &(p.right[i].to_owned(), t.to_owned()), '<');
                }
            }
        }
    }
}

///
/// Find the greater relations
///
fn find_greater(
    table: &mut HashMap<(String, String), char>,
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
                    insert_table(table, &(t.to_owned(), p.right[i + 1].to_owned()), '>');
                }
            }
        }
    }
}

///
/// Print table
///
fn print_table(table: &HashMap<(String, String), char>, ts: &HashSet<String>) {
    print!(" \t");
    for j in ts {
        print!("{}\t", j);
    }
    print!("\n");

    for i in ts {
        print!("{}\t", i);
        for j in ts {
            let ttuple = (i.clone(), j.clone());
            if table.contains_key(&ttuple) {
                print!("{}\t", table[&ttuple]);
            } else {
                print!(" \t");
            }
        }
        print!("\n");
    }
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

    let mut table: HashMap<(String, String), char> = HashMap::new();
    // if there is conflict on operator precedence,
    // then the grammar is ambiguous.
    find_eq(&mut table, &productions, &nts);
    find_less(&mut table, &productions, &nts, &firstvt);
    find_greater(&mut table, &productions, &nts, &lastvt);

    let ts = get_terminals(&productions, &nts);
    print_table(&table, &ts);
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
