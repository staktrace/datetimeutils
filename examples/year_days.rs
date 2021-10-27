use datetimeutils::{days_in_year, PostEpochTime};

fn main() {
    let now = PostEpochTime::now().unwrap();
    println!(
        "The current year is {}, which has {} days",
        now.year(),
        days_in_year(now.year())
    );
}
