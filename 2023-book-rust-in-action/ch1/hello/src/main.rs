fn main() {
    println!("Hello, world!");
    let regions = ["China", "US"];
    for region in regions {
        println!("Region: {}", region);
    }

    let raw_data = "\
    name, height
    Alice, 170
    Bob, 180
    ";

    let records = raw_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0{
            continue;
        }
        let fields: Vec<&str> = record.split(',').map(|field|field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("fields: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // let height = fields[1].parse::<i32>().unwrap();
        if let Ok(height) = fields[1].parse::<i32>() {
            println!("{} is {} cm tall", name, height);
        } else {
            println!("{} is not a valid height", name);
        }
    }
}
