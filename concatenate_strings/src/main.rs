fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    result.push_str(a);
    result.push_str(b);
    result
}

fn main() {
    let string1 = String::from("string1");
    let string2 = String::from("string2");
    let concatenated_string = concatenate_strings(&string1[..], &string2[..]);
    println!("{concatenated_string}");
}
