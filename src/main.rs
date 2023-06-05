#[derive(Debug)]
struct Todo {
    description: String,
    status: TodoStatus,
    duedate: (i32,i32,i32),
}
impl Todo {
    fn create_todo(desc: String, stat: TodoStatus, due: (i32, i32, i32)) -> Self {
        Self {
            description: desc,
            status: stat,
            duedate: due,
        }
    }

    fn update_status(self, newstatus: TodoStatus) -> Self {
        match self.status {
            TodoStatus::Done => todo!(),
            _ => Self { status: newstatus, ..self }
        }
    }
}

#[derive(Debug)]
enum TodoStatus {
    ToDo,
    InProgress,
    Blocked,
    Wait,
    Done,
}

fn main() {
    let task1 = Todo {
        description: String::from("Configure hw"),
        status: TodoStatus::ToDo,
        duedate: (5,6,2023)
    };

    println!("{:?}", task1);
    println!("Created with constructor ==> {:?}", Todo::create_todo("Another task".to_string(), TodoStatus::ToDo, (6,6,2023)));
    let utask1 = Todo::update_status(task1, TodoStatus::InProgress);
    println!("Updated status on prev::> {:?}", utask1);
    let utask2 = utask1.update_status(TodoStatus::Blocked);
    Todo::update_status(utask2, TodoStatus::Wait);
    println!("Hello, world!");
}
