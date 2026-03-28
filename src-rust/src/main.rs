fn main() {
    //guessing_game();
    //primitive_data_types();
    compound_data_types();
}

/*
   #################################|--------------------|##############################################
   #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$|   Guessing Game    |$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$#
   #################################|--------------------|##############################################
*/
fn guessing_game() {
    let mut lessThan = false;
    let mut greaterThan = false;
    let mut equalTo = false;
    println!("Please enter your guess");
    let mut guess = String::new();

    let mut random_num1 = rand::thread_rng();
    let random_numInt = rand::Rng::gen_range(&mut random_num1, 1..=100);

    println!("Random Number = {}", random_numInt);
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Could not read input");

    let guess: i32 = guess.trim().parse().expect("Please enter a valid number");
    //  the line above is lwk rlly imp cauz if u comparet 2 values, and one of them is having hwite space
    // then it will give incorect result

    println!("You guessed, {}", guess);
    //  let mut random_num = random_numInt.to_string();
    let mut random_num = random_numInt + 1 - 1;

    match guess.cmp(&random_num) {
        std::cmp::Ordering::Less => {
            println!("Too small");
            lessThan = true;
            greaterThan = false;
            equalTo = false;
        }

        std::cmp::Ordering::Greater => {
            println!("wayy more bruh");
        }

        std::cmp::Ordering::Equal => {
            println!("equall");
        }
    };
}

// to_string()

/*
   !!!!!!!!!!!!!!!!!!!!!!!!!!!!!|--------------------|##################################################
   $$$$$$$$$$$$$$$$$$$$$$$$$$$$$|END END END END END |guessingGame$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
   #############################|--------------------|!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
*/
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
/*
   #################################|------------------------|##############################################
   #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$|  Primitive Data Types  |$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$#
   #################################|------------------------|##############################################
*/
// + Signed VS Unsigned (more about that in obsidian vault)
fn primitive_data_types() {
    println!("Hellow rust from cargo ");
    let x: i32;
    x = 32;
    let xneg: i32 = -42;
    let y: u64 = 100;
    //    let y: u64 = -100; <- INCORRECT

    // The i32 can take 32bit of data, wehre as the u64 can take 64bit data
    //range for i32 is 2^31 (from negative to positve)
    // 2^63 (neg to postivitve) * i64
    // i32 - 2147483647
    // i64 - 9223372036854775807
    let mut wat32: i32 = 2147483647;
    let mut negwat32: i32 = -2147483647;
    let mut wat64: i64 = 9223372036854775807;
    // let mut wat64plus1: i64 = 9223372036854775807 + 1; <- INCORRECT OVERFLOW
    println!(
        "Signed integer positive ={}, Signed integer negative ={}, Unsigned Integer ={}",
        x, xneg, y
    );

    // int, float, bolean, char
    //Integer
    //Rust has sigened (+ and -), and unsigned integers (only +) tpyes of diffrent sizes:
    // i8, i16, i32, i64, i128: Signed Integers
    // u8 "      ""          ": Unsigned Integers
    // unassigned (u) is always a bit larger then assinged (I)
    // if I is to the power of 63, u is to the power of 64
    /*
    Signed (i32): Temperatures (-10°C to 40°C), financial balances (-500 PKR to +500 PKR).
    // so lwk ts isthe size of yo balls
    Unsigned (u32): Array indices (0 to n), file sizes (0 to 4 GB), network packet lengths.
    */
    // Signed' smost range is used for neg and half for positive, unsiged full range on postivive
    // floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi = {}", pi);
    //Boolean Values ; True or False (no capital)
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);
    // character type - char
    let letter: char = 'a';
}
/*
   !!!!!!!!!!!!!!!!!!!!!!!!!!!!!|--------------------|##################################################
   $$$$$$$$$$$$$$$$$$$$$$$$$$$$$|END END END END END |$$primitiveDataTypes$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
   #############################|--------------------|!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
*/
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

/*
   #################################|-----------------------|##############################################
   #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$|  Compound Data Types  |$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$#
   #################################|-----------------------|##############################################
*/
// arrays, tuples, slices, and string (slice string)
fn compound_data_types() {
    // Compound Data Types
    //ARRAYS
    //They contain elements of the same type
    // so an array of intergers can onl hav ints
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    //println!("Number Arry: {}", numbers);
    // the reason this is an error (above line)
    // RUST WE HAVE T2 TYPES OF FORMATERERS (2 TPYES)
    // DEBUGGABLE {:?}
    // TODO: Learn Debuggable and display formats
    // Display {}
    println!("Number Array: {:?}", numbers);
    //&str = string slice
    //let mix = [1,2, "apple", true]
    //println!("MIX = {:?}", mix)

    // TODO: Learn &str and string
    //&str = not a string per say, but a refernece to a string
    let fruits: [&str; 3] = ["apples", "banana", "grape"];
    println!("Fruits = {:?}", fruits);
    println!("Fruits 1= {}", fruits[0]); // dont need debugger format to display a elemtn 
    println!("Fruits 2= {}", fruits[1]);
    println!("Fruits 3= {}", fruits[2]);

    // Tuples
    let human1 = (" water", true);
    //let human(String, i32, bool) = ("Alice", 32, false); False
    //let human(&str, i32, bool) = ("Alice", 32, false); Correct
    let human: (String, i32, bool) = ("Alice".to_string(), 32, false);
    println!("Human = {:?}", human);

    //ignore ocmments  196, 197,198 (3 lines below htis comment)
    //u can call x.to_string(), inside tuple then print it in debug format cauz it is not debuggable, u have to
    //speiify it is string, also anotehr thing, u acant do _ if u do x.to_string()
    // also u cant have a tuple with inf elemtns, u acn to have a certian amount

    //let mix_tuples = ("Sigma", "awater".to_string(),true,,[1, 3, 5],["Water", "Fire"],0.3, [true, false, true],[0.12, 0.12, 0.214],['a', 'b'],"b","329402304982340328420934820923842002394820",340000000000000000000000000000000000,342942342342323,-1294,"-1293",);
    //println!("Human = {:?}", mix_tuples);
    let mix_tuples = (
        "Sgima".to_string(),
        "Water",
        [1, 2, 3, 4],
        ["abc", "def", "gdh"],
        ["a", "b"],
        ['a', 'b'],
        'b',
        -1,
        2,
        0.012,
        [0.1, 0.01, 0.001, 01.213, 0.4, 02.12, 0.2],
        ("water", 021, [32423, 2913, -3121], "b"),
    );
    // this is max limit of number of elemetn, b4 u cant print it using debuggable format,
    // u can have more elemnts but u owuld have to print it by dividing
    // u techinally print it but u pwud have to usea custom struct or nest tuples or use array or vectors for collection
    println!("{:?}", mix_tuples); //only print12
    println!("The elemtn{}", (mix_tuples.11).2[2]);
    //    (mix_tuples.9).8.6.7[8]

    //11th indx of "mix_typles", whcih is a tubple it self, 2nd index of that tubple (which is an array), the 2nd index of that which is -3121

    // Slices:
}
/*
   !!!!!!!!!!!!!!!!!!!!!!!!!!!!!|--------------------|##################################################
   $$$$$$$$$$$$$$$$$$$$$$$$$$$$$|END END END END END |$$compoundDataTypse$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
   #############################|--------------------|!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
*/
