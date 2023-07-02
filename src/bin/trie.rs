use std::collections::HashMap;

pub fn main() {
    println!("Implement Trie");
}

struct TrieNode {
    // Char isn't for this case but it's the used one since in reality we use words form different languages
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for ch in word.chars() {
            let next_node = current_node.children.entry(ch).or_insert(TrieNode::new());

            current_node = next_node;
        }

        current_node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for ch in word.chars() {
            match current_node.children.get(&ch) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        current_node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;

        for ch in prefix.chars() {
            match current_node.children.get(&ch) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        true
    }
}
