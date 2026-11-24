pub trait Factory<'a>: Clone + 'a {
    fn r#use(&self) -> impl (Fn() -> Self) + Clone + 'a {
        let this = self.clone();
        move || this.clone()
    }
}

impl<'a, T: Clone + 'a> Factory<'a> for T {}