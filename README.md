Coding Exercise
================================================

In this task, you’ll work with Rust code that simulates sending messages through an instruction processor.

**Your goal is to fix a bug in the `message-program` code.**

Overview
--------
There are 2 main crates in this repo:
- `message-program`: Contains the core logic, including the `Message` struct and instruction processing functions.
- `solution`: The entry point where you’ll craft a dummy message and send it as an instruction.

Objective
---------
Your tasks are:
1. **Craft a Dummy Message**: Create an instance of the `Message` struct with dummy data.
2. **Send the Instruction**: Prepend an instruction code and send the message via `send_instruction`.
3. **Fix a Bug**: There is a bug in `lib.rs` that prevents correct execution. Identify and fix the bug without adding to the `Message` struct or changing the signature of `send_instruction`.

Constraints
-----------
- You cannot add fields to the `Message` struct.
- You cannot change the type signature of `send_instruction`.
- You must start by creating a `Message` and send it via `send_instruction`.
- You cannot change the visibility of any functions.
- Try to avoid using AI tools as much as possible.

Expected Output
----------------
Depending on the instruction you choose, running your code should produce one of the following outputs:

- **Initialize**:
```
Allocatin [size] bytes for [sender_id]...
```
- **Close**:
```
[size] deleted.
```
- **Update**:
```
collected [priority_fee] lamports from [sender_id]
```
  
_Note: Replace placeholders with actual values from your dummy message._

Submission
----------
Please submit you code as either a github repo or a zip file and make sure to include the following:
- Modified `lib.rs` With the bug fixed.
- `main.rs` ontaining your code to craft and send the message.
- Explanation (optional): Briefly describe the bug and how you fixed it.

Hints
-----
- You are allowed to use the internet to look up docs or examples, we just ask that you don't use AI.
- The bugfix should be relatively small and doesn't require bringing in any new dependencies (though you can if you have a good reason).
- The resulting code doesn't have to be exactly functionally the same as the original (within reason), but it should produce the same output. For example it is ok if you slightly change the error handling or safety of the code.
- If you find yourself taking much more than an hour, please reach out. It's possible you've run into an issue we didn't anticipate.


Good luck!