#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]

mod rust_examples {

    use std::thread;
    use std::sync::{Arc, Mutex, Condvar};

    pub struct BankerAlgorithm<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> {
        data: Arc<Mutex<BankerAlgorithmData<NUM_RESOURCES, NUM_PROCESSES>>>
    }

    struct BankerAlgorithmData<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> {
        m_num_resources: usize,
        m_num_processes: usize,
        m_resources: [usize; NUM_RESOURCES],
        m_claim: [[usize; NUM_RESOURCES]; NUM_PROCESSES],
        m_available: [usize; NUM_RESOURCES],
        m_alloc: [[usize; NUM_RESOURCES]; NUM_PROCESSES],
        m_running: [bool; NUM_PROCESSES],
        m_monitor_cv: Vec<Arc<Condvar>>,
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
                //m_monitor_mutex: BankerAlgorithm::<NUM_RESOURCES, NUM_PROCESSES>::init_mutex(resources),
                //m_monitor_mutex: Arc::new(Mutex::new(resources)),
                m_monitor_cv: BankerAlgorithmData::<NUM_RESOURCES, NUM_PROCESSES>::init_cv(resources),
            }
        }

        fn init_cv(resources: [usize; NUM_RESOURCES]) -> Vec<Arc<Condvar>> {
            let mut v: Vec<Arc<Condvar>> = Vec::new();
            
            for resource in resources.iter() {
                v.push(Arc::new(Condvar::new()));
            }
            v
        }
    }

    impl<const NUM_RESOURCES: usize, const NUM_PROCESSES: usize> BankerAlgorithm<NUM_RESOURCES, NUM_PROCESSES> {


        pub fn new(resources: [usize; NUM_RESOURCES], claim: [[usize; NUM_RESOURCES]; NUM_PROCESSES]) -> BankerAlgorithm<NUM_RESOURCES, NUM_PROCESSES> {
            BankerAlgorithm {
                data: Arc::new(Mutex::new(BankerAlgorithmData::<NUM_RESOURCES, NUM_PROCESSES>::new(resources, claim))),
            }
        }

        fn allocate_resource(&self, process: usize, resource: usize, amount: usize) -> bool {

            let lock = &*self.data;
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

            return true;
        }
        // fn releaseResource(process: i32, resource: i32, amount: i32) -> bool;
        // fn terminateProcess(process: i32) -> bool;
    
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
