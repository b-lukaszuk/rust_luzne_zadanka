fn main() {
    println!("Toy program.");
    println!("It adds a few elts to slice.");
    println!("Moreover, it prints its capacity whenever it changes.\n");

    let mut v: Vec<i32> = vec![];
    let mut prev_cap: usize = 0;
    println!("{:?}, length: {}, capacity: {}", v, v.len(), v.capacity());

    for i in 1..20 {
        println!("Adding num to vector.");
        v.push(i);
        if prev_cap != v.capacity() {
            // capacity seems to be doubled when running out of space
            prev_cap = v.capacity();
            println!("{:?}, length: {}, capacity: {}", v, v.len(), prev_cap);
        }
    }

    println!("\nThat's all. Goodbye!");
}
