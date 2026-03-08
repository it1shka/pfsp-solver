use pfsp_solver::solver::problem;
use std::io;

fn main() {
    let problem = {
        let handle = io::stdin().lock();
        problem::parse_problem(handle)
    };
    println!("{:#?}", problem);
}
