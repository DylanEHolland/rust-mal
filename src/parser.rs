use std::vec::Vec;

pub struct TreeNode {
    token: char,
    sub_tree: Vec::<TreeNode>,
    is_tree: bool
}

pub fn build_tree(tokens: Vec::<char>) -> Vec::<TreeNode> {
    let mut tree = Vec::<TreeNode>::new();
    let working_tree = tokens.to_vec();
    let mut counter = 0;

    for token in tokens {
        if token == '(' {
            let mut sub_tokens = Vec::<char>::new();
            let mut sub_counter = counter + 1;
            
            loop {
                if sub_counter >= working_tree.len() {
                    break;
                }

                sub_tokens.push(working_tree[sub_counter]);
                sub_counter += 1;
            }

            let sub_tree = build_tree(sub_tokens);
            let len = sub_tree.len();
            let node = TreeNode {
                token: '\0',
                sub_tree: sub_tree,
                is_tree: true
            };

            counter += len;
            tree.push(node);
        } else if token == ')' {
            return tree;
        } else {
            let node = TreeNode {
                token: token,
                sub_tree: Vec::<TreeNode>::new(),
                is_tree: false
            };

            tree.push(node);
        }

        counter += 1;
    }

    return tree;
}

pub fn dump_tree(tree: Vec::<TreeNode>) {
    for item in tree {
        if item.is_tree {
            dump_tree(item.sub_tree);
        } else {
            println!("{}", item.token);
        }
    }
}

pub fn parse(line: &str) {
    let tokens = tokenize(line);
    let tree = build_tree(tokens);
    dump_tree(tree);
}

pub fn tokenize(line: &str) -> Vec::<char> {
    let mut tokens = Vec::<char>::new();
    
    for c in line.chars() {        
        tokens.push(c);
    }

    return tokens;
}