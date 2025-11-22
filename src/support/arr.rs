use rand::seq::SliceRandom;

pub struct Arr;

#[allow(dead_code)]
impl Arr {
    /// Get a random element from a slice.
    pub fn random<T>(slice: &[T]) -> Option<&T> {
        let mut rng = rand::thread_rng();
        slice.choose(&mut rng)
    }

    /// Shuffle a vector in place.
    pub fn shuffle<T>(slice: &mut [T]) {
        let mut rng = rand::thread_rng();
        slice.shuffle(&mut rng);
    }

    /// Wrap a value in a Vector if it's not already one (conceptually).
    /// In Rust, this just creates a Vec with one item.
    pub fn wrap<T>(value: T) -> Vec<T> {
        vec![value]
    }

    /// Join array elements with a string.
    pub fn join<T: ToString>(slice: &[T], glue: &str) -> String {
        slice.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(glue)
    }
}
