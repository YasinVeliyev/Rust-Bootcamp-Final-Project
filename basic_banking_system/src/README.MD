# Task: Creating a basic banking system using Traits

In this task, students will create a basic banking system using Traits in Rust. The program will allow users to create accounts, deposit and withdraw money, and view their account balance.

**Steps**

1.  Create a Trait called Account that defines the methods deposit, withdraw, and balance. These methods should take a mutable reference to self as an argument.
2.  Implement the Account Trait for a struct called BankAccount. The BankAccount struct should have the fields account_number, holder_name, and balance.
3.  In the implementation of the deposit method for BankAccount, add the deposit amount to the balance.
4.  In the implementation of the withdraw method for BankAccount, subtract the withdraw amount from the balance.
5.  In the implementation of the balance method for BankAccount, return the current balance.
6.  In the main function, create two BankAccount instances with different account numbers and holder names.
7.  Call the deposit method on one of the accounts, passing in a deposit amount.
8.  Call the withdraw method on the other account, passing in a withdraw amount.
9.  Call the balance method on both accounts and print the result to the console.
10. Compile and run the program to ensure it works as expected.

**Checklist**

*   Create a Trait called Account with the methods deposit, withdraw, and balance.
*   Implement the Account Trait for a struct called BankAccount with the fields account_number, holder_name, and balance.
*   In the implementation of the deposit method for BankAccount, add the deposit amount to the balance.
*   In the implementation of the withdraw method for BankAccount, subtract the withdraw amount from the balance.
*   In the implementation of the balance method for BankAccount, return the current balance.
*   Create two BankAccount instances in the main function with different account numbers and holder names.
*   Call the deposit method on one of the accounts, passing in a deposit amount.
*   Call the withdraw method on the other account, passing in a withdraw amount.
*   Call the balance method on both accounts and print the result to the console.
*   Compile and run the program to test its functionality.
