use entity::sea_orm::Set;
use sea_orm::ActiveValue::NotSet;

pub fn mutation_value_check<T: Into<sea_orm::Value>>(input_value:Option<T>) -> sea_orm::ActiveValue<T> {
    match input_value {
        None => NotSet,
        Some(value) => Set(value)
    }
}
