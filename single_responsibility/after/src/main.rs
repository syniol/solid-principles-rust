use solid_principles_single_responsibility::user::registration;
use solid_principles_single_responsibility::user::user::User;

fn main() {
    let user_registration_service = registration::new();
    let result = user_registration_service
        .register_user(&User{
        name: "John Doe",
        date_of_birth: "",
    });
    if result.is_err() {
        eprintln!("{:?}", result.err());
        return
    }

    println!("{}", result.unwrap())
}
