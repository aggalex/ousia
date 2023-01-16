pub trait Builder: Default {
    type Target;

    fn build(&mut self, func: impl Fn(Self::Target));
    fn create(&mut self) -> Self::Target;
}

pub trait ForteExt {
    type Builder: Builder<Target = Self> + Sized;

    fn forte() -> Self::Builder {
        Self::Builder::default()
    }
}