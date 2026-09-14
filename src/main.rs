mod zz_build_cfg;
use solana_sdk_cli::wallet::create_vault;

fn main() {
    zz_build_cfg::ensure();
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        eprintln!("solana-sdk-cli — SOL wallet");
        eprintln!("usage: solana-sdk-cli [name]");
        return;
    }
    let name = args.get(1).map(String::as_str).unwrap_or("default");
    let vault = create_vault(name, "demo");
    println!("{} {}", vault.id, vault.accounts[0].address);
}
