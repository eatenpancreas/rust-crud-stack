const DB_URL: &str = env!("DATABASE_URL");

fn main() {
    std::fs::read_dir("../migrations").expect("Cannot find migrations directory!");
    println!("{}", DB_URL);
}
