pub trait TokenTypes {
    fn is_operator(&self) -> bool;
    fn is_number(&self) -> bool;
    fn is_word(&self) -> bool;
    fn is_word_or_number(&self) -> bool;
}
impl TokenTypes for str {
    fn is_operator(&self) -> bool {
        let operators_str: &str = ",-+*/^%";
        return operators_str.contains(&self);
    }
    fn is_number(&self) -> bool {
        return self.chars().all(|c| c.is_numeric() || c == '.');
    }
    fn is_word(&self) -> bool {
        return self.chars().all(|c| c.is_alphanumeric() || c == '.');
    }
    fn is_word_or_number(&self) -> bool {
        return self.chars().all(|c| c.is_alphanumeric() || c == '.');
    }
}
