use chrono::Utc;



type TzType = Utc;

pub type DateTime = chrono::DateTime<TzType>;


/// Returns the current time
pub fn now() -> DateTime {
    TzType::now()
}