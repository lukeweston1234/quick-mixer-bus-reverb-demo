pub trait Playable<T> where T: Send  {
    fn tick(&mut self) -> Option<(T, T)>;
}
