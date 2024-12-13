use candid::Nat;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[ic_cdk::query]
fn calculate_currency_price(curr_a: Nat, curr_b: Nat) -> Nat {
    curr_a / curr_b
}
