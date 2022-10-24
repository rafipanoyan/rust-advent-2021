use data::data::INPUT;

mod data;

#[derive(Clone, Copy, Debug)]
struct Lanternfish {
    timer: i8
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish { timer: 8 }
    }
    
    fn from(timer: i8) -> Lanternfish {
        Lanternfish { timer }
    }

    fn tick(&mut self) -> Option<Lanternfish> {
        let current_timer = self.timer;
        let new_timer = current_timer - 1;

        if new_timer < 0 {
            self.timer = 6;
            Some(Lanternfish::new())
        } else {
            self.timer = new_timer;
            None
        }
    }
}

fn main() {
    let sea = INPUT.split(',').into_iter().map(|timer| {
        Lanternfish::from(timer.parse().unwrap())
    }).collect::<Vec<Lanternfish>>();

    let days = 2;

    let final_fishes = pass_day(sea, days);

    println!("After {} days: {} fishes", days, final_fishes.len());
}

fn pass_day(mut sea: Vec<Lanternfish>, days: i32) -> Vec<Lanternfish> {
    println!("Day : {}, Input : {:?}", days, sea);
    if days < 0 {
        return sea;
    } else {
        let mut new_fishes: Vec<Lanternfish> = sea.iter_mut().map(|fish| {
            fish.tick()
       }).flatten().collect();
       new_fishes.extend(sea.iter().map(|f| { f.clone() }).collect::<Vec<Lanternfish>>());
       return pass_day(new_fishes, days - 1);
    }
}