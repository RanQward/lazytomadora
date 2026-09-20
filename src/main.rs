use std::{time};

const MINUT: u64 = 60;
enum TimeState {
        Work,
        Break,
        Finished,
}

fn run_time(duration: u64, state: TimeState) -> TimeState {
     let mut new_state = match state {
        TimeState::Work => {
            println!("Работать {duration} секунд");
            TimeState::Break
        }

        TimeState::Break => {
            println!("Отдыхать {duration} секунд");
            TimeState::Work
        }

        TimeState::Finished => {
            println!("Завершить работу");
            TimeState::Finished
        }
    };

    let start = time::Instant::now();
    let seconds = time::Duration::from_secs(duration);

    while start.elapsed() < seconds {
//       let remaining = seconds.saturating_sub(start.elapsed()); оставшееся время
       println!("Прошло {:?} секунд", start.elapsed());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
   println!("Время вышло");
    new_state
}

fn timer(time: u64, state: TimeState, n: u64) {
     
    let mut new_state = state;
    let mut score: u64 = 0;

    while score < n {
        new_state = run_time(time, new_state);

        score += 1;
    }
}

fn main() {
    let current = TimeState::Work;

    timer(15, current, 3);


}
