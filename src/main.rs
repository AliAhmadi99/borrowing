fn main() {
    let mut my_laptop: String = String::from("my laptop");

    // let ali_laptop = &my_laptop;
    // let ehsan_laptop = &my_laptop;
    // let bahram_laptop = &mut my_laptop;

    lend_to_ali(&my_laptop);
    lend_to_ehsan(&my_laptop);
    lend_to_bahram(&mut my_laptop);
}

fn lend_to_ali(input: &String) {
    println!("Ali has borrowed {input}");
}

fn lend_to_ehsan(input: &String) {
    println!("Ehsan has borrowed {input}");
}

fn lend_to_bahram(input: &mut String) {
    input.push_str(" and upgraded it to Windows 11!");

    println!("Bahram has borrowed {input}");
}
