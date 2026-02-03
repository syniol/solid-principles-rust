use solid_principles_open_close::payment;
use solid_principles_open_close::user::member::Member;

fn main() {
    let resp = payment::new().pay(&Member {
        id: 2,
        name: "John Doe",
        preferred_payment_method: "OB",
    });
    if resp.is_err() {
        eprintln!("{:?}", resp.err().unwrap());
    }
}
