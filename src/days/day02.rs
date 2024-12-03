struct Report {
    levels: Vec<u32>,
    is_increasing: bool,
    is_decreasing: bool,
    is_safe: bool,
}

impl Report {
    fn check_direction(&mut self) -> &Self {
        let vals = &self.levels;
        match vals[0].cmp(&vals[1]) {
            std::cmp::Ordering::Less => self.is_increasing = true,
            std::cmp::Ordering::Equal => self.is_safe = false,
            std::cmp::Ordering::Greater => self.is_decreasing = true,
        }
        self
    }

    fn check_safety(&self) -> bool {
        if self.is_safe == false {
            return false;
        }
        let vals = &self.levels;
        if self.is_increasing {
            for i in 0..vals.len() - 1 {
                if vals[i + 1] <= vals[i] {
                    return false;
                }
                println!("{} - {} = {}", vals[i + 1], vals[i], vals[i + 1] - vals[i]);
                let ret = vals[i + 1] - vals[i] >= 1 && vals[i + 1] - vals[i] <= 3;

                if ret == false {
                    return false;
                }
            }
        } else if self.is_decreasing {
            for i in 0..vals.len() - 1 {
                if vals[i + 1] >= vals[i] {
                    return false;
                }
                println!("{} - {} = {}", vals[i], vals[i + 1], vals[i] - vals[i + 1]);
                let ret = vals[i] - vals[i + 1] >= 1 && vals[i] - vals[i + 1] <= 3;

                if ret == false {
                    return false;
                }
            }
        }

        true
    }

    fn set_safety(&mut self, safety: bool) -> &Self {
       self.is_safe = safety;
       self
    }
}

pub fn find_solution() {
    let contents = std::fs::read_to_string("src/inputs/02/day02.txt");
    let mut result = 0;
    for line in contents.unwrap().lines() {
        let vals: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        let mut n_report = Report {
            levels: vals,
            is_increasing: false,
            is_decreasing: false,
            is_safe: true,
        };

        let n_report = n_report.check_direction();

        let safety = n_report.check_safety();
        if safety {
            result += 1;
        }

        println!("{}, {}", line, n_report.is_safe);
    }

    println!("The total number of safe reports is {}", result);
}
