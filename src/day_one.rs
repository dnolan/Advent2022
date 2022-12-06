fn day_one() {
    let mut elves: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    let mut biggest: i32 = 0;
    let mut count: i32 = 1;

    if let Ok(lines) = utility::read_lines("data.txt") {
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    elves.push(total);

                    println!("Elf {} has {} calories", count, total);
                    count += 1;

                    if total > biggest {
                        biggest = total;
                        println!("Bigger:{}", biggest);
                    }

                    total = 0;
                    continue;
                }

                total += calories.parse::<i32>().unwrap();
            }
        }
    }

    println!("Biggest {}", biggest);

    
    utility::bubble_sort(&mut elves);

    println!("Top 3 = {}, {}, {}", elves[elves.len()-1], elves[elves.len()-2], elves[elves.len()-3]);

    println!("Total Top 3 {}", elves[elves.len()-1]+elves[elves.len()-2]+elves[elves.len()-3]);
}