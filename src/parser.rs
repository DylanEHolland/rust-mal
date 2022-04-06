use std::vec::Vec;
use crate::subroutines;

pub struct TreeNode {
    token: char,
    sub_tree: Vec::<TreeNode>,
    is_tree: bool
}

pub fn build_tree(tokens: Vec::<char>) -> Vec::<TreeNode> {
    let mut tree = Vec::<TreeNode>::new();
    let working_tree = tokens.to_vec();
    let mut counter = 0;

    loop {
        let token = tokens.to_vec()[counter];
        if counter >= tokens.to_vec().len() {
            break;
        }

        let mut node = TreeNode {token: '\0', sub_tree: Vec::<TreeNode>::new(), is_tree: false};

        if token == ')' {
            return tree;
        } else if token == '(' {
            println!("test: {}", token);
            subroutines::char_vec_offset(tokens.to_vec(), counter);
        } else {
            node.token = token;
        }

        // if token == '(' {
        //     println!("test: opening");
        //     let mut sub_tokens = Vec::<char>::new();
        //     let mut sub_counter = counter + 1;
            
        //     loop {
        //         if sub_counter >= working_tree.len() {
        //             break;
        //         }

        //         sub_tokens.push(working_tree[sub_counter]);
        //         sub_counter += 1;
        //     }

        //     let sub_tree = build_tree(sub_tokens);
        //     let len = sub_tree.len();
        //     let node = TreeNode {
        //         token: '\0',
        //         sub_tree: sub_tree,
        //         is_tree: true
        //     };

        //     println!("test opening: {}, {}", counter, len);
        //     counter += len + 1; // acount for increment to subcounter
        //     tree.push(node);

        // } else if token == ')' {
        //     println!("test: {}", working_tree.len());
        //     return tree;
        // } else {
        //     //println!("{}", token);
        //     let node = TreeNode {
        //         token: token,
        //         sub_tree: Vec::<TreeNode>::new(),
        //         is_tree: false
        //     };
        //     tree.push(node);
        //     counter += 1;
        // }

        counter += 1;
    }

    return tree
}

pub fn dump_tree(tree: Vec::<TreeNode>, prepend: String) {
    for item in tree {
        if item.is_tree {
            dump_tree(item.sub_tree, format!("{}->", prepend));
        } else {
            println!("{} {}", prepend, item.token);
        }
    }
}

pub fn parse(line: &str) {
    let tokens = tokenize(line);
    let tree = build_tree(tokens);
    dump_tree(tree, format!(""));
}

pub fn tokenize(line: &str) -> Vec::<char> {
    let mut tokens = Vec::<char>::new();
    
    for c in line.chars() {        
        tokens.push(c);
    }

    return tokens;
}

pub fn tree_vec_offset(tree: Vec::<TreeNode>, mut offset: u16) -> Vec::<TreeNode> {
    let mut buffer = Vec::<TreeNode>::new();
    loop {
        offset += 1;
        println!("test {}", tree.len());
        break;
    }

    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_tree_vec_offset() {
    //     let mut buffer = Vec::<TreeNode>::new();
    //     buffer.push(TreeNode { token: 'A', is_tree: false, sub_tree: Vec::<TreeNode>::new() });
    //     buffer.push(TreeNode { token: 'B', is_tree: false, sub_tree: Vec::<TreeNode>::new() });
    //     buffer.push(TreeNode { token: 'C', is_tree: false, sub_tree: Vec::<TreeNode>::new() });
    //     buffer.push(TreeNode { token: 'D', is_tree: false, sub_tree: Vec::<TreeNode>::new() });
    //     buffer.push(TreeNode { token: 'E', is_tree: false, sub_tree: Vec::<TreeNode>::new() });

    //     let result = tree_vec_offset(buffer, 1);
    //     for t in result {
    //         println!("test: {}", t.token);
    //     }
    // }

    // #[test]
    // fn test_build_tree_single_list() {
    //     let tokens = tokenize("(+ 1 2)");
    //     let _tree = build_tree(tokens);

    //     assert_eq!(_tree[0].is_tree, true);
    //     assert_eq!(_tree[0].sub_tree[0].token, '+');
    //     assert_eq!(_tree[0].sub_tree[1].token, ' ');
    //     assert_eq!(_tree[0].sub_tree[2].token, '1');
    //     assert_eq!(_tree[0].sub_tree[3].token, ' ');
    //     assert_eq!(_tree[0].sub_tree[4].token, '2');
    // }

    #[test]
    fn test_build_tree_double_list() {
        //let tokens = tokenize("(+ 2 1) (- 3 4)");
        let tokens = tokenize("() ()");
        let tree = build_tree(tokens);

        assert_eq!(tree[0].is_tree, true);
        assert_eq!(tree[1].is_tree, true);
    }


    
    //#[test]
    // fn test_build_tree_medium() {
    //     let tokens = tokenize("(+ (+ 1 2) 3) (- 2 (+ 1 4))");
    // }
}