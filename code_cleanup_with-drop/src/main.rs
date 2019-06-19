struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self)  {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomerSmartPointers created");
    // c.drop(); // not allowed
    drop(c); // std::mem::drop
    println!("CustomSmartPointer dropped before the end of main");
}
