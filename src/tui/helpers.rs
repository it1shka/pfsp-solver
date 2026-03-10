use pfsp_solver::solver::solution::Solution;

pub fn parse_solution(raw_solution: &str) -> Option<Solution> {
    raw_solution
        .split_whitespace()
        .map(|chunk| str::parse::<usize>(chunk))
        .collect::<Result<Vec<_>, _>>()
        .ok()
        .map(|result| Solution(result))
}
