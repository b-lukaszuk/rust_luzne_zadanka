use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{}'", s);
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{}", count_str);
    };
    (count, item)
}

fn main() {
    // if get_count_item("chairs") == (3, "chairs") {
    // if get_count_item("k chairs") == (3, "chairs") {
    if get_count_item("3 chairs") == (3, "chairs") {
        println!("All is OK. get_count_item(\"3 chairs\") == (3, \"chairs\")");
        println!("That's all. Goodbye.");
    } else {
        println!("Shouldn't have reached this part. Terminating the program.");
    }
}
