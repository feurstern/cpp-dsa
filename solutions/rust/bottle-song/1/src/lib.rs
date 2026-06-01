pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| {
            let current = start_bottles - i;
            let next = current - 1;

            format!(
                "{} green {} hanging on the wall,\n\
                 {} green {} hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be {} green {} hanging on the wall.",
                num_to_word(current, true),
                pluralize(current),
                num_to_word(current, true),
                pluralize(current),
                num_to_word(next, false),
                pluralize(next)
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n") 
}


fn num_to_word(n: u32, capitalize: bool) -> &'static str {
    match (n, capitalize) {
        (0, false) => "no",
        (0, true)  => "No",
        (1, false) => "one",
        (1, true)  => "One",
        (2, false) => "two",
        (2, true)  => "Two",
        (3, false) => "three",
        (3, true)  => "Three",
        (4, false) => "four",
        (4, true)  => "Four",
        (5, false) => "five",
        (5, true)  => "Five",
        (6, false) => "six",
        (6, true)  => "Six",
        (7, false) => "seven",
        (7, true)  => "Seven",
        (8, false) => "eight",
        (8, true)  => "Eight",
        (9, false) => "nine",
        (9, true)  => "Nine",
        (10, false) => "ten",
        (10, true)  => "Ten",
        _ => "",
    }
}

fn pluralize(n: u32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}