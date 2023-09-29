# Rust Email Blaster

A console-based email sender written in Rust, which reads contacts from a `CSV` file, uses a `HTML` template for the email body, and utilizes `SMTP` for sending emails. It logs the status of each sent email to another `CSV` file.

## Prerequisites

1. **Rust**: This project is written in Rust. You need the Rust toolchain to compile it. You can install it from [here](https://rust-lang.org/).
2. **Cargo**: This is Rust’s build system and package manager, typically installed with Rust itself.

## Build Instructions

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/rust-email-sender.git
   cd rust-email-sender
   ```
   
2. Install Dependencies:

The dependencies are listed in Cargo.toml and will be automatically installed when you build the project.

3. Compile the Code:

   ```bash
   cargo build --release
   ```

>The compiled binary will be located under target/release/.

## How to Run

To run the program:

   ```bash
   ./target/release/rust_email_sender /path/to/target/folder
   ```

>Replace /path/to/target/folder with the actual path to your target folder, which should contain the required contacts.csv, email.html, and config.json files.

## Config Setup (config.json)
The config.json file in your target folder should look something like this:

   ### Sample:
   ```json
   {
      "smtp_host": "smtp.example.com",
      "smtp_port": 587,
      "username": "your@email.com",
      "password": "yourpassword",
      "delay": 1000,
      "from": "noreply@email.com"
   }
   ```
>The delay is the delay in milliseconds between each email sent. This is to prevent rate limiting by your SMTP server. The value can be 0 to 4294967295 (max value of u32).

>**Security Note:** Be sure to secure your config.json file properly as it contains sensitive data.

## contacts.csv Format
The contacts.csv file should contain the following columns (Column 1, 2 and 3):

* email
* first_name
* last_name

### Sample:

   ```csv
   email,first_name,last_name
   example1@example.com,John,Doe
   example2@example.com,Jane,Doe
   ```

## email.html Template
Create an email.html file in your target folder. You can use [email], [first_name], [last_name], and [full_name] as placeholders that will be replaced by the actual values.

Sample:

   ```html
   <h1>Hello [first_name] [last_name],</h1>
   <p>This is a sample email to [email].</p>
   <p>Regards,</p>
   <p>Your Company</p>
   ```

## Expected Output
The program will generate a log.csv file in the target folder with the following columns:

* email
* first_name
* last_name
* result

>status can either be success or failed.

## Troubleshooting
1. SMTP Connection Errors: Double-check your config.json settings, specifically the host, port, username, and password.
2. File Not Found: Ensure contacts.csv, email.html, and config.json are in the correct target folder.
3. Compilation Errors: Ensure that you have the latest version of Rust and Cargo installed.
4. Rate Limiting: If your emails are not being sent, check if your SMTP server has rate limiting and adjust the program accordingly.