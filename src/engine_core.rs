use crate::thread_pool::ThreadPool;
use std::sync::Arc;

/// These are the public methods that an engine object needs
pub trait EngineObject {
    /// This method is called once before the main loop starts
    fn start_up(&mut self, core: &Core);
    /// This method is called once per loop before the draw method is called
    fn update(&mut self, core: &Core);
    /// This method is called after all objects have been updated
    fn draw(&self, core: &Core);
    /// This method is called when save is requested or automatically during a graceful shutdown
    fn save(&self, core: &Core);
    /// This method is called when load is requested
    fn load(&mut self, core: &Core);
}

/// Contains the engine objects as well as the core to run them
pub struct Engine {
    items: Vec<Box<dyn EngineObject>>,
    core:  Arc<Core>,    
}

impl Engine {
    /// Returns a engine  
    ///
    /// # Arguments  
    ///
    /// * `threads` - The number of threads to use  
    ///
    /// # Example
    ///
    /// ```
    /// use nerdee_engine::engine_core::*;
    /// use nerdee_engine::terrain::*;
    /// let mut engine = Engine::new(8);
    /// engine.add(Box::new(Terrain::new()));
    /// ```
    pub fn new(threads: usize) -> Engine {
        let items = Vec::new();
        let core = Arc::new(Core::new(threads));
        Engine {
            items,
            core,
        }
    }

    pub fn initialize(&mut self) {
        let core = self.core.clone();
        self.start_up(&core);
    }

    /// Adds a EngineObject to the Engine
    pub fn add(&mut self, item: Box<dyn EngineObject>) {
        self.items.push(item);
    }

    /// This method contains the engine loop and will only return once the engine is gracefully shutdown
    pub fn run_forever(&mut self) {
        let core = self.core.clone();
        self.start_up(&core);
        loop {
            self.update(&core);
            self.draw(&core);
            println!("Frame: {}", core.wait());
        }
    }
}

/// Auto-saves before shutting down
impl Drop for Engine {
    fn drop(&mut self) {
        let core = self.core.clone();
        self.save(&core);
    }
}

/// Calls the EngineObject methods on all other EngineObjects
impl EngineObject for Engine {
    fn start_up(&mut self, core: &Core) {
        self.items.iter_mut().for_each(|item| item.start_up(core));
    }

    fn update(&mut self, core: &Core) {
        self.items.iter_mut().for_each(|item| item.update(core));
    }

    fn draw(&self, core: &Core) {
        self.items.iter().for_each(|item| item.draw(core));
    }

    fn save(&self, core: &Core) {
        self.items.iter().for_each(|item| item.save(core));
    }

    fn load(&mut self, core: &Core) {
        self.items.iter_mut().for_each(|item| item.load(core));
    }
}

/// Contains all of the components needed for running each engine object
pub struct Core {
    pool: ThreadPool
}

impl Core {
    /// Returns the Core
    /// # Arguments
    /// 
    /// * `threads` - The number of threads to create for the ThreadPool
    pub fn new(threads: usize) -> Core {
        let pool = ThreadPool::new(threads);
        Core { pool }
    }

    /// Enqueue a job to the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        self.pool.execute(f);
    }

    /// Waits until all jobs enqueued since last call to wait are finished  
    /// See ThreadPool for more info
    pub fn wait(&self) -> usize {
        return self.pool.wait();
    }
}