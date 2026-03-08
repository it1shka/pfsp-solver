use pfsp_solver::solver::problem::Problem;
use std::io;

fn main() {
    let problem = {
        let handle = io::stdin().lock();
        Problem::parse(handle)
    };
    println!("{:#?}", problem);
}
