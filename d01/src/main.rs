fn main() {
    let input = {
        let fname = std::env::args()
            .nth(1)
            .expect("Please give input as first argument!");
        std::fs::read_to_string(fname).unwrap()
    };

    // part one
    let module_fuel: i32 = input
        .lines()
        .map(|mass| mass.parse::<i32>().unwrap())
        .map(|mass| mass / 3 - 2)
        .sum();
    println!("Module fuel required: {}", module_fuel);

    // part one
    let total_fuel: i32 = input
        .lines()
        .map(|mass| mass.parse::<i32>().unwrap())
        .map(|mut mass| {
            let mut total = 0;
            loop {
                mass = mass / 3 - 2;
                if mass < 0 {
                    break;
                }
                total += mass;
            }
            total
        })
        .sum();
    println!("Total fuel required: {}", total_fuel);
}
