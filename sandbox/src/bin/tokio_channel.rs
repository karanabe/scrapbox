use rand::Rng;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use tokio::task::id;

enum Command {
    Increment,
}

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, u64, oneshot::Sender<String>)>(40);

    let t = tokio::spawn(async move {
        while let Some((cmd, threadid, response)) = cmd_rx.recv().await {
            tokio::spawn(async move {
                let random = {
                    let mut rng = rand::thread_rng();
                    let result: u64 = rng.gen_range(1000..5000);
                    result
                };
                tokio::time::sleep(tokio::time::Duration::from_millis(random)).await;
                match cmd {
                    Command::Increment => response.send(threadid.to_string()).unwrap(),
                }
            });
        }
    });

    let cmd_tx_sender = cmd_tx.clone();
    let t1 = tokio::spawn(async move {
        send_hello(cmd_tx_sender).await;
    });
    let cmd_tx_sender = cmd_tx.clone();
    let t2 = tokio::spawn(async move {
        send_hello(cmd_tx_sender).await;
    });
    let cmd_tx_sender = cmd_tx.clone();
    let t3 = tokio::spawn(async move {
        send_hello(cmd_tx_sender).await;
    });

    let (_, _, _, _) = (
        t.await.unwrap(),
        t1.await.unwrap(),
        t2.await.unwrap(),
        t3.await.unwrap(),
    );
}

async fn send_hello(cmd_tx: Sender<(Command, u64, oneshot::Sender<String>)>) {
    let random = {
        let mut rng = rand::thread_rng();
        let result: u64 = rng.gen_range(1000..5000);
        result
    };
    tokio::time::sleep(tokio::time::Duration::from_millis(random)).await;
    let (resp_tx, resp_rx) = oneshot::channel::<String>();
    cmd_tx
        .send((Command::Increment, random, resp_tx))
        .await
        .ok()
        .unwrap();
    let res = resp_rx.await.unwrap();

    println!(
        "{} - {} / task_id: {} - thread_id: {}",
        random,
        res,
        id(),
        thread_id::get()
    );
}

/*
#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel::<String>(40);
    let tx_count = tx.clone();
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();
    let mut rx4 = tx.subscribe();
    let mut rx5 = tx.subscribe();


    let t1 = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(7000)).await;
        let _x = tx.send("TEST".to_string());
        "good"
    });


    // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
    if let Err(_) = timeout(Duration::from_millis(10), rx1.recv()).await {
        println!("did not receive value within 10 ms");
    }
    println!("{:?}", tx_count.receiver_count());


    let t2 = tokio::spawn(async move {
        recv_hello1(&mut rx2).await;
        "good"
    });

    let t3 = tokio::spawn(async move {
        recv_hello2(&mut rx3).await;
        "good"
    });


    let result = t1.await.unwrap();
    println!("{:?}", result);
    println!("come here?");
}


async fn recv_hello1(rx: &mut Receiver<String>){
    if let Err(_) = timeout(Duration::from_millis(10), rx.recv()).await {
        println!("did not receive value within 10 ms");
    }
    match rx.recv().await {
        Ok(v) => println!("hello1 got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}

async fn recv_hello2(rx: &mut Receiver<String>){
    if let Err(_) = timeout(Duration::from_millis(10), rx.recv()).await {
        println!("did not receive value within 10 ms");
    }
    match rx.recv().await {
        Ok(v) => println!("hello2 got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
*/
