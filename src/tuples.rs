// Tuples group together values of different types
// Max 12 elements

pub fn tuples_res(){
    let name = "Galih";
    let city = "Sidoarjo";
    let old = 22;
    let person: (&str, &str, i8) = (name, city, old);
    //or
    let person1: (&str, &str, i8) = ("Aulia", "Surabaya", 19);

    println!("{} from {} and {} years old", person.0, person.1, person.2);
    println!("{} from {} and {} years old", person1.0, person1.1, person1.2);
}