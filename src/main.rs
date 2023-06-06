use std::io;

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

struct TodoList {
    project_name: String,
    todos: Vec<Todo>
}
impl TodoList {
    fn new() -> Self {
        Self { project_name: "Work - default".to_string(), todos: Vec::new() }
    }
    fn add_to_list(&mut self, td: Todo) {
        self.todos.push(td)
    }
    fn display_l(&self) {
        println!("============================================\nList\n============================================");
        for td in &self.todos {
            println!("Item = {:?}", td);
        }
        println!("============================================");
    }
    fn get_by_id(&mut self/*, id:i32*/) -> Todo {
        self.todos.pop().unwrap()
    }
}

enum Command {
    Create(Todo),
    Update(Todo),
    Archive(Todo),
    ListOut, // something like inprogress todos
}
impl Command {
    fn process(uc: String, lot: &mut TodoList) -> Self {
        match uc.trim().to_lowercase().as_str() {
            "c" => {
                let mut desc = String::new();
                let due = (1,1,2023);
                println!("Default due date added!\nEnter task description");
                io::stdin().read_line(&mut desc).expect("Failed to get desc");
                Command::Create(Todo { description: desc, status: TodoStatus::ToDo, duedate: due })
            },
            "u" => {
                println!("Enter todo id to update:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to get id!");
                Command::Update(lot.get_by_id())
            },
            "ls" => Command::ListOut,
            _ => todo!(),
        }
    }
}

fn display(lot: &Vec<Todo>) {
    println!("============================================\nList\n============================================");
    for td in lot {
        println!("Item = {:?}", td);
    }
    println!("============================================");
}

fn main() {
    let task1 = Todo {
        description: String::from("Configure hw"),
        status: TodoStatus::ToDo,
        duedate: (5,6,2023)
    };
    let mut todo_list: Vec<Todo> = Vec::new();
    todo_list.push(task1);

    let mut td_list = TodoList::new();

    loop {
        println!("\nMain menu\nEnter command!\nAvailable commands ->\n\tc - Create Todo\n\tls - List Todos\n\tu - Update todos");
        let mut user_command = String::new();
        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to get command");
        println!("Processing - {}", user_command);
        let processed_command = Command::process(user_command, &mut td_list);
        let p_c_res = match processed_command {
            Command::Create(td) => Some(Todo::create_todo(td.description, td.status, td.duedate)),
            Command::Update(td) => Some(td.update_status(TodoStatus::Wait)),
            Command::Archive(_) => todo!(),
            Command::ListOut => {
                //display(&todo_list);
                td_list.display_l();
                None
            }
        };
        //println!("Created with command ==> {:?}", p_c_res);
        if let Some(td) = p_c_res {
            //todo_list.push(td);
            td_list.add_to_list(td);
        }
    }
    //todo_list.push(p_c_res);
    //display(&todo_list);
    //println!("{:?}", task1);
    //println!("Created with constructor ==> {:?}", Todo::create_todo("Another task".to_string(), TodoStatus::ToDo, (6,6,2023)));
    //let utask1 = Todo::update_status(task1, TodoStatus::InProgress);
    //println!("Updated status on prev::> {:?}", utask1);
    //let utask2 = utask1.update_status(TodoStatus::Blocked);
    //Todo::update_status(utask2, TodoStatus::Wait);
}
