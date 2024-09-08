mod citizen; // Import citizen //

fn main() {
    // Define test names //
    let names = [
        "Liam", "Emma", "Noah", "Olivia", "Ethan", "Ava",
        "Lucas", "Sophia", "Mason", "Isabella"
    ];

    // Create a random citizen with 'corporate' government
    let test_citizen = citizen::create_citizen(&names, "corporate");

    citizen::display_citizen(&test_citizen)
}