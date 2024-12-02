use std::collections::HashMap;
use std::fs;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("File Path: {file_path}");

    let file_content = fs::read_to_string(file_path).expect("Error, could not read file");
    
    // this is horrific there must be a way to do this without swapping back and
    // forth between lists and vectors :?
    let numbers: Vec<&str>  = file_content.split_whitespace().collect();
    
    // different versions
    let mut list1: Vec<i32> = [].to_vec();
    let mut list2: Vec<i32> = Vec::with_capacity(0);
    let mut map2: HashMap<i32, i32> =  HashMap::new();


    for pair in numbers.chunks(2)
    {
        list1.push(pair[0].parse::<i32>().unwrap());
        let int2 = pair[1].parse::<i32>().unwrap();
        list2.push(int2);
        
        // Get value with key int2 or insert new value of 0
        // then add 1
        *map2.entry(int2).or_insert(0) += 1;
    }

    list1.sort();
    list2.sort();
    
    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for i in 0..(list1.len())
    {
        part1_sum += (list1[i] - list2[i]).abs();

        if map2.contains_key(&list1[i])
        {
            part2_sum += list1[i] * map2.get(&list1[i]).unwrap();
        }
    }
    
    println!("Part1: {part1_sum} \nPart2:{part2_sum}");

}