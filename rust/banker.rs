//#[allow(unused_imports)]
//#[allow(unused_variables)]
#[allow(dead_code)]

mod rust_examples {

    use std::sync::{Arc, Mutex, Condvar};

    pub struct BankerAlgorithm<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> {
        m_monitor_mutex: Arc<Mutex<BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES>>>,
        m_monitor_cv: Vec<Arc<Condvar>>,
    }

    struct BankerAlgorithmData<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> {
        m_num_resources: usize,
        m_num_processes: usize,
        m_resources: [usize; NUM_RESOURCES],
        m_claim: [[usize; NUM_RESOURCES]; NUM_PROCESSES],
        m_available: [usize; NUM_RESOURCES],
        m_alloc: [[usize; NUM_RESOURCES]; NUM_PROCESSES],
        m_running: [bool; NUM_PROCESSES],
    }
    
    impl<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES>  {
    
        fn new(resources: [usize; NUM_RESOURCES], claim: [[usize; NUM_RESOURCES]; NUM_PROCESSES]) -> BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES> {
            BankerAlgorithmData {
                m_num_resources: NUM_RESOURCES,
                m_num_processes: NUM_PROCESSES,
                m_resources: resources.clone(),
                m_available: resources.clone(),
                m_claim: claim.clone(),
                m_alloc: [[0; NUM_RESOURCES]; NUM_PROCESSES],
                m_running: [true; NUM_PROCESSES],
            }
        }
    }

    impl<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> BankerAlgorithm<NUM_RESOURCES, NUM_PROCESSES> {


        pub fn new(resources: [usize; NUM_RESOURCES], claim: [[usize; NUM_RESOURCES]; NUM_PROCESSES]) -> BankerAlgorithm<NUM_RESOURCES, NUM_PROCESSES> {
            BankerAlgorithm {
                m_monitor_mutex: Arc::new(Mutex::new(BankerAlgorithmData::<NUM_RESOURCES, NUM_PROCESSES>::new(resources, claim))),
                m_monitor_cv: BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::init_cv(resources),
            }
        }

        fn init_cv(resources: [usize; NUM_RESOURCES]) -> Vec<Arc<Condvar>> {
            let mut v: Vec<Arc<Condvar>> = Vec::new();
            
            for _ in resources.iter() {
                v.push(Arc::new(Condvar::new()));
            }
            v
        }

        pub fn allocate_resource(&self, process: usize, resource: usize, amount: usize) -> bool {

            let lock = &*self.m_monitor_mutex;
            let mut monitor = lock.lock().unwrap();

            println!("ALLOCATION REQUEST BY PROCESS {} : RESOURCE {} --> {}", process, resource, amount);

            // check parameter correctness
	        if resource >= NUM_RESOURCES || process >= NUM_PROCESSES {
		        println!("WRONG PARAMETERS IN RESOURCE REQUEST");
                // No need to unlock, data goes out of scope
		        return false;
	        }

            	// check if request is consistent with the claim
	        if monitor.m_alloc[resource][process] + amount > monitor.m_claim[resource][process] {
		        println!("WRONG RESOURCE REQUEST BY PROCESS {}", process);
                // No need to unlock, data goes out of scope
		        return false;
	        }

            let mut safe = false;

            // try to allocate until the state is safe
	        while !safe {

                // check if resource is available; if not, sleep until
    		    // resource becomes available
                let available = monitor.m_available[resource];

		        if amount > available { //monitor.m_available[resource] {
			        println!("RESOURCE NOT AVAILABLE: SUSPENDING PROCESS {}", process);
                    BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::print_state(&monitor);
			        monitor = self.m_monitor_cv[resource].wait(monitor).unwrap();
                    continue;
		        }


		        // simulate allocation
		        monitor.m_alloc[resource][process] += amount;
		        monitor.m_available[resource] -= amount;


		        // check if the state is safe
		        safe = BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::is_safe(&*monitor);
		        //safe = true;	// uncomment this line to disable resource allocation denial


		        // if state is not safe, restore the original
		        // state and suspend
		        if !safe {

			        // unsafe state detected

			        monitor.m_alloc[resource][process] -= amount;
			        monitor.m_available[resource] += amount;
			
			        // suspend is state is unsafe
			        // (will wake-up when resources will be freed)
	
			        println!("UNSAFE STATE DETECTED: SUSPENDING PROCESS {}", process);
			        BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::print_state(&monitor);
			        monitor = self.m_monitor_cv[resource].wait(monitor).unwrap();
                    
			        continue;
		        }
	        }   

        	// state is safe

	        println!("SAFE STATE DETECTED: ALLOCATION GRANTED TO PROCESS {}", process);
            BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::print_state(&monitor);

	        // pthread_mutex_unlock(&m_monitor_mutex);
            // No need to unlock, data goes out of scope 
            return true;
        }


        pub fn release_resource(&self, process: usize, resource: usize, amount: usize) -> bool {

            let lock = &*self.m_monitor_mutex;
            let mut monitor = lock.lock().unwrap();

            println!("RELEASE REQUEST BY PROCESS {} : RESOURCE {} --> {}", process, resource, amount);

	        // check parameter correctness
	        if resource >= NUM_RESOURCES || process >= NUM_PROCESSES {
	        	println!("WRONG PARAMETERS IN RELEASE REQUEST");
                // Automatic unlock
	        	return false;
	        }

	        // check if resource is actually allocated to the process
	        if monitor.m_alloc[resource][process] < amount {
	        	println!("WRONG RELEASE REQUEST BY PROCESS {}", process);
	        	// Automatic unlock
	        	return false;
	        }

            monitor.m_alloc[resource][process] -= amount;
            monitor.m_available[resource] += amount;
        
            println!("RESOURCE RELEASED BY PROCESS {}", process);
            BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::print_state(&*monitor);
        
            // wake-up suspended processes
            self.m_monitor_cv[resource].notify_all();
        
            // pthread_mutex_unlock(&m_monitor_mutex);
            // automatic unlock
        
            return true;

        }

        pub fn terminate_process(&self, process: usize) -> bool {

            let lock = &*self.m_monitor_mutex;
            let mut monitor = lock.lock().unwrap();

	        println!("DEALLOCATION OF PROCESS {}\n", process);

	        // check parameter correctness
	        if process >= NUM_PROCESSES {
	        	println!("WRONG PARAMETERS IN TERMINATION REQUEST");
	        	// Automatic unlock
	        	return false;
	        }

	        // check if process is running
	        if monitor.m_running[process] == false {
	        	println!("PROCESS ALREADY TERMINATED");
	        	// Automatic unlock
	        	return false;
	        }

	        // release all resources
	        for resource in 0..monitor.m_num_resources {

                if monitor.m_alloc[resource][process] > 0 {
                    monitor.m_available[resource] += monitor.m_alloc[resource][process];
                    monitor.m_alloc[resource][process] = 0;
        
                    // wake-up suspended processes waiting for a resource
                    self.m_monitor_cv[resource].notify_all()
                }
            }

	        // mark process as "inactive"
	        monitor.m_running[process] = false;

            BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::print_state(&*monitor);

            // Automatic unlock
	        return true;
        }
    
        fn is_safe(monitor: &BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES>) -> bool {
        
            // array for simulating resource availability
            let mut currentavail = monitor.m_available.clone();
            
            let mut rest_processes: Vec<usize> = Vec::new(); //LinkedList<usize> = LinkedList::new(); // Vec<usize> = Vec::new();
            let mut safe_sequence:  Vec<usize> = Vec::new(); //LinkedList<usize> = LinkedList::new(); // Vec<usize> = Vec::new();

	        // the safe state is checked by only considering "running" processes
	        for i in 0..monitor.m_num_processes {
		        if monitor.m_running[i] == true {
                    //rest_processes.push_back(i);
                    rest_processes.push(i);
                }
	        }

	        let mut possible = true;
            let mut curr_proc = 0;

	        while possible {

	            // find a process such that (claim - alloc <= currentavail)
	            let mut found = false;
                //let mut it = rest_processes.iter();
                
                let mut index = 0;
	            while !found && index < rest_processes.len()/* it != None*/ {

	                found = true;
	     		    curr_proc = rest_processes[index]; //it.next();

                    for i in 0..monitor.m_num_resources {
                        if monitor.m_claim[i][curr_proc] - monitor.m_alloc[i][curr_proc] > currentavail[i] {
             				found = false;
                        }
                    }

                    if !found {
                        index += 1;
                        continue;
                    }
                }

	            if found {
			        // simulate execution of process "curr_proc"
			        for i in 0..monitor.m_num_resources {
				        currentavail[i] += monitor.m_alloc[i][curr_proc];
			        }


                    // spero faccia la stessa cosa di rest_processes.erase(it);
                    rest_processes.remove(rest_processes.iter().position(|x| *x == curr_proc).unwrap()); 
	                safe_sequence.push(curr_proc); //safe_sequence.push_back(curr_proc);
                } else {
			        possible = false;
		        }
	        }
            
	        if rest_processes.len() == 0 {

	            // print safe process sequence found
	            println!("\nSAFE PROCESS SEQUENCE: ");

                for elem in safe_sequence {
                    print!("{} ", elem);
                }

                println!("\n");
            }

            return rest_processes.len() == 0;
        }
    
        fn print_state(monitor: &BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES>) {

            for i in 0..monitor.m_num_resources {
        
                if i == 0 {
                    print!("\nALLOCATED (CLAIM)\n\n");
                    print!("+-->   \t\tProcessi\n");
                    print!("|\n");
                    print!("V\n");
                    print!("Risorse\t\t");
        
                } else {
        
                    print!("       \t\t");
                }
        
        
                for j in 0..monitor.m_num_processes {
                    print!("{} ({})\t", monitor.m_alloc[i][j], monitor.m_claim[i][j]);
                }
        
                println!();
            }

            println!();
            println!();        
        
            print!("\nAVAILABLE (TOTAL)\n\n");
        
            for i in 0..monitor.m_num_resources {
                print!("{} ({})\t", monitor.m_available[i], monitor.m_resources[i]);
            }
        
            println!();
            println!();
            println!("--------------------------------------------");
            println!();
        
        }

    }
}






use std::{thread, time};

const NUM_RES: usize = 3;
const NUM_PROC: usize = 3;

const PROC1: usize = 0;
const PROC2: usize = 1;
const PROC3: usize = 2;

const RES1: usize = 0;
const RES2: usize = 1;
const RES3: usize = 2;

fn main() {

    let claim = [
        [ 70, 70, 50 ],
        [ 1, 1, 0 ],
        [ 0, 0, 1 ]
    ];
    let resources = [ 100, 1, 1 ];

    let child1 = thread::spawn(move || {
        let banker = rust_examples::BankerAlgorithm::<NUM_RES, NUM_PROC>::new(resources, claim);
    	banker.allocate_resource(PROC1, RES1, 40);
        thread::sleep(time::Duration::from_millis(5000));

	    banker.allocate_resource(PROC1, RES1, 30);
	    banker.allocate_resource(PROC1, RES2, 1);
	    thread::sleep(time::Duration::from_millis(5000));

	    // release resources
	    banker.terminate_process(PROC1);
    });


    let child2 = thread::spawn(move || {
        let banker = rust_examples::BankerAlgorithm::<NUM_RES, NUM_PROC>::new(resources, claim);
        thread::sleep(time::Duration::from_millis(1000));

    	banker.allocate_resource(PROC2, RES1, 40);
        thread::sleep(time::Duration::from_millis(5000));

	    banker.allocate_resource(PROC2, RES1, 30);
	    banker.allocate_resource(PROC2, RES2, 1);
	    thread::sleep(time::Duration::from_millis(5000));

	    // release resources
	    banker.terminate_process(PROC2);
    });

    let child3 = thread::spawn(move || {
        let banker = rust_examples::BankerAlgorithm::<NUM_RES, NUM_PROC>::new(resources, claim);
        thread::sleep(time::Duration::from_millis(2000));

    	banker.allocate_resource(PROC3, RES1, 10);
        thread::sleep(time::Duration::from_millis(4000));

	    banker.allocate_resource(PROC3, RES1, 40);
	    banker.allocate_resource(PROC3, RES3, 1);
	    thread::sleep(time::Duration::from_millis(4000));

	    // release resources
	    banker.terminate_process(PROC3);
    });

    let _res1 = child1.join();
    let _res2 = child2.join();
    let _res3 = child3.join();


    println!("Simulation correctly terminated");
}
