use clap::Parser;
use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(about = "Test your luck with dice rolls!")]
struct Args {
    #[arg(short, long, help("Number of frames"), default_value_t = 18)]
    frames: i32,

    #[arg(short, long, help("Seconds until the result"), default_value_t = 1.8)]
    seconds: f64,

    #[arg(short, long, help("For the impatient: Automatically roll the dice because waiting is too hard..."), default_value_t = false)]
    auto: bool
}

// Get the corresponding ASCII art for each dice roll and combine them into a string
fn generate_dice_string(roll_list: &Vec<i32>) -> String {
    const DICE: [&str; 6] = [
        "[ . ]",
        "[ : ]",
        "[...]",
        "[: :]",
        "[:.:]",
        "[:::]",
    ];

    let result: Vec<String> = roll_list.iter()
        .map(|&roll| DICE[(roll - 1) as usize].to_string())
        .collect();
    
    result.join(" ")
}

fn main() {
    let args = Args::parse();

    // Initialize frames and loop_duration with validation
    let frames = if args.auto { 5 } else if args.frames >= 1 { args.frames } else { 18 } ;
    let loop_duration = if args.auto { 25 } else { (1000.0 * args.seconds) as i32 / frames };

    // Ensure loop duration is at least 1 millisecond
    let loop_duration = if loop_duration >= 1 { loop_duration } else { 100 };

    // Output introduction
    println!("Welcome to the game of 'Roll Until You Get a Triplet'!");
    println!("Here’s how it works: Press Enter over and over, and keep hoping for the magic triplet...\n");
    println!("***************************************************");
    println!("********* WARNING: Proceed with caution! **********");
    println!("** Pressing Enter before the dice have settled ****");
    println!("** will cause chaos! Take a deep breath and wait **");
    println!("** for the dice to decide. Patience is key! *******");
    println!("** Can’t wait? Try '-a' and see what happens... ***");
    println!("***************************************************\n");

    loop {
        let mut rng = rand::rng();

        // Generate the final dice rolls
        let roll_list: Vec<i32> = (0..3)
            .map(|_| rng.random_range(1..=6))
            .collect();

        // Start the slot machine-like animation
        for frame in 0..frames {
            let dice_counter = (frame + 1) * 3 / frames;

            // Keep the front part of the roll_list intact
            let front_part = roll_list.iter().take(dice_counter as usize).cloned().collect::<Vec<i32>>();

            // Generate the remaining part randomly
            let back_part: Vec<i32> = (0..(3 - dice_counter))
                .map(|_| rng.random_range(1..=6))
                .collect();

            // Combine both parts to create roll_list_for_display
            let mut roll_list_for_display = front_part;
            roll_list_for_display.extend(back_part);

            // Display the dice on the screen
            print!("\r{} ", generate_dice_string(&roll_list_for_display));
            io::stdout().flush().unwrap();

            // Wait for the specified duration before the next frame
            sleep(Duration::from_millis(loop_duration as u64));
        }

        // If a triplet is rolled, display a message and exit the loop
        if roll_list.iter().all(|&roll| roll == roll_list[0]){
            println!("\nA triple {}! The rarest of them all, you've rolled a legendary set! Congratulations, you're on fire!", roll_list[0]);
            break;
        }

        // Wait for user input to roll again, or automatically continue if auto mode is enabled
        if args.auto{
            println!("");
        } else {
            let _ = io::stdin().read_line(&mut String::new());
        }
    }
}
