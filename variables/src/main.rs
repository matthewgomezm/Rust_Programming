// constant var. truly always immutable
// constants can only be set; cant make it a value that will be evaluated at runtime
// must be annotated

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    
    let mut x = 5;  // mut shows that the program will 99% change this variable later
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    
    // example of shadowing. x takes most recent value in scope
    // 'let' allows you to perform transformations while still being immutable after theyve completed. like resetting.
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("x in inner scope is {x}");
    }

    println!("val of x is {x}");
    


    // different types and operations below
    // also cool: _ before the var name lets compiler know its intentionally unused
    let _sum = 5+10;
    let _difference = 95.0-23.1;
    let _product = 6*8;
    let _quotient = 56.7/32.2;
    let _truncated = -5/3;    // results in -1
    let _remainder = 43%5;
    let _t = true;
    let _f:bool = false;
    let c = 'z';
    let z: char = 'Z';

    
    // compound types group multiple vals into one type; 2 compound types: tuples and arrays
    let tup: (i32,f64,u8) = (500, 6.4, 1);

    let(x, y, z) = tup;

    println!("The value of z is: {z}");

    //can also access using (.) and the index of the tuple

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // lets you overwrite the values in the tuple
    let mut f: (i32, i32) = (1, 2);
    f.0 = 0;
    f.1 += 5;

    // arrays
    let a = [1,2,3,4,5,6];
    // useful to allocate on the stack; when u want to ensure you always have fixed number of elements
    let a: [i32; 5] = [1,2,3,4,5];
    // can declare array of same number for how long:
    let a = [3;5]; // five 3's in the array
    // element access:
    let first = a[0];
    let second = a[1];

}