pub fn main() {
    println!("Word Dictionary");
}

#[derive(Default)]
pub struct TrieNode {
    childern: [Option<Box<TrieNode>>; 26],
    end: bool,
}

#[derive(Default)]
pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;

        for &c in word.as_bytes() {
            node = node.childern[(c - b'a') as usize].get_or_insert(Box::new(TrieNode::default()));
        }
        node.end = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::search_intern(&self.root, &word.as_bytes())
    }

    fn search_intern(node: &TrieNode, word: &[u8]) -> bool {
        if let Some(&ch) = word.first() {
            if ch == b'.' {
                for child in &node.childern {
                    if let Some(nd) = child {
                        if Self::search_intern(&nd, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else if let Some(nd) = &node.childern[(ch - b'a') as usize] {
                return Self::search_intern(&nd, &word[1..]);
            }

            return false;
        } else {
            node.end
        }
    }
}
