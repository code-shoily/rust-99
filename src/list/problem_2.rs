#[allow(dead_code)]
/// (*) Find the last but one element of a list.
pub mod solutions {
    pub fn but_last_1<T>(lst: &[T]) -> Result<&T, i32> {
        match &lst[..] {
            [] => Err(0),
            [_] => Err(1),
            [a, _] => Ok(a),
            otherwise => but_last_1(&otherwise[1..])
        }
    }

    pub fn but_last_2<T>(lst: &[T]) -> Result<&T, i32> {
        match lst.len() {
            0 => Err(0),
            1 => Err(1),
            size => Ok(&lst[size-2])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::{but_last_1, but_last_2};

    #[test]
    fn but_last_1_works() {
        assert_eq!(but_last_1(&vec![1, 2, 3]), Ok(&2));
        assert_eq!(but_last_1(&vec![1]), Err(1));
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(but_last_1(&empty_vec), Err(0));
    }

    #[test]
    fn but_last_2_works() {
        assert_eq!(but_last_2(&vec![1, 2, 3]), Ok(&2));
        assert_eq!(but_last_2(&vec![1]), Err(1));
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(but_last_2(&empty_vec), Err(0));
    }
}