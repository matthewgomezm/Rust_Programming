fn main() {
    let number = 3;

    if number < 5 {
        println!("cond true");
    }
    else {
        println!("cond false");
    }

    // if number { ... } is wrong. cant change type to boolean unless explicitly defined

    if number != 0 {
        println!("Num is not zero");
    }

    let number = 6;

    // if else if, else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // can use if to assign variables since its an expression
    let condition = true;
    let number = if condition {5} else {6};
    println!("the val of number is: {number}");


    // looping example. breaks when the counter reaches 10 and stores counter * 2 into result
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 'counting up is called a loop label. the statement breaks that specific loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");



    // while loop terminating once number is 0
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let z = [1,2,3,4,5];
    let mut index = 0;
    while index < 5 {
        println!("The number in z at index {index} equals:{}", z[index]);
        index += 1;
    }

    // cleaner way to do example above. kinda like python with the placeholder identifier
    for element in z {
        println!("The numbers in z are (one by one): {element}");
    }

    println!("LIFTOFF!!!");
}

