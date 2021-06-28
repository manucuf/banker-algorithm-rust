#[allow(unused_imports)]
//#[allow(unused_variables)]
#[allow(dead_code)]

mod rust_examples {

    use std::thread;
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
            let monitor = lock.lock().unwrap();

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

            let safe = false;

            // try to allocate until the state is safe
	        while !safe {

                // check if resource is available; if not, sleep until
    		    // resource becomes available
                let arc_for_iteration = Arc::clone(&self.m_monitor_mutex);
                let mut safe_monitor = arc_for_iteration.lock().unwrap();
                let available = safe_monitor.m_available[resource];

		        if amount > available { //monitor.m_available[resource] {
			        println!("RESOURCE NOT AVAILABLE: SUSPENDING PROCESS {}", process);
			        //printState();
			        let _result = self.m_monitor_cv[resource].wait(safe_monitor);
                    continue;
		        }


		        // simulate allocation
		        safe_monitor.m_alloc[resource][process] += amount;
		        safe_monitor.m_available[resource] -= amount;


		        // check if the state is safe
		        //safe = isSafe();
		        //safe = true;	// uncomment this line to disable resource allocation denial


		        // if state is not safe, restore the original
		        // state and suspend
		        if !safe {

			        // unsafe state detected

			        safe_monitor.m_alloc[resource][process] -= amount;
			        safe_monitor.m_available[resource] += amount;
			
			        // suspend is state is unsafe
			        // (will wake-up when resources will be freed)
	
			        println!("UNSAFE STATE DETECTED: SUSPENDING PROCESS {}", process);
			        //printState();
			        let _result = self.m_monitor_cv[resource].wait(safe_monitor);
                    
			        continue;
		        }
	        }   

        	// state is safe

	        println!("SAFE STATE DETECTED: ALLOCATION GRANTED TO PROCESS {}", process);
	        //printState();

	        // pthread_mutex_unlock(&m_monitor_mutex);
            // No need to unlock, data goes out of scope 
            return true;
        }


        fn release_resource(&self, process: usize, resource: usize, amount: usize) -> bool {

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
            //printState();
        
            // wake-up suspended processes
            self.m_monitor_cv[resource].notify_all();
        
            // pthread_mutex_unlock(&m_monitor_mutex);
            // automatic unlock
        
            return true;

        }

        fn terminate_process(&self, process: usize) -> bool {

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

	        //printState();

            // Automatic unlock
	        return true;
        }
    
        fn is_safe() -> bool {
            return true;
        }
    
        fn print_state() {}
    }


}

const NUM_RES: usize = 3;
const NUM_PROC: usize = 3;

fn main() {

    let claim = [
        [ 70, 70, 50 ],
        [ 1, 1, 0 ],
        [ 0, 0, 1 ]
    ];
    let resources = [ 100, 1, 1 ];

    let _banker_algorithm = rust_examples::BankerAlgorithm::<NUM_RES, NUM_PROC>::new(resources, claim);

    println!("It works!");
}
