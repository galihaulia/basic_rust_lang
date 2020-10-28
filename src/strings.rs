// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn strings_res(){
    let mut hello = String::from("Galih Aulia ");

    //Get Length
    println!("Length: {}", hello.len());

    //Push Char
    hello.push('A');

    //Push string
    hello.push_str(" Al Hakim!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Contains
    println!("Contains 'Al' {}", hello.contains("Al"));//if in hello noop "Al" => false

    println!("{}", hello);

    //Replace
    println!("Replace: {}", hello.replace("A Al Hakim", "Al Hakim"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('G');
    s.push_str("aL");

    //Assertion testing
    assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}