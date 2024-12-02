use std::fs;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("File Path: {file_path}");

    let file_content = fs::read_to_string(file_path).expect("Error, could not read file");

    let mut sum = 0;
    for row in file_content.split('\n')
    {
        let mut sign: i32 = 0;
        let mut valid = true;

        let mut items = row.split(' ').peekable();

        while let Some(item) = items.next() {
            let next = items.peek();
            let diff;
            if next == None
            {
                continue;
            }
            else
            {
                diff = next.unwrap().parse::<i32>().unwrap() - item.parse::<i32>().unwrap();
            }
            
            if sign == 0
            {
                if diff > 0
                {
                    sign = 1;
                }
                else if diff < 0
                {
                    sign = -1;
                }
            }
            
            let check_value = sign * diff;
            
            if check_value <= 0 || check_value.abs() > 3
            {
                valid = false;
                break;
            }
        }
        if valid
        {
            println!("{row}");
            sum += 1;
        }
    }

    println!("{sum}");

}