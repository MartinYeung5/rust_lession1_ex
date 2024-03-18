fn main() {

    fn task1a_z() {
        for task1_1 in b'a'..=b'z' {
            print!("{} ", task1_1 as char)
        }
        for task1_2 in b'A'..=b'Z' {
            print!("{} ", task1_2 as char);
        }
    }
    task1a_z();

    println!();

    fn task2a_z() {
        for task2 in b'A'..=b'z' {
            if task2 <=90 || task2 >=97{
            print!("{} ", task2 as char);
            }
        }
    }
    task2a_z();

    
}
