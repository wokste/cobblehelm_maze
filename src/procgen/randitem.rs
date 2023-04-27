


pub trait RandItem{
    type Item;

    fn rand_front_loaded(&self) -> &Self::Item;
}

impl<T> RandItem for Vec<T> {
    type Item = T;
    
    fn rand_front_loaded(&self) -> &Self::Item {
        let len = self.len();
        let id0 = fastrand::usize(0..len);
        let id1 = fastrand::usize(0..len + 1);
        &self[usize::min( id0, id1)]
    }
}