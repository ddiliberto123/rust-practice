fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 2; // Results in -1
    println!("hello {truncated}");

    // remainder
    let remainder = 43 % 5;

    let b: (i32, i32, i32) = (1, 2, 3);
    let a = [b, b, b, b, b];

    let first = a[0].1;
    let second = a[1];

    another_function(4);
    print_labeled_measurement(5, 'h');

    let y = // statement
    { // expression
        let x = 3; //statement, 3 is an expression
        x + 1 //expression
    };
    //evaluates to 4

    let y = // statement
    { // expression
        let x = 3; //statement, 3 is an expression
        x + 1; //statement
        //^^^^ semicolon turns into statement
    };
    //evaluates to ()

    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
