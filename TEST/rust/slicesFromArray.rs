fn main() {
    /*
    let arr = [1, 2, 3, 4];
    let slice = &arr[0..4]; // slice = &[2, 3]
    println!("array = {:?}", arr);
    println!("Array's slice = {:?}", slice);
     */
    let mut watr: &str = "Water";
    // mut is not necary for &str cauz u can only change what it is pointing to (ie: 2 lines below),
    // mut means to be able to be changing contents inside var
    //let watr = watr.push('x'); <- Cant do this
    let watr = "wateg";
    let guess = 1;
    //guess = 2;  <- CANT REASSIGN
    let guess = 2; <- // CAN REDECLARE
    std::io::stdin()
    .readline()
    .expect();


}
/*

\---------------------------------------------------------\
\/* Mutation of binding (needs mut) */                    \
\let mut s: &str = "Water";                               \
\s = "Juice"; /* valid, mut required */                   \
\                                                         \
\/* Shadowing (no mut needed)  */                         \
\let s: &str = "Water";                                   \
\let s = "Juice"; /*  new variable, mut not required */   \
\                                                         \
\/* Mutating contents (only works with String)  */        \
\let mut owned = String::from("Water");                   \
\owned.push_str(" Juice");/*modifies the actual string*/  \
\---------------------------------------------------------\

*/
