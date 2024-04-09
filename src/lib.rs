use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{anyhow, Result};

static IS_INITED: AtomicBool = AtomicBool::new(false);

/// Automatically collects system topology and optimizes key operations on the fly.
///
/// ## Panics
///
/// Panies when failing to init SAS.
/// Note that reinitialization is not supported.
///
#[inline]
pub fn init() {
    try_init().unwrap()
}

/// Automatically collects system topology and optimizes key operations on the fly.
///
#[inline]
pub fn try_init() -> Result<()> {
    Sas::default()
        .init()
        .map_err(|error| anyhow!("failed to init SAS: {error}"))
}

/// SAS optimization arguments.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "clap", derive(::clap::Parser))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Sas {
    /// Runtime system type.
    #[cfg_attr(
        feature = "clap",
        arg(
            default_value = "SystemType::default()",
            env = "SAS_SYSTEM_TYPE",
            long = "sas-system-type",
            value_name = "TYPE"
        )
    )]
    pub system_type: SystemType,
}

impl Sas {
    /// Optimizes key operations with given arguments.
    ///
    pub fn init(self) -> Result<()> {
        if !IS_INITED.swap(true, Ordering::SeqCst) {
            self.init_unchecked()
        } else {
            Ok(())
        }
    }

    fn init_unchecked(self) -> Result<()> {
        #[cfg(feature = "rayon")]
        {
            use rayon::ThreadPoolBuilder;

            let (has_multiple_numa_nodes, threads) = prepare_threads()?;

            let mut builder = ThreadPoolBuilder::new().num_threads(threads.len());
            if matches!(self.system_type, SystemType::Python) {
                builder = builder.use_current_thread();
            }
            builder.build_global()?;

            if has_multiple_numa_nodes {
                bind_threads(threads)?;
            }
        }

        Ok(())
    }
}

/// Runtime system type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(::clap::Parser))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(::strum::Display, ::strum::EnumString))]
pub enum SystemType {
    /// Use all threads without the main thread
    #[default]
    Generic,
    /// Use all threads even with the main thread
    Python,
}

#[cfg(all(feature = "numa", feature = "rayon"))]
#[inline]
fn get_topology() -> Result<::hwlocality::Topology> {
    ::hwlocality::Topology::new().map_err(Into::into)
}

#[cfg(all(not(feature = "numa"), feature = "rayon"))]
fn prepare_threads() -> Result<(bool, Vec<usize>)> {
    use std::thread;

    let num_threads = thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1);
    Ok((false, (0..num_threads).collect()))
}

#[cfg(all(feature = "numa", feature = "rayon"))]
fn prepare_threads() -> Result<(bool, Vec<usize>)> {
    use rand::{
        distributions::{Distribution, Uniform},
        thread_rng,
    };

    // get NUMA/CPUs info
    let topology = get_topology()?;
    let all_numa_nodes = topology.nodeset();
    let all_cpus = topology.cpuset();

    // count the resources
    let num_numa_nodes = all_numa_nodes
        .last_set()
        .map(|set| set.into())
        .unwrap_or(0usize)
        + 1;
    let num_cpus = all_cpus.last_set().map(|set| set.into()).unwrap_or(0usize) + 1;
    let num_threads_per_cpu = num_cpus / num_numa_nodes;

    // pick a random NUMA node
    let numa_node = Uniform::new(0usize, num_numa_nodes).sample(&mut thread_rng());

    // get all the CPUs in the NUMA node
    let cpu_begin = numa_node * num_threads_per_cpu;
    let cpu_end = cpu_begin + num_threads_per_cpu;
    let cpus = (cpu_begin..cpu_end).collect();
    Ok((num_numa_nodes > 1, cpus))
}

#[cfg(all(not(feature = "numa"), feature = "rayon"))]
#[inline]
fn bind_threads(_: Vec<usize>) -> Result<()> {
    Ok(())
}

#[cfg(all(feature = "numa", feature = "rayon"))]
fn bind_threads(threads: Vec<usize>) -> Result<()> {
    use hwlocality::cpu::{binding::CpuBindingFlags, cpuset::CpuSet};

    ::rayon::scope(|s| {
        s.spawn_broadcast({
            move |_, ctx| {
                // bind the given thread into the NUMA node
                let topology = get_topology().expect("failed to load topology");
                let cpus = {
                    let mut res = CpuSet::new();
                    res.set(threads[ctx.index()]);
                    res
                };
                topology
                    .bind_cpu(&cpus, CpuBindingFlags::THREAD)
                    .expect("failed to bind the rayon thread into CPU");
            }
        });
    });
    Ok(())
}
