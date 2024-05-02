mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("Adding to waitlist.");
        }
        fn seat_at_table() {
            println!("Sitting at a table.");
        }
    }

    mod serving {
        fn take_order() {
            println!("Taking order.");
        }
        fn serve_order() {
            println!("Serving order.");
        }

        fn take_payment() {
            println!("Taking payment");
        }
    }
}
