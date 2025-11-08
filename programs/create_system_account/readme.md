# Creating system Account

This account is going to be a system account - meaning it will be owned by the System Program. In short, this means only the System Program will be allowed to modify it's data.

In the test, we use two methods for creating the accounts. One of the methods uses Cross program invocation and the other calls the System Program directly.

Cross program invocation means that we send the transaction to create the account first to our deployed Solana Program, which then calls the System Program.

Calling the System Program directly means that the client sends the transaction to create the account directly to the Solana Program

In this example, this account will simply hold some SOL.
