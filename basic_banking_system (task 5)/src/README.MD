# 🔥Task: Adding error handling to the basic banking system

In this task, students will add error handling to the basic banking system created in the previous task. The program will handle errors related to depositing and withdrawing money from the bank account.

**Steps**

1. Modify the deposit method of the Account Trait to return a Result<(), String> instead of (). The Result type will be used to handle errors. The String type willbe used to hold the error message.
2. Modify the withdraw method of the Account Trait to return a Result<(), String> instead of (). The Result type will be used to handle errors. The String type willbe used to hold the error message.
3. In the implementation of the deposit method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant toindicate failure.
4. In the implementation of the withdraw method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant toindicate failure.
5. In the main function, create two BankAccount instances with different account numbers and holder names.
6. Call the deposit method on one of the accounts, passing in a deposit amount. Handle any errors returned by the deposit method using a match statement.
7. Call the withdraw method on the other account, passing in a withdraw amount. Handle any errors returned by the withdraw method using a match statement.
8. Call the balance method on both accounts and print the result to the console.
9. Compile and run the program to ensure it works as expected.

 <br/>

**Checklist**

10. Modify the deposit method of the Account Trait to return a Result<(), String> instead of ().
11. Modify the withdraw method of the Account Trait to return a Result<(), String> instead of ().
12. In the implementation of the deposit method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant to indicate failure.
13. In the implementation of the withdraw method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant to indicate failure.
14. Create two BankAccount instances in the main function with different account numbers and holder names.
15. Call the deposit method on one of the accounts, passing in a deposit amount. Handle any errors returned by the deposit method using a match statement.
16. Call the withdraw method on the other account, passing in a withdraw amount. Handle any errors returned by the withdraw method using a match statement.
17. Call the balance method on both accounts and print the result to the console.
18. Compile and run the program to test its functionality.
