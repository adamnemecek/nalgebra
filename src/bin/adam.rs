extern crate nalgebra;
use nalgebra::Quaternion;



// use std::fmt::Debug;

// /*
// * i
// */


// // https://apps.dtic.mil/dtic/tr/fulltext/u2/a615302.pdf


//     // /// fsda
//     // #[inline]
//     // pub fn wedge2(&self, other: &Self) -> Self {
//     //     Quaternion::from_imag(self.imag().cross(&other.imag()))
//     // }

fn main() {
    let a = Quaternion::new(0.0, 1.0, 2.0, 3.0);
    let b = Quaternion::new(0.0, 5.0, 2.0, 1.0);
    let div = a.left_div(&b).unwrap();
    // dbg!(div);
    dbg!(div * b);

//     // let para = a.perp(&b);
//     let dot = a.dot(&b);
//     let inner = a.inner(&b);
//     let wedge = a.wedge(&b);
//     // dbg!(wedge);

//     // let wedge2 = a.wedge2(&b);
//     // dbg!(wedge2);
    

//     let proj = a.proj(&b);
//     let rej = a.rej(&b);

//     // let para = a.para(&b);
//     // let perp = a.perp(&b);
    
//     // dbg!(dot);
//     // dbg!(inner);
//     // dbg!(wedge);
//     dbg!(proj);
//     dbg!(rej);
//     // dbg!(para);
//     // dbg!(perp);
//     // dbg!(rej);

}