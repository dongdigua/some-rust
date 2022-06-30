pub struct Trie<'a> {
    pub is_end_of_world: bool,
    pub sub_nodes: Vec<&'a Trie<'a>> // 编译器听我说屑屑你
}
