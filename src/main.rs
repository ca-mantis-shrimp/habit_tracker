fn add_habit(habit_list: &mut Vec<String>, habit: &str) {
    habit_list.push(habit.to_string());
}

fn main() {
    let mut habit_list: Vec<String> = vec![];
    let new_habit = std::env::args().nth(1).expect("no habit given");

    add_habit(&mut habit_list, &new_habit);
    println!("{}", &habit_list[0]);
}

#[test]
fn add_basic_habit() {
    let mut test_habit_list = vec![];
    add_habit(&mut test_habit_list, "test habit");
    assert!(test_habit_list[0] == "test habit")
}

#[test]
fn add_two_habits() {
    let mut habit_list = vec![];
    add_habit(&mut habit_list, "first habit");
    add_habit(&mut habit_list, "second habit");

    assert!(habit_list[0] == "first habit" && habit_list[1] == "second habit");
}
