use tuple::tuple::{Tuple, TupleField};

use crate::util::{Serializable, DisplayBinary};

mod tuple;
mod tuple_space;
mod util;

fn main() {
    let mut t1 = Tuple::new("t1", 2);
    t1.insert(0, TupleField::Int(Some(3)));
    t1.insert(1, TupleField::Float(Some(std::f32::consts::PI)));

    let tuple_str = "('t2', float 6.276, int ?, float 2231, float ?, int 21)";
    let t2 = match Tuple::from_str(tuple_str) {
        Ok(ok) => {
            println!("Created tuple from str: {:?}", ok);
            ok
        }
        Err(err) => {
            println!(
                "Couldn't create tuple `{tuple_str}` because of error: {:?}",
                err
            );
            Tuple::new("t2", 0)
        }
    };

    let t1_bytes = t1.serialize();
    println!("Tuple 1: {:?}", t1);
    println!(
        "Tuple 1 in byte form: {:?}",
        t1_bytes
            .iter()
            .map(|e| format!("{e:08b}"))
            .collect::<Vec<_>>()
    );

    let t2_bytes = t2.serialize();
    println!("Tuple 2: {:?}", t2);
    println!("Tuple 2 in byte form: {:?}", t2_bytes.display_bin());

    println!(
        "Tuple 1 from bytes: {:?}",
        Tuple::deserialize(&t1_bytes).unwrap()
    );

    println!(
        "Tuple 2 from bytes: {:?}",
        Tuple::deserialize(&t2_bytes).unwrap()
    );
}
