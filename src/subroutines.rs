use std::vec::Vec;

pub fn char_vec_offset(tree: Vec::<char>, mut offset: usize) -> Vec::<char> {
    let mut buffer = Vec::<char>::new();

    loop {
        if offset >= tree.len() {
            break;
        }

        buffer.push(tree[offset]);
        offset += 1;
    }

    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_vec_offset() {
        let mut buffer = Vec::<char>::new();
        buffer.push('A');
        buffer.push('B');
        buffer.push('C');
        buffer.push('D');
        buffer.push('E');

        
        let result = char_vec_offset(buffer, 1);
        assert_eq!(result[0], 'B');
    }
}