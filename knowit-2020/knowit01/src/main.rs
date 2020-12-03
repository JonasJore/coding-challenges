#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://julekalender-backend.knowit.no/challenges/1/attachments/numbers.txt")
        .await?
        .text()
        .await?;

    let mut seq: Vec<u32> = res.split(",")
        .collect::<Vec<&str>>().into_iter()
        .map(|num| num.parse().unwrap())
        .collect();

    seq.sort();

    for i in 0..seq.len() {
        if seq[i] + 1 != seq[i + 1] {
            println!("tallet som mangler er: {}", seq[i]);
            break;
        }
    }

    Ok(())
}