#![allow(non_snake_case)]

use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let first_number: i32= first.parse::<i32>().unwrap();

    let second = args.nth(0).unwrap();
    let second_number: i32 = second.parse::<i32>().unwrap();


    let result: i32 = euclideanAlgorithm(first_number, second_number);

   

    println!("{:?}", result);

} 

fn euclideanAlgorithm(number_one: i32, number_two: i32) -> i32{
    if number_one == 0{
        return number_two;
    }else if number_two == 0{
        return number_one;
    }

    if number_one > number_two{
        return euclideanAlgorithm(number_one % number_two, number_two);
    }else{
        return euclideanAlgorithm(number_two % number_one, number_one);
    }

}

fn primalityTest(number: i32) -> bool{
    let mut i: i32 = 2;
    while i < number{
        if number % i == 0{
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn factorial(number: i32) -> i32{
    if number == 1{
        1
    }else{
        number * factorial(number - 1)
    }
    
}

fn nthFibonacciNumber(index: i32) -> i32{
   
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut temp:i32 = 0;
    let mut i: i32 = 1;

    while i < index{
        temp = b;
        b = a + b;
        a = temp;
        i = i + 1
    }
    return b;
}

fn powerOfTwo(number_to_consider: i32) -> bool{
    let mut i: i32 = 0;
    let mut numberOfOnes: i32 = 0;
    let mut movingValue = number_to_consider;
    let mask: i32 = 0x1;
    while i < 32{
        if movingValue & mask == mask{
            numberOfOnes = numberOfOnes + 1;
        }
        movingValue = movingValue >> mask;
        i = i + 1;
    }
    if numberOfOnes > 1i32{
        return false;
    }
    return true;
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
    if operator == '+' {
        return first_number + second_number;
    } else if operator == '-'{
        return first_number - second_number;
    } else if operator == '/' {
        return first_number / second_number;
    } else if operator == '*' {
        return first_number * second_number;
    } else{
        return 0.0;
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String{
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}