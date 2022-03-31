// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));

    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(int_opt) = optional_integers_vec.pop() {
        if let Some(int) = int_opt {
            println!("current value: {}", int);
        }
    }
}