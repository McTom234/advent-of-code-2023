struct Race {
    distance: i32,
    time: i32,
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
    let mut sum = 1;
    for race in RACES {
        let mut successful_tries = 0;
        for i in 1..race.time {
            let a = distance(i, race.time) - race.distance;
            successful_tries = if a > 0 {
                successful_tries + 1
            } else {
                successful_tries
            };
        }
        println!("{}", successful_tries);
        sum *= successful_tries;
    }
    println!("sum: {}", sum);
}

fn distance(a: i32, time: i32) -> i32 {
    // ax - a^2 -9 ; a = acceleration, x = time
    return (a * time) - (a * a);
}
