use chrono::prelude::*;
use rand;
use rand::Rng;

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    category_id: String,
    due_date: u32,
    id: String,
    global_task_id: String,
    note: String,
    parent_global_task_id: String,
    status: TaskStatus,
    title: String,
}

#[derive(Debug)]
#[derive(Deserialize)]
enum TaskStatus {
    CHKECKED,
    UNCHECKED
}

#[derive(Debug)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskReq {
    id: String,
    title: String,
    due_date: i64,
    note: String,
}

impl TaskReq {
    #[allow(dead_code)]
    pub fn new(title: String, due_date: DateTime<Local>, note: String) -> TaskReq {
        TaskReq {
            id: gen_random_id(),
            title,
            due_date: due_date.timestamp(),
            note
        }
    }
}

fn gen_random_id() -> String {
    let mut rng = rand::thread_rng();
    return rng.gen_ascii_chars().take(16).collect()
}

#[test]
fn test_gen_random_id_length() {
    assert_eq!(gen_random_id().len(), 16);
}