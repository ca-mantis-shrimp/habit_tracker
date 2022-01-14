fn add_habit(habit_list: Vec<String>, habit: &str) -> Vec<String> {
    let mut updated_habit_list = habit_list.clone();
    updated_habit_list.push(habit.to_string());
    return updated_habit_list;
}

fn main() {
    let new_habit = std::env::args().nth(1).expect("no habit given");
    let updated_list = add_habit(vec![], &new_habit);
    println!("{}", updated_list[0]);
}

#[test]
fn add_basic_habit() {
    let test_habit_list = add_habit(vec![], "test habit");
    assert!(test_habit_list[0] == "test habit")
}

#[test]
fn add_two_habits() {
    let habit_list = add_habit(vec![], "first habit");
    let habit_list = add_habit(habit_list, "second habit");

    assert!(habit_list[0] == "first habit" && habit_list[1] == "second habit");
}
