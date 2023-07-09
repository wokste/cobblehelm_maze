pub trait RandItem {
    type Item;

    fn rand_front_loaded(&self, rng: &mut fastrand::Rng) -> &Self::Item;
}

impl<T> RandItem for [T] {
    type Item = T;

    fn rand_front_loaded(&self, rng: &mut fastrand::Rng) -> &Self::Item {
        let len = self.len();
        let id0 = rng.usize(0..len);
        let id1 = rng.usize(0..len + 1);
        &self[usize::min(id0, id1)]
    }
}

impl<T> RandItem for Vec<T> {
    type Item = T;

    fn rand_front_loaded(&self, rng: &mut fastrand::Rng) -> &Self::Item {
        self.as_slice().rand_front_loaded(rng)
    }
}
