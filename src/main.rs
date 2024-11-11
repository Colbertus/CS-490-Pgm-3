/*
    Author: Colby McClure
    CS 490 Pgm 3
    Date: 11/20/2024
    Dependency: rand = "0.8.5"
    Environment: VS Code on Windows 11
*/

// Needed crates and dependencies
use rand::Rng;
use std::collections::BinaryHeap;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

// The Process struct contains an id, priority, and sleep_time
struct Process {
    id: i32,
    priority: i32,
    sleep_time: u64,
}

// The Ord trait is used to compare two instances of a type
// For this use case, we want to compare the priority field of two Process instances
// We reverse the order to create a min-heap
impl Ord for Process {
    // Use the cmp method to compare the priority field of two Process instances
    // The output is an Ordering enum that represents the ordering of the two instances
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

// The Eq trait is used to compare two instances of a type for equality
// We keep this empty since the Eq trait is automatically implemented for types that implement the PartialEq trait
impl Eq for Process {}

// The PartialEq trait is used to compare two instances of a type for equality
// For this use case, we want to compare the priority field of two Process instances
impl PartialEq for Process {
    // Use the eq method to compare the priority field of two Process instances
    // The output is a boolean value that represents the equality of the two instances
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

// The PartialEq trait is used to compare two instances of a type for equality
// For this use case, we want to compare the priority field of two Process instances
impl PartialOrd for Process {
    // Use the partial_cmp method to compare the priority field of two Process instances
    // The output is an Option enum that contains the ordering of the two instances
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse the order to create a min-heap
        Some(other.priority.cmp(&self.priority))
    }
}

fn main() {
    // Start by taking in input for the phases, sleep time, and number of processes, respectively
    let mut phases = String::new();
    println!("Hello! Please enter the number of generation phases for the producer simulation: ");

    io::stdin()
        .read_line(&mut phases)
        .expect("Failed to read input");

    // Make sure that the input is a valid number
    let phases: i32 = match phases.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let mut sleep_time = String::new();

    println!("Please enter the sleep time for the producer to pause between generation phases: ");

    io::stdin()
        .read_line(&mut sleep_time)
        .expect("Failed to read input");

    // Make sure the input is a valid number (has to be u64 for thread::sleep)
    let sleep_time: u64 = match sleep_time.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let mut num_processes = String::new();

    println!("Please enter the number of processes to generate in each phase: ");

    io::stdin()
        .read_line(&mut num_processes)
        .expect("Failed to read input");

    // Make sure the input is a valid number
    let num_processes: i32 = match num_processes.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("Starting the Simulation...");

    // Initialize a binary heap to store the processes
    // This binary heap has a wrapper of Arc and Mutex to allow for shared ownership and thread safety
    let binary_heap = Arc::new(Mutex::new(BinaryHeap::new()));

    // Create a producer heap that is a clone of the binary heap to allow for shared ownership
    let producer_heap = Arc::clone(&binary_heap);

    // The start of the producer thread
    let producer = thread::spawn(move || {
        // Initialize a process_id variable to keep track of the number of processes generated along with id info for the process
        let mut process_id = 0;

        print!("...producer is starting its work...");

        // For each phase that was entered...
        for _ in 0..phases {
            // For each process that gets generated in the phase...
            for _ in 0..num_processes {
                // Increment the process_id and generate a new process with random values for priority and sleep_time
                process_id += 1;
                let process = Process {
                    id: process_id,
                    priority: rand::thread_rng().gen_range(0..100),
                    sleep_time: rand::thread_rng().gen_range(200..2000),
                };

                // Lock the heap and push the process into the binary heap
                let mut heap = producer_heap.lock().unwrap();
                heap.push(process);

                // Drop the lock to allow other threads to access the heap after pushing the process
                drop(heap);
            }

            // Print the sleep message and pause the producer thread for the specified sleep time
            println!("...producer is now sleeping...");
            thread::sleep(std::time::Duration::from_millis(sleep_time));
        }

        // After producing everything, print the final producer message
        println!(
            "...producer has finished with a total of {} processes generated...",
            process_id
        );
    });

    // Pause the main thread for 500 ms to allow the producer to continue generating processes
    thread::sleep(std::time::Duration::from_millis(500));

    // Create a consumer heap that is a clone of the binary heap to allow for shared ownership
    let consumer_heap1 = Arc::clone(&binary_heap);

    // The start of the first consumer thread
    let consumer1 = thread::spawn(move || {
        // Start by initializing a variable to keep track of the number of processes executed
        let mut num_processes = 0;

        // Loop through the binary heap and pop each process to execute
        loop {
            // Lock the heap and pop the process from the binary heap
            let mut heap = consumer_heap1.lock().unwrap();
            let process = heap.pop();

            // If the process exists, drop the lock and pause the thread for the specified sleep time
            // Have to use as_ref() to check if the process exists, this will return a reference to the process
            if let Some(process_ref) = process.as_ref() {
                drop(heap);
                num_processes += 1;
                thread::sleep(std::time::Duration::from_millis(process_ref.sleep_time));
            }

            // If a process was not found, break out of the loop, otherwise print the process details
            match process {
                Some(p) => {
                    println!(
                        "Consumer 1: Process Node {} with priority {} has been executed for {} ms",
                        p.id, p.priority, p.sleep_time
                    );
                }
                None => {
                    break;
                }
            }
        }

        // After the consumer has finished, print the final message with the number of processes executed
        println!(
            "Consumer 1 has finished processing and has executed {} processes",
            num_processes
        );
    });

    // Initialize the second consumer heap that is a clone of the binary heap to allow for shared ownership
    let consumer_heap2 = Arc::clone(&binary_heap);

    // The start of the second consumer thread
    let consumer2 = thread::spawn(move || {
        // Initialize a variable to keep track of the number of processes executed
        let mut num_processes = 0;

        // Loop through the binary heap and pop each process to execute
        loop {
            // Lock the heap and pop the process from the binary heap
            let mut heap = consumer_heap2.lock().unwrap();
            let process = heap.pop();

            // If the process exists, drop the lock and pause the thread for the specified sleep time
            if let Some(process_ref) = process.as_ref() {
                drop(heap);
                num_processes += 1;
                thread::sleep(std::time::Duration::from_millis(process_ref.sleep_time));
            }

            // If a process was not found, break out of the loop, otherwise print the process details
            match process {
                Some(p) => {
                    println!(
                        "Consumer 2: Process Node {} with priority {} has been executed for {} ms",
                        p.id, p.priority, p.sleep_time
                    );
                }
                None => {
                    break;
                }
            }
        }

        // After the consumer has finished, print the final message with the number of processes executed
        println!(
            "Consumer 2 has finished processing and has executed {} processes",
            num_processes
        );
    });

    // Wait for the threads to finish their work before terminating the program
    producer.join().unwrap();
    consumer1.join().unwrap();
    consumer2.join().unwrap();

    // Final message to indicate that the simulation has completed
    println!("Both consumers have finished their work. Simulation complete!");
}
