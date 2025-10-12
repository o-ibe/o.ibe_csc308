use std::io;

fn main() {
<<<<<<< HEAD
    let mut temp1 = String::new();
    let mut temp2 = String::new();

    println!("\nPlease enter the first temperature in Celsius");
        io::stdin().read_line(&mut temp1).expect("Not a valid string");
        let a:f32 = temp1.trim().parse().expect("Not a valid number");
    
    println!("\nPlease enter the second temperature in Fahrenheit");
        io::stdin().read_line(&mut temp2).expect("Not a valid string");
        let b:f32 = temp2.trim().parse().expect("Not a valid number");

    let new_temp1:f32 = (a * (9.0/5.0)) + 32.0;
    let new_temp2:f32 = (b - 32.0) * (5.0/9.0);

    println!("\nTemperature: {}C", a);
    println!("\nConverted: {}F", new_temp1);
    println!("\nTemperature: {}F", b);
    println!("\nConverted: {}C", new_temp2);
    
=======
    let mut input = String::new();

    println!("Enter the original bill");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let old_bill:f32 = input.trim().parse().expect("Not a valid number");
    let new_bill:f32;
    let discount:i32;

    if old_bill >= 5000.0 && old_bill < 10000.0{
        new_bill = 0.9 * old_bill;
        discount = 10;
    } else if old_bill >= 10000.0{
        new_bill = 0.85 * old_bill;
        discount = 15;
    } else {
        new_bill = old_bill;
        discount = 0;
    }

    println!("Original Bill: N{}", old_bill);
    println!("Discount Applied: {}%", discount);
    println!("Final Bill: N{}", new_bill);
>>>>>>> 1737174 (completed first week's second task)
}
