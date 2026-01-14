# Nitrogen
**A context-aware, backtrace-friendly error handling extension built on top of `std::result::Result`**

## Motivation

Errors rarely happen in isolation.  
A single low-level failure (`FileNotFound`, `ParseError`, `InvalidToken`, …) usually travels through many layers of your application before it reaches the place where it's actually handled or logged.  
By the time it arrives, critical context is often lost:

- Which user/transaction/request caused the failure?
- Through which high-level operation did it pass?
- What was the business intent?

`nitrogen` allows you to:

- Propagate errors while preserving the full history of all errors that led to the final result
- Attach custom context to add important details (user_id, request_id, transaction_id, session_id, …) for better searchable logs

<br>

**This library is made for my own use - I don't recommend anyone actually use it.**
