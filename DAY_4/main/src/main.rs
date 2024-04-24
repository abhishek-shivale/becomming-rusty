fn main() {
    //  Scalar Types
    let a = 1212_12; //decimal
    let b = 0xff; //hex
    let c = 0o77; //octal
    let d = 0b111_000; //binary
    let e = b'A'; //byte (8bit only u8)
    let f: u8 = 255; //max
                     // flote
    let f = 3.0; //floate
    let g = true; //bool
    let h = 'h'; //char

    //Compound types
    //tuples
    //can have mutilpe data type
    let tup: (&str, i64) = ("abhishek", 930784567820);
    let (x, y) = tup; //  destucture
                      // //dot notation
    let firstvalue = tup.0; // First value i s start with 0

    // //array
    // //only have one data type
    // //fix not change
    let arrays = [1, 2, 3, 4, 5];
    let one = arrays[0];
    let two = arrays[1];

    //function
    let sum = my_main(12, 12);
    print!("sum is {}", sum);

    //control flow
    const CONDITION: bool = true;
    if (CONDITION) {
        println!("condtion matched");
    } else {
        println!("condition unmatched")
    }

    // //also
    let value = if (CONDITION) {
        println!("condtion matched")
    } else {
        println!("condition unmatched")
    };

    //loop
    //1st
    loop {
        println!("loop")
        //untile we call break
    }
    //while run untile certainn codtion matched
    let i = 0;
    while (i == 1) {}

    //for loop
    //useful for loopin through an collection of a data
    let data = [1, 2, 3, 45];
    for element in data.iter() {}

    // we can also loop throgh range like this
    for num in 1..3 {}
}

// all small letter and _ for space b/w
// we ca also give paramter by spacify parameter and data type
// function is smae as statemet not return value ad expression return value
//
fn my_main(x: u32, y: u32) -> u32 {
    return x + y;
}
// fn my_main(x:u32,y:u32) -> u32 {
//       x+y
//  }
