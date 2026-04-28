Scratch notes for organizing thoughts...

There are 5 different types of actions from input CSV

\*\*\* these types probably have tx id's associated with them \*\*\*

Deposit("deposit") - credit to client account, increase AVAILABLE && TOTAL funds

Withdrawl("withdrawl") - decrease AVAILABLE && TOTAL funds

- If the withdrawl exceeds current available funds then fail this transaction and leave account unchanged

\*\*\* end new tx id types \*\*\*

\*\*\* we ignore all the cases below where tx refers to a specific transaction id (probably deposits only) \*\*\*

Dispute("dispute") - amountless transaction, referring to a disputed action on the account, place available funds on HOLD for this tx amount
leave total funds the same

- if tx does not exist, ignore this dispute

Resolve("resolve") - customer dispute was resolved

- increase available funds
- decrease held funds
- total funds remains same
- mark dispute tx as resolved

Chargeback("chargeback") - a chargeback finalizes a reverse transaction (dispute) - lower total and available funds by the dispute amount

- we freeze the client account

- descrease total and available funds

- decrease held funds

- assuming freeze means we don't allow any more actions except withdrawls on this account?

So it looks like we needs

Transaction - needs to include

- a unique id
- a type (deposit, resolve, dispute, etc...)
- an ammount
- probably will scope this to just withdrawl and deposits...

Disputes

- can add dispute status
  - Dispute, Resolve, Chargeback \<- some kind of mutable state? expensive to update, but saves on memory
- associated tx id

Account - would be the client's account

- uuid - the unique id of the account, assumed from the csv input
- total_funds - a total amount of funds, this would probably be held + available
- held funds - funds that are tied up in current disputes
- available funds - funds that are available for the account to use, probably just calculated based on (total - held)
- frozen - a boolean that indicates whether or not this account is frozen

Can parse the line items like ...

```rust
InputLineItem {
  r#type: TypeEnum,
  /// customer id
  cx: u16,
  tx: u32,
  /// Decimal value with 4 precision points. (Will just use f32 for now, this would cap out at around 1.6 million though assuming most tx won't be that large)
  /// Could be adjusted to f64 for better accuracy at higher numbers
  amount: f32,
}
```

parse inputs for each line like `let item = line.parse::<InputLineItem>()` then send it to the account processor

at the end of the system... we could output using a similar writer stream...

```rust
OutputLineItem {
  /// Can alias account id
  client: AccountId,
  available: f32,
  held: f32,
  total: f32,
  locked: bool,
}
```

Outer system struct that initializes the system
core features to implement...

- csv parser, can just use a csv stream to read the lines and process them individually.
