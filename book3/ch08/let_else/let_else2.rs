use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{}'", s),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse itneger: '{}'", count_str);
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
