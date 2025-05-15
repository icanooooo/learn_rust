struct Rectangle {
    x: u32,
    y: u32,
}

impl Rectangle {
    fn is_bigger(&self, another: Rectangle) -> bool {
        if self.x > another.x && self.y > another.y {
            true
        } else {
            false
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // Using assert to test boolean values
    #[test]
    fn test_rectangle() {
        let first = Rectangle {x: 32, y: 20};
        let second = Rectangle {x: 20, y: 15};
        
        assert!(first.is_bigger(second));       // Expecting True
                                                // assert will be ok if return is true
    }

    // Using assert to test negative boolean values
    #[test]
    fn test_rectangle_smaller() {
        let first = Rectangle {x: 32, y: 20};
        let second = Rectangle {x: 20, y: 15};
        
        assert!(!second.is_bigger(first));      // Add '!' because we expected false
                                                // assert! only we be ok if it's true
    }

    // using 'assert_eq!' for testing equalit
    #[test]
    fn test_equality() {
        let result = add(1, 2);

        assert_eq!(result, 3);
    }
}
