use ethers::prelude::{LocalWallet, Signer, TransactionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init wallet
    let wallet = "YOUR_PRIVATE_KEY".parse::<LocalWallet>()?;

    // create tx
    let tx = TransactionRequest::new()
        .to("RECEIPT_ADDRESS")
        .value(10000)
        .into();

    // sign tx
    let signed_tx = wallet.sign_transaction(&tx).await?;
    println!("{}", signed_tx);
    
    // sign message
    let signature = wallet.sign_message("hello,world!").await?;
    signature.verify("hello,world!", wallet.address()).unwrap();

    Ok(())
}
