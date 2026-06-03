pub fn is_armstrong_number(num: u32) -> bool {
  let num_str = num.to_string();
 let power = num_str.len() as u32;
    let totals : u32 = num_str.chars().map(|f| f.to_digit(10).unwrap().pow(power)).sum();

    num == totals
}
