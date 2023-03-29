pub fn update_json_value<T>(new_value: Option<T>, old_value: T) -> T {
    match new_value {
        Some(value) => value,
        None => old_value 
    }
}
