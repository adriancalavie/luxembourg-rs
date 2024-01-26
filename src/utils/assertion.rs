pub enum Assertion<T, E> {
    True(T),
    False(E),
}

impl<T, E> Assertion<T, E> {
    #[allow(dead_code)]
    pub fn is_true(&self) -> bool {
        matches!(self, Assertion::True(_))
    }

    #[allow(dead_code)]
    pub fn is_false(&self) -> bool {
        !self.is_true()
    }
}
