pub fn run() {
    let name = "Sai Kiran"; // this is NOT mutable
    let mut age = 26; // this is mutable variable
    println!("my age is {}", age);

    age = 27; //mutated

    println!("my name is {} and after birthday, my age is {}", name, age);

    //Also
    let (my_name, my_age) = ("Sai Kiran", 28);
    println!("my name is {} and my age is {}", my_name, my_age);
}