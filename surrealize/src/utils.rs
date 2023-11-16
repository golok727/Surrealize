/// Trait for converting a PascalCase string to snake_case.
pub trait PascalToSnake {
    /// Converts the string to snake_case.
    fn to_snake_case(&self) -> String;
}

/// Implementation of the PascalToSnake trait for str.
impl PascalToSnake for str {
    fn to_snake_case(&self) -> String {
        self.chars()
            .enumerate()
            .fold(String::new(), |mut acc, (index, c)| {
                if index > 0 && c.is_uppercase() {
                    acc.push('_');
                }
                acc.push(c.to_lowercase().next().unwrap());
                acc
            })
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_pascal_to_snake_case() {
        assert_eq!("UserModel".to_snake_case(), "user_model");
        assert_eq!("PostModel".to_snake_case(), "post_model");
        assert_eq!("PascalCase".to_snake_case(), "pascal_case");
    }
}
