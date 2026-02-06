fn ass1(){
    const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}
     // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;

    // Convert it to Celsius and print
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);

    println!("--- Calculating next 5 degrees ---");

    // Loop 5 times to print the next integer temperatures
    // We iterate from 1 to 5
    for i in 1..6 {
        // We create a new f64 based on the original + loop index
        // We act as if the input is an integer (e.g. 33.0, 34.0)
        let next_f = temp_f + (i as f64); 
        let converted = fahrenheit_to_celsius(next_f);
        println!("{}°F is {:.2}°C", next_f, converted);
    }
}

fn ass2(){
    fn is_even(n: i32) -> bool {
    n % 2 == 0
}
// Create an array of 10 integer numbers
    let numbers = [1, 2, 3, 4, 5, 9, 10, 15, 23, 30];

    println!("--- Analyzing Numbers ---");
    
    // Use a for loop to iterate through the array
    for n in numbers {
        // We use if-else if-else to prioritize FizzBuzz logic
        if n % 3 == 0 && n % 5 == 0 {
            println!("{}: FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{}: Fizz", n);
        } else if n % 5 == 0 {
            println!("{}: Buzz", n);
        } else {
            // Only check even/odd if it wasn't Fizz/Buzz/FizzBuzz
            // (Based on the 'instead' instruction in the prompt)
            let even = is_even(n);
            if even {
                println!("{}: Even", n);
            } else {
                println!("{}: Odd", n);
            }
        }
    }

    println!("--- Calculating Sum (While Loop) ---");

    // Use a while loop to find the sum
    let mut sum = 0;
    let mut i = 0;
    while i < 10 {
        sum = sum + numbers[i];
        i = i + 1;
    }
    println!("The sum of the numbers is: {}", sum);

    println!("--- Finding Largest (Loop) ---");

    // Use a basic loop to find the largest number
    let mut max = numbers[0]; // Start assuming first is largest
    let mut j = 0;
    
    loop {
        if j == 10 {
            break;
        }
        
        if numbers[j] > max {
            max = numbers[j];
        }
        
        j = j + 1;
    }
    println!("The largest number is: {}", max);
}

fn ass3(){
    fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}
let secret_number = 42;
    // We start with a simulation variable. 
    // Let's pretend the user starts guessing at 0.
    let mut current_guess = 0; 
    let mut attempts = 0;

    println!("Secret number is hidden. Starting simulation...");

    loop {
        attempts = attempts + 1;
        
        // Call the check function
        let result = check_guess(current_guess, secret_number);

        // Use if-else expression to print status and update the "simulated" user input
        if result == 0 {
            println!("Guess {}: Correct! You found the number.", current_guess);
            break;
        } else if result == 1 {
            println!("Guess {}: Too high!", current_guess);
            // Simulate user choosing a lower number
            current_guess = current_guess - 1; 
        } else {
            println!("Guess {}: Too low!", current_guess);
            // Simulate user choosing a higher number
            current_guess = current_guess + 1;
        }
    }

    println!("Game Over. It took {} guesses.", attempts);
}

fn main() {
    ass1();
    ass2();
    ass3();
}
