fn main() {
    println!("Hello, world!");

    another_function();
    second_function(6);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("X equals {x}.");
    let x = plus_one(x);
    println!("X after plus_one function: {x}");
}

// can be declared before or after main, doesnt matter.
// just has to be in a scope that can be seen by caller
fn another_function() {
    println!("Hi from another function.");
}

// HAVE to declare the type of each parameter in the function. rust quirk
fn second_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements are instructions that perform an action and dont return a value
// expressions result in a value; dont put ending semicolons
fn expression(x: i32, y: i32) {
    // letting y = the result of the expression in { }
    let y = {
        let x = 3;
        x+1
    };
    println!("The value of y is: {y}");
}

// declaring return type of a function. again, no semicolon because we want to return the value
fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    x + 1
}