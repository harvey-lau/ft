use libafl::bolts::rands::StdRand;
use libafl::bolts::shmem::{ShMem, ShMemProvider, StdShMemProvider};
use libafl::bolts::tuples::tuple_list;
use libafl::bolts::{current_nanos, AsMutSlice};
use libafl::corpus::{Corpus, InMemoryCorpus, OnDiskCorpus};
use libafl::events::SimpleEventManager;
use libafl::executors::{ForkserverExecutor, TimeoutForkserverExecutor};
use libafl::feedbacks::{MaxMapFeedback, TimeFeedback, TimeoutFeedback};
use libafl::inputs::BytesInput;
use libafl::monitors::SimpleMonitor;
use libafl::mutators::{havoc_mutations, StdScheduledMutator};
use libafl::observers::{HitcountsMapObserver, StdMapObserver, TimeObserver};
use libafl::schedulers::{IndexesLenTimeMinimizerScheduler, QueueScheduler};
use libafl::stages::StdMutationalStage;
use libafl::state::{HasCorpus, StdState};
use libafl::{feedback_and_fast, feedback_or, Error, Fuzzer, StdFuzzer};
use std::path::PathBuf;
use std::time::Duration;
use rand::Rng;

const MAP_SIZE: usize = 65536;

fn main() -> Result<(), Error> {
    
    // Corpus
    let corpus_dirs = vec![PathBuf::from("/home/anti/github/ft/libafl_xpdf/corpus")];
    let input_corpus = InMemoryCorpus::<BytesInput>::new();
    let timeouts_corpus = OnDiskCorpus::new(PathBuf::from("/home/anti/github/ft/libafl_xpdf/timeouts"))?;

    // Observer
    let time_observer = TimeObserver::new("time");
    let mut shmem_provider = StdShMemProvider::new()?;
    let mut shmem = shmem_provider.new_shmem(MAP_SIZE)?;
    shmem.write_to_env("__AFL_SHM_ID")?;
    let shmem_buf = shmem.as_mut_slice();

    let edges_observer = 
        unsafe {
            HitcountsMapObserver::new(StdMapObserver::new("shard_mem", shmem_buf))
        };

    // Feedback
    let mut feedback = feedback_or!(
        MaxMapFeedback::tracking(&edges_observer, true, false),
        TimeFeedback::with_observer(&time_observer)
    );
    let mut objective = 
        feedback_and_fast!(TimeoutFeedback::new(), MaxMapFeedback::new(&edges_observer));

    // Monitor
    let monitor = SimpleMonitor::new(|_s| println!("{}", rand::thread_rng().gen_range(1..=100)));

    // EventManager
    let mut mgr = SimpleEventManager::new(monitor);

    // State
    let mut state = StdState::new(
        StdRand::with_seed(current_nanos()),
        input_corpus,
        timeouts_corpus,
        &mut feedback,
        &mut objective,
    )?;

    // Scheduler
    let scheduler = IndexesLenTimeMinimizerScheduler::new(QueueScheduler::new());

    // Fuzzer
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Executor
    let fork_server = ForkserverExecutor::builder()
        .program("/home/anti/github/ft/libafl_xpdf/xpdf/install/bin/pdftotext")
        .parse_afl_cmdline(["@@"])
        .coverage_map_size(MAP_SIZE)
        .build(tuple_list!(time_observer, edges_observer))?;
    let timeout = Duration::from_secs(5);
    let mut executor = TimeoutForkserverExecutor::new(fork_server, timeout)?;

    // Corpus
    if state.corpus().count() < 1 {
        state
            .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, &corpus_dirs)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to load initial corpus at {:?}: {:?}.",
                    &corpus_dirs, err
                )
            });
        println!("Imported {} inputs from disk.", state.corpus().count());
    }

    // Mutator
    let mutator = StdScheduledMutator::new(havoc_mutations());

    // Stage
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)?;

    Ok(())
}
