use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// A server on this computer answering every request on a connection with
/// "ok", counting the connections it was opened.
async fn server() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    let opened = Arc::new(AtomicUsize::new(0));
    let count = opened.clone();
    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            count.fetch_add(1, Ordering::SeqCst);
            tokio::spawn(async move {
                let mut buf = vec![0; 4096];
                while let Ok(n) = socket.read(&mut buf).await {
                    if n == 0 {
                        break;
                    }
                    let reply = "HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nok";
                    if socket.write_all(reply.as_bytes()).await.is_err() {
                        break;
                    }
                }
            });
        }
    });
    (url, opened)
}

fn get(url: &str, cancel_id: Option<&str>) -> HttpRequest {
    HttpRequest {
        url: url.to_string(),
        method: None,
        headers: BTreeMap::new(),
        body: None,
        body_base64: false,
        cancel_id: cancel_id.map(str::to_string),
    }
}

#[tokio::test]
async fn a_plugins_requests_go_over_one_kept_connection() {
    let (url, opened) = server().await;
    let granted = vec!["network:*".to_string()];
    for _ in 0..3 {
        let answer = send("test.kept", &granted, get(&url, None)).await.unwrap();
        assert_eq!(answer.status, 200);
    }
    assert_eq!(opened.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn a_cancelled_request_answers_at_once() {
    let work = cancellable("test.cancel", Some("r1".into()), async {
        tokio::time::sleep(Duration::from_secs(60)).await;
        Ok(())
    });
    let cancel = async {
        tokio::time::sleep(Duration::from_millis(20)).await;
        plugin_http_cancel("test.cancel".into(), "r1".into());
    };
    let started = std::time::Instant::now();
    let (answer, ()) = tokio::join!(work, cancel);
    assert_eq!(answer, Err("cancelled".to_string()));
    assert!(started.elapsed() < Duration::from_secs(5));
    assert!(!locked(&PENDING).contains_key(&("test.cancel".into(), "r1".into())));
}

#[tokio::test]
async fn a_second_request_under_the_same_id_leaves_the_first_running() {
    let slow = |n| async move {
        tokio::time::sleep(Duration::from_millis(80)).await;
        Ok(n)
    };
    let first = cancellable("test.twice", Some("1".into()), slow(1));
    let second = async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        cancellable("test.twice", Some("1".into()), slow(2)).await
    };
    let (a, b) = tokio::join!(first, second);
    assert_eq!((a, b), (Ok(1), Ok(2)));
}

/// A server answering every request with `size` bytes of one letter.
async fn file(size: usize) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    tokio::spawn(async move {
        let body = vec![b'x'; size];
        let head = format!("HTTP/1.1 200 OK\r\ncontent-length: {size}\r\n\r\n");
        while let Ok((mut socket, _)) = listener.accept().await {
            let (head, body) = (head.clone(), body.clone());
            tokio::spawn(async move {
                let mut buf = vec![0; 4096];
                let _ = socket.read(&mut buf).await;
                let _ = socket.write_all(head.as_bytes()).await;
                let _ = socket.write_all(&body).await;
            });
        }
    });
    url
}

/// A sync downloads the files of the repository it mirrors, and a folder of
/// notes holds a PDF of 33 MiB; a response limit of 32 MiB refused it, and the
/// plugin could only say "check your network", which is not what happened.
#[tokio::test]
async fn a_downloaded_file_of_thirty_three_megabytes_reaches_the_plugin() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let size = 33 * 1024 * 1024;
    let url = file(size).await;
    let granted = vec!["network:*".to_string()];
    let answer = send("test.file", &granted, get(&url, None)).await.unwrap();
    assert_eq!(answer.status, 200);
    assert_eq!(STANDARD.decode(&answer.body).unwrap().len(), size);
}
