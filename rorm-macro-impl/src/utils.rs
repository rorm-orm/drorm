pub fn to_db_name(name: String) -> String {
    let mut name = name;
    name.make_ascii_lowercase();
    name
}
