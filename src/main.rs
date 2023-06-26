use std::io;

#[derive(Debug)]
struct Todo {
    description: String,
    status: TodoStatus,
    duedate: (i32,i32,i32),
}
impl Todo {
    fn create_todo(desc: String, due: (i32, i32, i32)) -> Self {
        Self {
            description: desc,
            status: TodoStatus::init(),
            duedate: due,
        }
    }

    fn update_status(self) -> Self {
        Self { status: self.status.toggle(), ..self }
    }
}

#[derive(Debug)]
enum TodoStatus {
    ToDo,
    InProgress,
    Blocked,
    Done,
    Closed,
}
impl TodoStatus {
    fn init() -> Self {
        TodoStatus::ToDo
    }

    fn toggle(self) -> Self {
        println!("Which status to toggle to?\ntd - ToDo\nip - InProgress\nb - Blocked\nd - Done\nc - Closed");
        let mut ns = String::new();
        io::stdin()
            .read_line(&mut ns)
            .expect("Failed to read command");
        match ns.trim().to_lowercase().as_str() {
            "td" => self,
            "ip" => match self {
                Self::ToDo => Self::InProgress,
                Self::InProgress => Self::InProgress,
                Self::Blocked => Self::InProgress,
                Self::Done => Self::InProgress,
                Self::Closed => Self::InProgress,
            },
            "b" => match self {
                Self::ToDo => Self::Blocked,
                Self::InProgress => Self::Blocked,
                Self::Blocked => Self::Blocked,
                Self::Done => Self::Done,
                Self::Closed => Self::Closed,
            },
            "d" => match self {
                Self::ToDo => Self::ToDo,
                Self::InProgress => Self::Done,
                Self::Blocked => Self::Blocked,
                Self::Done => Self::Done,
                Self::Closed => Self::Closed,
            },
            "c" => match self {
                Self::ToDo => Self::Closed,
                Self::InProgress => Self::Closed,
                Self::Blocked => Self::Closed,
                Self::Done => Self::Done,
                Self::Closed => Self::Closed,
            },
            _ => todo!(),
        }
    }
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
        println!("============================================\nProject: {:?}\n============================================", self.project_name);
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
                Command::Create(Todo::create_todo(desc, due))
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
            Command::Create(td) => Some(td),
            Command::Update(td) => Some(td.update_status()),
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
