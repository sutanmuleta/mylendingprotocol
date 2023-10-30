# mylendingprotocol
Built on Astar Networks using Ink! 
Lending Protocol Documentation
Overview
 The protocol facilitates the lending and borrowing of tokens within a decentralized network, allowing users to interact directly with the contract to manage their deposits and loans.

Key Features
1. Token Management
Supported Tokens: The protocol initializes with a predefined set of supported tokens. However, new tokens can be added dynamically.
Deposits: Users can deposit supported tokens into the protocol.
Withdrawals: Users can withdraw their deposits, given they have sufficient balance.
2. Lending and Borrowing
Borrowing: Users can borrow tokens up to the amount they have deposited.
Repayments: Borrowers can repay their loans, reducing their outstanding debt.
3. Interest Calculation
Interest on Deposits: Depositors earn interest over time, calculated based on the duration and the amount of tokens deposited. The interest rate is set at 5% per 5 days.
Workflow
Token Addition:

New tokens can be added to the protocol dynamically through the add_token function.
Deposit Tokens:

Users deposit tokens into the protocol by invoking the deposit function, specifying the account, token symbol, and deposit amount.
Borrow Tokens:

Users can borrow tokens through the borrow function. The borrowed amount cannot exceed the user's total deposit for the specified token.
Repay Borrowed Tokens:

Borrowers can repay their loans through the repay function, specifying the account, token symbol, and repayment amount.
Interest Calculation:

Interest earned on deposits is calculated through the calculate_interest function, periodically updating the user’s balance based on the pre-defined interest rate.
