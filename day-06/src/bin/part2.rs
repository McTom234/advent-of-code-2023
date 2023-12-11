struct Race {
    distance: i64,
    time: i64,
}

const RACES: [Race; 4] = [
    Race {
        time: 49,
        distance: 263,
    },
    Race {
        time: 97,
        distance: 1532,
    },
    Race {
        time: 94,
        distance: 1378,
    },
    Race {
        time: 94,
        distance: 1851,
    },
];

fn main() {
    let mut final_race = Race {
        time: 49979494,
        distance: 263153213781851,
    };
    let mut successful_tries = 0;
    for i in 1..final_race.time {
        let a = distance(i, final_race.time) - final_race.distance;
        successful_tries = if a > 0 {
            successful_tries + 1
        } else {
            successful_tries
        };
    }
    println!("sum: {}", successful_tries);
}

fn distance(a: i64, time: i64) -> i64 {
    // ax - a^2 -9 ; a = acceleration, x = time
    return (a * time) - (a * a);
}
