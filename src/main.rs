mod modules;

fn main() {
    modules::banner::print_banner();
    modules::url_validation::ask_for_target();
}