use tuple::tuple::{Tuple, TupleField};

mod tuple;
mod tuple_space;

fn main() {
    let mut t1 = Tuple::new("t1", 2);
    t1.insert(0, TupleField::Int(Some(3)));
    t1.insert(1, TupleField::Float(Some(std::f32::consts::PI)));

    let _ = match Tuple::from_str("('t2', float 6.276, int ?)") {
        Ok(ok) => {
            println!("Created tuple from str: {:?}", ok);
            ok
        }
        Err(err) => {
            println!("Couldn't create tuple because of error: {:?}", err);
            Tuple::new("t2", 0)
        }
    };

    println!("Tuple 1: {:?}", t1);
}
