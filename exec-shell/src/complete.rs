use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &Vec<char>) {
        let mut current_node = &mut self.root;

        for c in word.iter() {
            current_node.insert(*c);
            current_node = current_node.children.get_mut(c).unwrap();
        }
        current_node.is_end = true
    }

    pub fn match_prefix(&self, prefix: &Vec<char>) -> Option<Vec<Vec<char>>> {
        match self.find(prefix) {
            None => return None,
            Some(node) => {
                return Some(recursive_search(
                    &node,
                    &node.children.clone().into_keys().collect(),
                    &vec![],
                    &vec![]
                ))
            }
        }
        fn recursive_search(
            upper: &TrieNode,
            childrens: &Vec<char>,
            acc: &Vec<Vec<char>>,
            upper_acc: &Vec<char>,
        ) -> Vec<Vec<char>> {
            if childrens.len() == 0 {
                acc.to_vec()
            } else {
                let (current, neibours) = my_pop(&childrens);
                println!("current node: {:?}", current);
                let current_node = &upper.children[&current];
                if current_node.is_end && current_node.children.len() == 0 {
                    recursive_search(
                        upper,
                        &neibours,
                        &my_push(&acc, my_push(&upper_acc, current)),
                        upper_acc
                    )

                } else {
                    let current_acc = recursive_search(
                        current_node,
                        &neibours,
                        &acc,
                        &my_push(&upper_acc, current)
                    );
                    if current_node.is_end {
                        recursive_search(
                            upper,
                            &neibours,
                            &my_push(&acc, my_push(&upper_acc, current)),
                            upper_acc
                        )
                    } else {
                        recursive_search(
                            upper, &neibours, &current_acc, upper_acc)
                    }
                }
                //return vec![vec![]];
            }
        }

        fn my_pop<T>(vec: &Vec<T>) -> (T, Vec<T>)
        where
            T: Clone,
        {
            let mut vec_copy = vec.clone();
            (vec_copy.pop().unwrap(), vec_copy)
        }

        fn my_push<T>(vec: &Vec<T>, elem: T) -> Vec<T>
        where
            T: Clone,
        {
            let mut vec_copy = vec.clone();
            vec_copy.push(elem);
            vec_copy
        }
    }

    fn find(&self, word: &Vec<char>) -> Option<TrieNode> {
        let mut current_node = &self.root;

        for c in word.iter() {
            if current_node.children.contains_key(&c) {
                current_node = current_node.children.get(&c).unwrap();
            } else {
                return None;
            }
        }
        Some(current_node.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub is_end: bool,
    pub children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, c: char) {
        if self.children.contains_key(&c) {
            ();
        } else {
            self.children.entry(c).or_insert(TrieNode::new());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_trie() -> Trie {
        let word1 = vec!['a', 'b', 'c'];
        let word2 = vec!['x', 'y', 'z'];
        let word3 = vec!['a', 'd', 'b'];
        let word4 = vec!['a', 'd'];
        let mut trie = Trie::new();

        trie.insert(&word1);
        trie.insert(&word2);
        trie.insert(&word3);
        trie.insert(&word4);

        trie
    }
    #[test]
    #[ignore]
    fn test_insert() {
        let trie = init_trie();
        println!("{:?}", trie);
    }

    #[test]
    #[ignore]
    fn test_find() {
        let trie = init_trie();
        let found = trie.find(&vec!['a', 'd']);
        println!("{:#?}", found);
        assert!(found.is_some());
        assert!(trie.find(&vec!['a', 'd', 'c']).is_none());
    }

    #[test]
    fn test_match_prefix() {
        let trie = init_trie();
        let maching = trie.match_prefix(&vec!['a']);
        println!("{:#?}", maching);
    }
}
