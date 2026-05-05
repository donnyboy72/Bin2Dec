use std::io;

fn main() {
    let mut input = String::new();

    loop {  
        input.clear(); 
        println!("Enter binary digits of length 8:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim(); 

        if trimmed.len() != 8 {
            println!("Not 8 digits long");
            continue;
        }

        if !trimmed.chars().all(|c| c == '0' || c == '1') {
            println!("Only 1 and 0 are allowed");
            continue;
        }
        break;
    }

    let dec = into_dec(&input.trim());

    println!("Binary number: {} \nDecimal number: {}",input,dec);
}

fn into_dec(number: &str) -> u32 {
    let mut result = 0;
    let mut exponent = 7;

    for num in number.chars() {
        if num == '1' {
            result += 2_u32.pow(exponent);
        }
        if exponent > 0 {
            exponent -= 1;
        }
    }
    result
}
