struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangle::new(5, 7);
        let r2 = Rectangle::new(3, 5);

        assert!(r1.can_hold(&r2))
    }
}
