//build and run with cargo run
//compiles to the target folder
//mod print; // imports the print.rs file
//mod vars;
mod command_line_args;

fn main() {
    //print::run();
    //vars::run();
    command_line_args::run();
}
