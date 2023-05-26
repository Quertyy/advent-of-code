mod day1;
mod day2;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
    let top_elves = day1::get_top_three();
    let mut total: i32 = 0;

    for (_, value) in &top_elves {
        total += value;
    }
    println!("Top 3 elves is {:?}", top_elves);
    println!("Total calories is {}", total);
}

fn day_2() {
    let total_score = day2::get_result();
    println!("Total score is {}", total_score);
}
