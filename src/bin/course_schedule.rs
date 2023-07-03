pub fn main() {
    println!("Course Schedule");
}

#[derive(Clone, Copy)]
enum Status {
    PendingCheck,
    CheckInProgress,
    Done,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    let mut graph = vec![Vec::new(); num_courses];

    for edge in prerequisites.iter() {
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    let mut status = vec![Status::PendingCheck; num_courses];

    (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph))
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
    match status[course] {
        Status::Done => false,
        Status::CheckInProgress => true,
        Status::PendingCheck => {
            status[course] = Status::CheckInProgress;
            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, status, graph))
            {
                return true;
            }

            status[course] = Status::Done;
            false
        }
    }
}
