use crate::{actions::handle_query, log_debug, log_info, structs::WorkerTask};
use tokio::{spawn, sync::mpsc::Receiver, task::JoinHandle};

// === ADJUST WORKERS ===

pub async fn adjust_workers(
  workers: &mut Vec<JoinHandle<()>>,
  rx: &Receiver<WorkerTask>,
  max_workers: usize,
  max_messages: usize,
  debug: bool,
) {
  if !debug {
    workers.retain(|worker| !worker.is_finished());
    return;
  }

  let count: usize = workers.len().min(max_workers);
  let queued: usize = rx.len().min(max_messages);
  let growing: bool = workers.len() < max_workers;
  let shrinking: bool = workers.len() > max_workers;

  if growing {
    log_debug!("Increasing the number of workers");
  }
  log_info!("Workers {count}/{max_workers} | Message Queue: {queued}/{max_messages}");

  workers.retain(|worker| !worker.is_finished());
  if shrinking {
    log_debug!("Reducing workers, removing inactive ones...");
  }
}

// === HANDLE WORKER TASK ===

pub async fn handle_worker_task(task: WorkerTask, debug: bool) {
  if let Err(e) = handle_query(
    &task.config,
    &task.lookup_socket,
    &task.socket,
    task.payload.to_vec(),
    task.source,
    debug,
  )
  .await
  {
    if debug {
      log_debug!("Error processing query for {}: {}", task.source, e);
    }
  }
}

// === WORKER POOL ===

pub async fn worker_pool(
  mut rx: Receiver<WorkerTask>,
  max_workers: usize,
  max_messages: usize,
  debug: bool,
) {
  let mut workers: Vec<JoinHandle<()>> = Vec::new();
  loop {
    if let Some(task) = rx.recv().await {
      let lenght: usize = workers.len();
      let slot = workers.iter_mut().find(|w| w.is_finished());
      match slot {
        Some(worker) => {
          worker.abort();
          *worker = spawn(handle_worker_task(task, debug));
        },
        None if lenght < max_workers => {
          workers.push(spawn(handle_worker_task(task, debug)));
        },
        None => {
          if debug {
            log_debug!("Maximum number of workers reached. Waiting...");
          }
        },
      }
    }

    adjust_workers(&mut workers, &rx, max_workers, max_messages, debug).await
  }
}
