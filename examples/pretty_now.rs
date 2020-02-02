use datetimeutils::PostEpochTime;

fn main() {
    println!("It is now {} in the UTC timezone", PostEpochTime::now().unwrap());
}
