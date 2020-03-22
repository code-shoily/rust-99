#[allow(dead_code)]
/// P01 (*) Find the last element of a list.
pub mod solutions {
    /// Using `last` function to find the last element
    pub fn last_element_1<T: Copy>(lst: &[T]) -> Option<T> {
        lst.last().cloned()
    }

    /// Using indexing to find the last element
    pub fn last_element_2<T: Copy>(lst: &[T]) -> Option<T> {
        if lst.is_empty() {
            None
        } else {
            let last_index = lst.len() - 1;
            Some(lst[last_index])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::{last_element_1, last_element_2};

    #[test]
    fn last_element_1_works() {
        let elements = vec![1,2,3];
        let output = last_element_1(&elements);
        match output {
            Some(3) => assert!(true),
            _ => assert!(false)
        }

        let elements: Vec<u32> = Vec::new();
        match last_element_1(&elements) {
            Some(_) => assert!(false),
            _ => assert!(true)
        }
    }

    #[test]
    fn last_element_2_works() {
        let elements = vec!["A","B","C"];
        match last_element_2(&elements) {
            Some("C") => assert!(true),
            _ => assert!(false)
        }
        let elements: Vec<u32> = Vec::new();
        match last_element_2(&elements) {
            Some(_) => assert!(false),
            _ => assert!(true)
        }
    }
}