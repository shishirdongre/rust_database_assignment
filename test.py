import os
import subprocess
import random
import string

# Define table name
table_name = "users"

# Define a static schema with 10 columns, a mix of INT and STRING types, with some columns marked as NOT NULL
columns = [
    "id INT NOT NULL",
    "username STRING(100) NOT NULL",
    "age INT",
    "email STRING(100) NOT NULL",
    "phone STRING(20)",
    "city STRING(50) NOT NULL",
    "state STRING(50)",
    "zip_code INT",
    "account_balance INT",
    "membership STRING(20) NOT NULL"
]

# Join columns to create the table schema
columns_str = ", ".join(columns)
create_table_command = f"CREATE TABLE {table_name} ({columns_str});"

# Step 1: Execute create table command
subprocess.run(["./target/release/db_project", create_table_command], check=True)

# Step 2: Insert 1,000 rows of data
def generate_random_string(length=10):
    return ''.join(random.choices(string.ascii_letters, k=length))

for row_id in range(1, 1001):
    # Generate values for each column in the schema
    values = [
        str(row_id),  # id
        f"'{generate_random_string(8)}'",  # username
        str(random.randint(18, 100)),  # age
        f"'{generate_random_string(5)}@example.com'",  # email
        f"'{random.randint(1000000000, 9999999999)}'",  # phone
        f"'{generate_random_string(6)}'",  # city
        "NULL" if random.choice([True, False]) else f"'{generate_random_string(6)}'",  # state (nullable)
        str(random.randint(10000, 99999)),  # zip_code
        str(random.randint(0, 10000)),  # account_balance
        f"'{random.choice(['Basic', 'Premium', 'VIP'])}'"  # membership
    ]

    # Join values to create the insert statement
    values_str = ", ".join(values)
    insert_command = f"INSERT INTO {table_name} VALUES ({values_str});"
    subprocess.run(["./target/release/db_project", insert_command], check=True)

# Step 3: Select all data to verify
select_command = f"SELECT * FROM {table_name};"
subprocess.run(["./target/release/db_project", select_command], check=True)
