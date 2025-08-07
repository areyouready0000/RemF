use std::any::TypeId;
use std::cell::OnceCell;

struct Vector<T: Copy> {
    coords: OnceCell<Vec<T>>
}

impl<T: Copy + 'static> Vector<T> {
    pub fn new() -> Self {
        let func = || { panic!("Vector as not been set") };

        Self {
            coords: OnceCell::new()
        }
    }

    pub fn set(&mut self, coords: &[T]) {
        if coords.len() == 0 {
            panic!("Vector has no coords");
        }

        match TypeId::of::<T>() {
            id if id == TypeId::of::<f32>() => (),
            id if id == TypeId::of::<f64>() => (),
            _ => panic!("Vector only supports floating point types (f32 or f64)"),
        }

        self.coords.get_or_init(|| {
            (*coords).to_owned()
        });
    }

    pub fn get(&self) -> &Vec<T> {
        self.coords.get().unwrap()
    }

    pub fn x(&self) -> T {
        self.coords.get().unwrap()[0]
    }
    pub fn y(&self) -> T {
        let coords = self.coords.get().unwrap();

        if (*coords).len() < 2 {
            panic!("Vector has less than two coords");
        }

        (*coords)[1]
    }
    pub fn z(&self) -> T {
        let coords = self.coords.get().unwrap();

        if (*coords).len() < 3 {
            panic!("Vector has less than three coords");
        }

        (*coords)[2]
    }
    pub fn w(&self) -> T {
        let coords = self.coords.get().unwrap();

        if (*coords).len() < 4 {
            panic!("Vector has less than four coords");
        }

        (*coords)[3]
    }
}