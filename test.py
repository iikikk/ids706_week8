import sqlite3
import os
import time
from memory_profiler import profile

@profile  # This decorator enables memory profiling for the function
def main():
    start_time = time.time()
    db_file = 'example.db'
    db_exists = os.path.isfile(db_file)

    # Connect to the SQLite database
    conn = sqlite3.connect(db_file)
    print("Database connection successful.")  # Confirm successful connection
    cursor = conn.cursor()

    if not db_exists:
        # Create a table
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                department TEXT NOT NULL
            )
        ''')

        # Insert data into the table (Create)
        cursor.execute('''
            INSERT INTO employees (name, department)
            VALUES ('Alice', 'HR'), ('Bob', 'Engineering'), ('Charlie', 'Marketing')
        ''')
        conn.commit()

        print("Data after insertion (Create):")
        cursor.execute('SELECT * FROM employees')
        rows = cursor.fetchall()
        for row in rows:
            print(row)
    else:
        print("Database already exists. Using the existing database.")

    # Read data from the table (Read)
    print("\nData retrieved (Read):")
    cursor.execute('SELECT * FROM employees')
    rows = cursor.fetchall()
    for row in rows:
        print(row)

    # Update a record
    cursor.execute('''
        UPDATE employees
        SET department = 'Sales'
        WHERE name = 'Charlie'
    ''')
    conn.commit()

    # Print the data after update (Update)
    print("\nData after update (Update):")
    cursor.execute('SELECT * FROM employees')
    rows = cursor.fetchall()
    for row in rows:
        print(row)

    # Delete a record
    cursor.execute('''
        DELETE FROM employees
        WHERE name = 'Bob'
    ''')
    conn.commit()

    # Print the data after deletion (Delete)
    print("\nData after deletion (Delete):")
    cursor.execute('SELECT * FROM employees')
    rows = cursor.fetchall()
    for row in rows:
        print(row)

    # First custom SQL query: Count the number of employees in each department
    print("\nEmployee count by department:")
    cursor.execute('''
        SELECT department, COUNT(*) FROM employees GROUP BY department
    ''')
    dept_counts = cursor.fetchall()
    for dept in dept_counts:
        print(dept)

    # Second custom SQL query: Select employees whose names start with 'A'
    print("\nEmployees whose names start with 'A':")
    cursor.execute('''
        SELECT * FROM employees WHERE name LIKE 'A%'
    ''')
    a_names = cursor.fetchall()
    for employee in a_names:
        print(employee)

    # Close the connection
    conn.close()

    end_time = time.time()
    print(f"Execution time: {end_time - start_time} seconds")

if __name__ == "__main__":
    main()
