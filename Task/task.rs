use std::collections::VecDeque;

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: &str) -> Self {
        static mut NEXT_ID: u32 = 0;
        unsafe {
            NEXT_ID += 1;
        }
        Task {
            id: NEXT_ID,
            description: String::from(description),
            completed: false,
        }
    }
}

struct ToDoList {
    tasks: VecDeque<Task>,
}

impl ToDoList {
    fn add_task(&mut self, description: &str) -> Task {
        let new_task = Task::new(description);
        self.tasks.push_back(new_task);
        new_task
    }

    fn complete_task(&mut self, id: u32) -> Option<&mut Task> {
        let mut index = None;
        for (i, task) in self.tasks.iter_mut().enumerate() {
            if task.id == id {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            self.tasks[index].completed = true;
            Some(&mut self.tasks[index])
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        println!("** To-Do Listesi **");
        for task in &self.tasks {
            println!("** Görev ID: {} **", task.id);
            println!("  - Açıklama: {}", task.description);
            println!("  - Tamamlandı: {}", if task.completed { "Evet" } else { "Hayır" });
            println!();
        }
    }
}

fn main() {
    let mut to_do_list = ToDoList { tasks: VecDeque::new() };

    // Görev ekleme
    to_do_list.add_task("Ödevini bitir");
    to_do_list.add_task("Dişlerini fırçala");
    to_do_list.add_task("Spora git");

    // Görevi tamamlama
    to_do_list.complete_task(1); // "Rust ödevini bitir" görevini tamamlar

    // To-do listesini yazdırma
    to_do_list.list_tasks();
}
