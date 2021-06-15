use std::collections::HashMap;
use std::collections::HashSet;

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

///
/// Compose the elements from mono and con
///
pub fn compose_elements(
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