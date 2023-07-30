use std::fs;

// Probably not the best way to do this. there is probably a really good functional way to do this.
fn group_elves(calories: Vec<&str>) -> Vec<Vec<i32>> {
    let mut full_list: Vec<Vec<i32>> = vec![];
    let mut buffer: Vec<i32> = vec![];
    for calorie in calories {
        if calorie.is_empty() {
            full_list.push(buffer);
            buffer = vec![];
        } else {
            let cal_parsed = match calorie.parse::<i32>() {
                Ok(number) => number,
                Err(error) => panic!("Not an int!: {:?}", error),
            };
            buffer.push(cal_parsed);
        }
    }
    return full_list;
}

fn sum_elves(mut e_calorie_counts: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = vec![];
    for x in e_calorie_counts.iter_mut() {
        let mut sum: i32 = 0;
        for y in x.iter_mut() {
            sum += *y;
        }
        sums.push(sum);
    }
    return sums;
}

fn main() {
    let input_result = fs::read_to_string("input");
    let input = match input_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let calories = input.split("\n").collect();
    let elves_count = group_elves(calories);
    let mut sums = sum_elves(elves_count);
    let maxvalue = sums.iter().max();
    match maxvalue {
        Some(max) => println!("Most calories held by one elf: {:#?}", max),
        None => println!("List is empty"),
    }
    sums.sort_by(|a, b| b.cmp(a));
    let sorted_sums: &[i32] = &sums;
    let top3 = &sorted_sums[0..3];
    let sum: i32 = top3.iter().sum();
    println!("Sum of top 3 elves calorie counts: {:#?}", sum)
}
