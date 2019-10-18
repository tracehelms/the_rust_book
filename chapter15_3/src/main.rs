struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer { data: String::from("a")};
    let b = CustomSmartPointer { data: String::from("b")};
    let c = CustomSmartPointer { data: String::from("c")};
    let d = CustomSmartPointer { data: String::from("d")};
    println!("CustomSmartPointers created.");

    drop(c);
    println!("'c' dropped before the end of main.");
}
