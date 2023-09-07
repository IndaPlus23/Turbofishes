/**
 * Solution to '<task-title>'
 * @see https://kth.kattis.com/problems/<problem-title>
 * @see https://open.kattis.com/problems/<problem-title>
 * 
 * Task #x -- DD13XX <inda-course-name>
 * 
 * @author <name> <<kth-id>@kth.se>
 * @created <date>
 * @last-edited <date>
 */

mod helper;

use std::io::{prelude::*, self};

fn main() {
    // Kattis submission:
    println!("{}", solve_problem(helper::read_input(io::stdin())));
}

fn solve_problem(input: String) -> String {
    // Problem solution...
}

mod tests {
    use super::{helper, solve_problem};

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn local_test() {
        let (input_buffer, target_output_buffer) = helper::get_io(/*<test-name>*/);
        let output = solve_problem(helper::read_input(input_buffer));
        // Assert output against target output...
    }
}