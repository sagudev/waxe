#[derive(Clone, Copy, Debug)]
pub enum Trio<T, E> {
    /// Some value `T`
    Ok(T),
    /// No value but ok
    Empty,
    /// error
    Err(E),
}

impl<T, E> Trio<T, E> {
    pub fn ok(self) -> Option<T> {
        match self {
            Trio::Ok(t) => Some(t),
            _ => None,
        }
    }
}