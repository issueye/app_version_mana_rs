use nanoid::nanoid;
use uuid::Uuid;

// 获取 随机 id
pub fn get_id() -> String {
    nanoid!()
}

// 获取 uuid
pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}