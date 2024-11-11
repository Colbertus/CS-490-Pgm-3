/*
    Author: Colby McClure
    CS 490 Pgm 2
    Date: 10/23/2024
    Dependency: rand = "0.8.5"
    Environment: VS Code on Windows 11 
*/

// Needed crates and dependencies
use std::io;
use std::collections::BinaryHeap;
use rand::Rng;
use std::thread; 
use std::sync::{Arc, Mutex};

// The Process struct contains an id, priority, sleep_time, and description field
struct Process {
    id: i32,
    priority: i32,
    sleep_time: u64,
}

impl Ord for Process {

    // Use the cmp method to compare the priority field of two Process instances
    // The output is an Ordering enum that represents the ordering of the two instances
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

        // Reverse the order to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

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

impl PartialOrd for Process {

    // Use the partial_cmp method to compare the priority field of two Process instances
    // The output is an Option enum that contains the ordering of the two instances
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        // Reverse the order to create a min-heap
        Some(other.priority.cmp(&self.priority))
    }
}

fn main() {

    let mut phases = String::new();
    println!("Hello! Please enter the number of generation phases for the producer simulation: ");

    io::stdin()
        .read_line(&mut phases)
        .expect("Failed to read input");

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

    let num_processes: i32 = match num_processes.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("Starting the Simulation...");

    let binary_heap = Arc::new(Mutex::new(BinaryHeap::new()));
    let producer_heap = Arc::clone(&binary_heap);
    let producer = thread::spawn(move || {

        let mut process_id = 0;

        print!("...producer is starting its work...");
        for _ in 0..phases {

            for _ in 0..num_processes {
                process_id += 1;
                let process = Process {
                    id: process_id,
                    priority: rand::thread_rng().gen_range(0..100),
                    sleep_time: rand::thread_rng().gen_range(200..2000)
                };
                let mut heap = producer_heap.lock().unwrap();
                heap.push(process);
            }

            println!("...producer is now sleeping...");
            thread::sleep(std::time::Duration::from_millis(sleep_time));
        }
        println!("...producer has finished with a total of {} processes generated...", process_id);
    });

    thread::sleep(std::time::Duration::from_millis(500));

    let consumer_heap1 = Arc::clone(&binary_heap);

    let consumer1 = thread::spawn(move || {
        
        let mut num_processes = 0;
        loop {
            let mut heap = consumer_heap1.lock().unwrap();
            let process = heap.pop();
            if let Some(process_ref) = process.as_ref() {
                drop(heap);
                num_processes += 1;
                thread::sleep(std::time::Duration::from_millis(process_ref.sleep_time));
            }
            match process {
                Some(p) => {
                    println!("Consumer 1: Process Node {} with priority {} has been executed for {} ms", p.id, p.priority, p.sleep_time);
                },
                None => {
                    break;
                }
            }
        }
        println!("Consumer 1 has finished processing and has executed {} processes", num_processes);
    });

    let consumer_heap2 = Arc::clone(&binary_heap);

    let consumer2 = thread::spawn(move || {
        
        let mut num_processes = 0;
        loop {
            let mut heap = consumer_heap2.lock().unwrap();
            let process = heap.pop();
            if let Some(process_ref) = process.as_ref() {
                drop(heap);
                num_processes += 1;
                thread::sleep(std::time::Duration::from_millis(process_ref.sleep_time));
            }
            match process {
                Some(p) => {
                    println!("Consumer 2: Process Node {} with priority {} has been executed for {} ms", p.id, p.priority, p.sleep_time);
                },
                None => {
                    break;
                }
            }
        }
        println!("Consumer 2 has finished processing and has executed {} processes", num_processes);
    });

    producer.join().unwrap();
    consumer1.join().unwrap();
    consumer2.join().unwrap();

    println!("Both consumers have finished their work. Simulation complete!");

}
