use std::sync::Arc;
use threadpool::ThreadPool;
use crate::door;

pub(crate) struct ParallelExecutor {
    pool: ThreadPool,
    dictionary: Arc<Vec<String>>,
}

impl ParallelExecutor {
    pub(crate) fn new() -> Option<ParallelExecutor> {
        let dictionary = Arc::new(get_dictionary()?);
        return Some(ParallelExecutor{ pool: ThreadPool::new(2), dictionary })
    }

    pub(crate) fn execute(&self, hash: String) {
        let dictionary = self.dictionary.clone();
        self.pool.execute(move || run(hash, dictionary));
    }
}

fn run(hash: String, dictionary: Arc<Vec<String>>) {
    let result = door::decode(&hash, dictionary.as_ref());

    match result {
        Ok(password) => {
            send_response(&password);
            return;
        }
        Err(err) => {tracing::warn!("Failed to decode password for {} with err {}", hash, err)}
    }
}

fn send_response(password: &str) {
    let response = ureq::get(format!("http://ai.private.dev.cs2024.one/jungle/unlock?password={}", password).as_str()).call();

    if let Err(error) = response {
        tracing::warn!("Failed to send password with error: {}", error);
    }
}

fn get_dictionary() -> Option<Vec<String>> {
    const WORD_LIST_URL: &str = "https://raw.githubusercontent.com/DavidWittman/wpxmlrpcbrute/master/wordlists/1000-most-common-passwords.txt";
    let response = ureq::get(WORD_LIST_URL).call().ok()?;
    let words = response.into_string().ok()?.split("\n").map(|word| word.to_string()).collect::<Vec<_>>();
    return Some(words);
}
