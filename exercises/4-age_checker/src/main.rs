struct Person {
    name: String,
    age: u8,
}

const AGE_OF_MAJORITY: u8 = 18;

fn mayor_de_age(name: String, age: u8, number: u8) {
    match age.cmp(&number) {
        std::cmp::Ordering::Less => println!("{} is a minor", name),
        std::cmp::Ordering::Greater => print!("{} is of legal age", name),
        std::cmp::Ordering::Equal => print!("{} is of legal age", name),
    }
}

fn main() {
    let person_one = Person {
        name: String::from("Justin"),
        age: 27,
    };

    print!("Name: {}, Age: {}\n", person_one.name, person_one.age);
    mayor_de_age(person_one.name, person_one.age, AGE_OF_MAJORITY);
}
