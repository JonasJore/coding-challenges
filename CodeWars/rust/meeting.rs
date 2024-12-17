// https://www.codewars.com/kata/59df2f8f08c6cec835000012/training/rust

fn string_to_tuple(str: &str) -> (&str, &str) {
    let parts: Vec<&str> = str.split(":").collect();
    let tuple = (parts[0], parts[1]);

    return tuple;
}

fn meeting(s: &str) -> String {
    let binding = s.to_uppercase();
    let uppercase_str: &str = &binding.as_str();
    let members_list: Vec<(&str, &str)> = uppercase_str
        .split(";")
        .map(|item| string_to_tuple(item))
        .collect();

    let mut sorted_members = members_list;

    sorted_members.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    let formatted_members: String = sorted_members
        .into_iter()
        .map(|member: (&str, &str)| format!("({}, {})", member.1, member.0))
        .collect::<Vec<String>>()
        .join("");

    return formatted_members;
}

fn main() {
    let meeting_participants = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill";
    println!("{}", meeting(meeting_participants));
}
