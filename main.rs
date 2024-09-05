use std::io;



fn main() {

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1. Add Task\n2. List Tasks\n3. Mark Task Completed\n4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => add_task(&mut tasks),
            2 => list_tasks(&tasks),
            3 => complete_task(&mut tasks),
            4 => break,
            _ => continue,
        }
}

}
struct Task {
    name: String,
    completed: bool,
}

fn add_task(tasks:&mut Vec<Task>){
    println!("Enter a name of the task:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    tasks.push(Task{
        name: name.trim().to_string(),
        completed: false,
    })

}

fn list_tasks(tasks: &Vec<Task>){
    for (i, task) in tasks.iter().enumerate(){
        let status = if task.completed {"Done"} else {"Pending"};
        println!("{}. {} [{}]", i + 1, task.name, status )
    }

}

fn complete_task(tasks:&mut Vec<Task>) {
    list_tasks(tasks);
    println!("Enter a task number to mark as complete");

    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Failed to read input");

    let task_number: usize = match task_number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if task_number > 0 && task_number <= tasks.len() {
        tasks[task_number - 1].completed = true;
    } else {
        println!("Choose a correct task number");
        
    } 




}