use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::time::Instant;

fn main() -> Result<()> {
    let start_time = Instant::now();

    let db_file = "example.db";
    let conn = Connection::open(db_file)?;

    // Check if the table exists
    let table_exists = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='employees';",
        NO_PARAMS,
        |row| row.get::<usize, i32>(0)
    )? > 0;

    if !table_exists {
        // Create table
        conn.execute(
            "CREATE TABLE employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                department TEXT NOT NULL
            )", NO_PARAMS,
        )?;

        // Insert initial data (Create)
        conn.execute(
            "INSERT INTO employees (name, department) VALUES (?, ?), (?, ?), (?, ?)",
            params!["Alice", "HR", "Bob", "Engineering", "Charlie", "Marketing"],
        )?;

        println!("Data after insertion (Create):");
        let mut stmt = conn.prepare("SELECT id, name, department FROM employees")?;
        let employee_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                department: row.get(2)?,
            })
        })?;

        for employee in employee_iter {
            println!("{:?}", employee?);
        }
    } else {
        println!("Database already exists. Using the existing database.");
    }

    // Read data from the table (Read)
    println!("\nData retrieved (Read):");
    let mut stmt = conn.prepare("SELECT id, name, department FROM employees")?;
    let employee_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            department: row.get(2)?,
        })
    })?;

    for employee in employee_iter {
        println!("{:?}", employee?);
    }

    // Update data (Update)
    conn.execute(
        "UPDATE employees SET department = 'Sales' WHERE name = 'Charlie'",
        NO_PARAMS,
    )?;

    println!("\nData after update (Update):");
    let mut stmt = conn.prepare("SELECT id, name, department FROM employees")?;
    let employee_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            department: row.get(2)?,
        })
    })?;

    for employee in employee_iter {
        println!("{:?}", employee?);
    }

    // Delete data (Delete)
    conn.execute("DELETE FROM employees WHERE name = 'Bob'", NO_PARAMS)?;

    println!("\nData after deletion (Delete):");
    let mut stmt = conn.prepare("SELECT id, name, department FROM employees")?;
    let employee_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            department: row.get(2)?,
        })
    })?;

    for employee in employee_iter {
        println!("{:?}", employee?);
    }
    // query1
    println!("\nEmployee count by department：");
    let mut stmt = conn.prepare("SELECT department, COUNT(*) FROM employees GROUP BY department")?;
    let dept_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    })?;

    for dept in dept_iter {
        println!("{:?}", dept?);
    }

    // query2
    println!("\nEmployees whose names start with 'A'：");
    let mut stmt = conn.prepare("SELECT id, name, department FROM employees WHERE name LIKE 'A%'")?;
    let employee_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            department: row.get(2)?,
        })
    })?;

    for employee in employee_iter {
        println!("{:?}", employee?);
    }

    // Print the execution time
    let duration = start_time.elapsed();
    println!("\nExecution time: {:?}", duration);

    Ok(())
}

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    department: String,
}
