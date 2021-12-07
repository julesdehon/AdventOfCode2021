use std::fs;
use std::collections::HashMap;

struct LanternFishSimulation {
    fish_with_age: HashMap<u32, u64>,
}

impl LanternFishSimulation {
    fn new(initial_ages: &Vec<u32>) -> LanternFishSimulation {
        let mut fish_with_age = HashMap::from([
            (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)
        ]);
        for age in initial_ages {
            let x = fish_with_age.get_mut(age).unwrap();
            *x += 1;
        }
        return LanternFishSimulation {
            fish_with_age,
        }
    }

    fn pass_day(&mut self) {
        let num_new_fish = *self.fish_with_age.get(&0).unwrap();
        for i in 0..=7 {
            self.fish_with_age.insert(i, *self.fish_with_age.get(&(i + 1)).unwrap());
        }
        self.fish_with_age.insert(8, num_new_fish);
        let age_6_fish = self.fish_with_age.get_mut(&6).unwrap();
        *age_6_fish += num_new_fish;
    }

    fn num_fish(&self) -> usize {
        return self.fish_with_age.values().fold(0, |total_num_fish, num_fish| total_num_fish + *num_fish as usize);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let initial_ages: Vec<u32> = contents.split(',').map(|days| days.parse().unwrap()).collect();

    let num_fish_after_80_days = fish_after_n_days(&initial_ages, 80);
    println!("After 80 days, there were {} fish", num_fish_after_80_days);

    let num_fish_after_256_days = fish_after_n_days(&initial_ages, 256);
    println!("After 256 days, there were {} fish", num_fish_after_256_days);
}

fn fish_after_n_days(initial_ages: &Vec<u32>, num_days: u32) -> usize {
    let mut simulation = LanternFishSimulation::new(initial_ages);
    for _ in 0..num_days {
        simulation.pass_day();
    }
    return simulation.num_fish();
}