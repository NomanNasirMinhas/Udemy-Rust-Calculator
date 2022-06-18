//It is highly recommended to watch this video in parts if you can not grasp all concept in one video.
use std::io;
enum Operations{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mean,
    Remainder
}

struct CalculationData{
    f_num: i32,
    s_num: i32,
    operation_type: Operations
}

fn main() {
    println!("Welcome to Project 1: The Calculator");

    println!("Please choose one of the following operation to perform");
    println!("1-Addition");
    println!("2-Subtraction");
    println!("3-Multiplication");
    println!("4-Division");
    println!("5-Average/Mean");
    println!("6-Remainder");

    //variable declaration and initialization
    //mutability
    //Result
    //Error Handling
    //placeholder
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("You have entered {}", input);
    let choice:i8 = input.trim().parse().unwrap();
    if choice <= 6{
        let mut first_number:i32 = 0;
    let mut second_number:i32 = 0;
    if choice == 5{
        println!("Please Enter All the numbers separated by commas and without spaces");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {}", input);
        let numbers = input.trim().split(",");
        let mut vec_num: Vec<i32> = vec![];
       
        for num in numbers{
            vec_num.push(num.parse().unwrap());
        }
        findMean(vec_num);
    }
    else{
        println!("Please Enter the first number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {}", input);
        first_number = input.trim().parse::<i32>().unwrap();
    
        println!("Please Enter the second number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {}", input);
       second_number = input.trim().parse::<i32>().unwrap();
    
        println!("You have entered {} and {}", first_number, second_number);
    }
    

    let mut result:i32 = 0;
    //flow control
    if choice==1{
        //Add
        result = calculation(CalculationData{
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Addition
        });
    }
    else if choice==2{
        //Subtract
        result = calculation(CalculationData{
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Subtraction
        });
    }
    else if choice==3{
        //Multiple
        result = calculation(CalculationData{
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Multiplication
        });
    }
    else if choice==4{
        //Divide
        result = calculation(CalculationData{
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Division
        });

    }
    else if choice==6{
        //Remainder
        let tuple_demo: (i64, f32) = (first_number as i64, second_number as f32);
        result = findRemainder(tuple_demo);

    }

    if choice != 5{
        println!("Your Result is = {}", result);
    }
    }
    else{
        println!("Please Enter a number between 1 to 6");
    }
    
    
}

fn calculation(data: CalculationData) -> i32{
   match data.operation_type{
       Operations::Addition => {
           let result = data.f_num + data.s_num;
           result
       },
       Operations::Subtraction => {
        data.f_num - data.s_num
       },
       Operations::Multiplication => {
        data.f_num * data.s_num
       },
       Operations::Division => {
        data.f_num / data.s_num
       },
       Operations::Mean =>{0},
       Operations::Remainder =>{0},
   }
}

fn findMean(values: Vec<i32>){
    let mut sum = 0;

    for num in values.iter(){
        sum = sum + num;
    }
    let result = sum/values.len() as i32;
    println!("The Mean of the given values = {}",result);
}

fn findRemainder(values: (i64, f32)) -> i32{
    let result = values.0 % values.1 as i64;
    result as i32
}
