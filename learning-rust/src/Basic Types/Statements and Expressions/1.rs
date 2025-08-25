// // Make it work with two ways
// fn main() {
//    let v = {
//        let mut x = 1;
//        x += 2
//    };

//    assert_eq!(v, 3);

//    println!("Success!");
// }

// 1
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };

//     assert_eq!(v, ());
//  }

// 2
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}
