#![allow(overflowing_literals)]


struct Person{
    name: String,
    age: u32,
    address: String,
}

impl Person{
    fn print_info(&self){
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Address: {}", self.address);
    }
}

impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person {
            name: name.into(),
            age: 0,
            address: String::new(),
        }
    }
}


pub fn entry_of_types() {
    let integer: i32 = 42;
    
    println!("Integer value: {integer}", );

    let float: f64 = 3.14;
    println!("Float value: {float}", );

    let boolean: bool = true;
    println!("Boolean value: {boolean}", );

    let array: [i64; 5] = [1, 2, 3, 4, 5];
    println!("Array value: {:?}", array);
    println!("Array value mem size: {}", std::mem::size_of_val(&array));
    

    let tuple = (1, "hello", 3.14);
    println!("Tuple value: {:?}", tuple);
    println!("Tuple value for integer: {}", tuple.0);
    println!("Tuple value for string: {}", tuple.1);
    println!("Tuple value for float: {}", tuple.2);

    let person1 = Person::from("ldoublej1");
    person1.print_info();

    let person2: Person = "ldoublej2".into();
    person2.print_info();

    println!(" 128 as a i8 is : {}", 128 as i8);





}