use threadpool::ThreadPool;
use crate::door;

pub(crate) struct ParallelExecutor {
    pool: ThreadPool,
}

impl ParallelExecutor {
    pub(crate) fn new() -> ParallelExecutor {
        return ParallelExecutor{ pool: ThreadPool::new(2), }
    }

    pub(crate) fn execute(&self, hash: String) {
        self.pool.execute(move || run(hash));
    }
}

fn run(hash: String) {
    let result = door::decode(&hash);

    if let Ok(password) = result {
        send_response(&password);
        return;
    }

    tracing::warn!("Failed to decode password for {}", hash);
}

fn send_response(password: &str) {
    let response = ureq::get(format!("http://ai.private.dev.cs2024.one/jungle/unlock?password={}", password).as_str()).call();

    if let Err(error) = response {
        tracing::warn!("Failed to send password with error: {}", error);
    }
}
