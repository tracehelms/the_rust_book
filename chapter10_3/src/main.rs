fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // let string5 = String::from("long string is long");
    // let result;
    // {
    //     let string6 = String::from("xyz");
    //     result = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
