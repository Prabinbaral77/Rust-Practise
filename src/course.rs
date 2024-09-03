use std::io;
fn main() {
    //1
    let mut arr1: [&str; 3] = ["Hello", "Word", "Coders"];
    write_arr(&mut arr1);
    println!("Array is {:?}", arr1);
    //2
    let mut v: Vec<i32> = Vec::new();
    let mut v = Vec::<i32>::new();
    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    //3
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(10);
    println!("The value of array is {:?}", v);
    //4
    let mut vc_str: Vec<&str> = vec!["hello", "word", "prabin"];
    write_str(&mut vc_str);
    println!("the value of vc_str {:?}", vc_str);
    //5. while loop
    let mut count = 1;
    while count < 5 {
        println!("Hello");
        count += 1;
    }
    //6. for loop
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for element in &arr {
        println!("the value of ele is {}", element);
    }
    //7.Match
    let number: u32 = 1;
    match number {
        1 | 9 => println!("The number is one or 9"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        4 => println!("The number is 4"),
        _ => println!("Unrecognized number"),
    }

    fn is_even(num: u8) -> bool {
        if num % 2 == 0 {
            return true;
        }
        return false;
    }

    let number = 5;
    match number {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _ => println!("unrecognized"),
    }
    //8 IO
    let mut input = String::new();
    println!("Enter your name");
    io::stdin().read_line(&mut input).expect("Running Failed");
    println!("Welcome to the gangsstaverse {}", input);
}

fn write_str(vss: &mut Vec<&str>) {
    vss.push("Bidhat");
    println!("the value of  vss {:?}", vss);
}
