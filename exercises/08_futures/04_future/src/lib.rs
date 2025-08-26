//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    yield_now().await;
    let non_send = Rc::new(1);
    println!("{}", non_send);
}

// An async fn is compiled into a state machine. Every .await is like a suspension point.
// Anything that lives across an .await must be stored inside the future’s state 
// and therefore must satisfy the same requirements as the future itself (Send, 'static, etc.).
