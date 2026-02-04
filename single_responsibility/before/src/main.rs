use solid_principles_single_responsibility::user::registration;
use solid_principles_single_responsibility::user::user::User;
use std::process;

fn main() {
    let user_registration_service = registration::new();
    let result = user_registration_service.register_user(&User {
        name: "John Doe",
        date_of_birth: "1995-08-01",
    });
    if result.is_err() {
        eprintln!("{}", result.err().unwrap());
        process::exit(1);
    }

    println!("{}", result.unwrap())
}
