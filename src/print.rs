pub fn print_res(){
    //Print Colsole
    println!("Hello from the print.rs file");

    //Formatting
    println!("Number = {}", 1);
    println!("{} from the {}", 1, 10);
    println!("{} from the {}", "Galih", "Sidoarjo");

    //Positional Arguments
    println!("{0} from the {1} and {0} like {2}", "Galih", "Sidoarjo", "Code");

    //Variable Arguments
    println!("{name} like {activity}", name = "Galih", activity = "coding");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 19, 19, 19);

    //placeholder debug trait
    println!("{:?}", (12, true, "Hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
    println!("10 x 10 = {}", 10 * 10);
    println!("10 : 10 = {}", 10 / 10);
}