///Adds left and right
/// 
/// # Arguments
/// 
/// # `left` - a usize to add
/// # `right` - a usize to add
/// 
/// # Example
/// 
/// ```
/// # use my_library::add;
/// let l: usize = 20;
/// let r: usize = 5;
/// assert_eq!(add(l,r), 20); 
/// ```

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
