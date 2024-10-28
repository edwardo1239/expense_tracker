# Expense Tracker CLI

A command-line expense tracking application written in Rust. This project is based on the [Expense Tracker project](https://roadmap.sh/projects/expense-tracker) from roadmap.sh, implemented as a learning exercise for Rust programming.

## Features

- Add expenses with descriptions and amounts
- List all recorded expenses
- Generate monthly expense summaries
- Delete expenses by ID
- Error handling for invalid inputs
- Persistent data storage

## Usage

The application supports the following commands:

### Add Expense
```bash
expense-tracker add --description "Grocery shopping" --amount 50.50
```

### List Expenses
```bash
expense-tracker list
```

### Monthly Summary
```bash
expense-tracker summary --month 2
```

### Delete Expense
```bash
expense-tracker delete --id 1
```

## Command Format

- `add`: Add a new expense
  - Required flags:
    - `--description`: Description of the expense
    - `--amount`: Amount spent (numeric value)

- `list`: Display all expenses
  - No additional flags required

- `summary`: Generate a monthly expense summary
  - Required flags:
    - `--month`: Month for which to generate the summary

- `delete`: Remove an expense
  - Required flags:
    - `--id`: ID of the expense to delete

## Error Handling

The application includes robust error handling for:
- Invalid commands
- Missing required flags
- Invalid amount formats
- Empty inputs
- Invalid IDs for deletion

## Project Structure

```
src/
├── main.rs         # Application entry point and main loop
├── models/
│   ├── config.rs   # Command line argument parsing
│   ├── expense.rs  # Expense data structure and operations
│   └── error.rs  # Error estructures and erros types
|
└── utils/
    ├─── io_utils.rs # Input/output utilities
    └── file_utils.rs # Read and write utilities
```

## Learning Goals

This project was implemented to practice:
- Rust syntax and concepts
- Error handling in Rust
- Command-line interface development
- Data persistence
- Project structure organization
- Input validation and processing

## Attribution

This project is implemented as a learning exercise, based on the [Expense Tracker project](https://roadmap.sh/projects/expense-tracker) from roadmap.sh.

## License

This project is available under the MIT License. See the LICENSE file for more details.
