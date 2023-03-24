use entity::sea_orm::Set;

pub fn mutation_value_check<T: Into<sea_orm::Value>>(input_value:Option<T>, original_value: sea_orm::ActiveValue<T>) -> sea_orm::ActiveValue<T> {
    match input_value {
        None => original_value,
        Some(value) => Set(value)
    }
}
