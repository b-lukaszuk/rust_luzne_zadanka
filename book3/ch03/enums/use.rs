#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // `use` so the names inside Stage are available without manual scoping
    use crate::Stage::{Advanced, Beginner};
    // `use` so the names inside Role are available without manual scoping
    use crate::Role::*;

    let stage = Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are acquiring knowledge!"),
    }
}
