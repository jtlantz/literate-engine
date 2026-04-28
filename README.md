# literate-engine

This is a simple transaction processor that handles flagging clients for disputes and such

There are a few core components...


`core::System` is the struct in which the csv record parser performs actions on the transactions
each line in the csv will be serialized to a `InputLineItem` which contains a `LineItemType` enum
to help identify what type of transaction it is. The rest of the information is populated from
the csv file.

Then the transaction object is passed back to the `System` in to which is will match either an existing
`Account` or create a new `Account` object. We then process `Transactions` against the `Account` object.

Since this is all serial, we don't do any mutex locking or asynchrnous actions, althought this could be adopted
with async actions by modifying the parser a bit and wrapping the accounts ledger in a DashMap instead
of a HashMap.

When the transactions are done being processed, `System::export_data()` fn should be called in main
which will serialize the output to a byte stream and dump to std_out.
