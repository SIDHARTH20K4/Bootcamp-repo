fn main() {
    fn main() {
        let x: i32 = 16;
        //to print the output, we need to write a macro => println! macro
        println!("{}",x);
    
        let z: String = String::from("Hello,Soroban");
        let y: &str = "Hello,Stellar";
    
        println!("{z}");
    
        // pub fn event (name: String) {
        //     let name: String = String::from("James");;
        //     println!("{}",name);
        // }
    
        let e: EventForKids = EventForKids{
            name: String::from("jack"),
            data: String::from("abc"),
            number_of_participants : 45,
            place: String::from("chennai")
        };
    }
    
    struct EventForKids {
        name: String,
        data: String,
        number_of_participants : u32,
        place: String 
    }
}
