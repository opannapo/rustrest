# Rustrest
#

## Overview

This project is built using a combination of modern technologies aimed at creating a highly efficient and scalable backend. The backend utilizes Rust with the `actix-web` framework to handle HTTP requests, `sqlx` for asynchronous database access, and Postgres for data persistence. The project also includes features like transactional database execution and bcrypt hashing for security. Performance testing is done using JMeter.
#

## Technologies Used

- **Rust**: The main language for the backend, providing high performance and memory safety.
- **Tokio**: An asynchronous runtime for Rust to enable concurrent operations.
- **actix-web**: A powerful, actor-based web framework for Rust that helps in building fast and reliable APIs.
- **sqlx**: An asynchronous SQL crate for Rust that interacts with Postgres and handles database queries and transactions.
- **Transactional DB Execution**: Ensures that database queries and operations are executed as part of a transaction, providing reliability and data consistency.
- **JMeter**: A tool used for performance testing, simulating millions of requests to evaluate the application's robustness.
- **Postgres**: A powerful relational database management system (RDBMS) used to persist data.
- **Dotenv**: Manages environment variables, keeping sensitive configurations like database credentials safe.
- **Bcrypt**: A hashing algorithm used to securely store and compare passwords.
#

## Features

- **Async Database Access**: Leverages `sqlx` for efficient, non-blocking database interactions.
- **Transactional Integrity**: Uses `sqlx` transactions to ensure that multiple database operations are executed atomically.
- **Secure Password Storage**: Passwords are hashed using `bcrypt` before being stored in the database.
- **Scalable and Concurrent**: Powered by `actix-web` and `Tokio`, the system is designed to handle large-scale, concurrent operations.
- **Sqlx Migration**: Powered by `sqlx`.

#

## Getting Started

### Prerequisites

- Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Install Postgres: [Postgres Installation Guide](https://www.postgresql.org/download/) / Docker: [Docker](https://hub.docker.com/_/postgres)
- Install JMeter: [JMeter Installation Guide](https://jmeter.apache.org/download_jmeter.cgi)

#

## Setup
Rust & Cargo
```
curl https://sh.rustup.rs -sSf | sh
```

Postgres
```
docker pull postgres
```

JMeter
```
[1] wget https://downloads.apache.org//jmeter/binaries/apache-jmeter-5.3.zip
[2] unzip apache-jmeter-5.3.zip
```

### TypeSense
```
make build-typesense-compose
```
Dashboard (API_KEY : opannapoTESTapiKEY123):<br>
https://bfritscher.github.io/typesense-dashboard/#/apikeys

### Environment Setup

1. Clone the repository:

    ```bash
    git git@github.com:opannapo/rustrest.git
    cd rustrest
    ```

2. Copy the example environment configuration file:

    ```bash
    cp .env .env
    ```

### Running the Project

1. Build (release) the project:

    ```bash
    make build-bin-api-release
    ```

2. Run the project:

    ```bash
    make run-bin-api
    ```


### SqlX Db Migration

1. Run Migration Up:

    ```bash
    make run-bin-migration-up
    ```

2. Run create new migration file:

    ```bash
    make run-bin-migration-new filename=create_table_location
    ```
#

## Performance Testing

To run performance tests using JMeter:

1. Open the JMeter UI and import the test plan.
   ```
   [1] cd apache-jmeter-5.3/bin
   [2] ./jmeter
   ```
2. Run the test plan with your desired load configuration.
3. Review the results and analyze the system performance.

### JMeter Result
- Number of Threads (users) = 1.000.000
- No Thread Delay
- Execution time duration (00:06:48) 6min 48sec
![image](https://github.com/user-attachments/assets/53f8a05f-dd45-47cb-b7a8-a5bc16005919)

### Postgres DB
- Connection Pool Max = 50
- Connection Pool Min = 2
![image](https://github.com/user-attachments/assets/20fb17a6-51f8-4b08-b806-2d98c73c33f0)

### App
- Processing = Inserting 1,000,000 records into the user and credential tables within a transaction via the POST endpoint http://localhost:8080/v1/auth/signup with 1,000,000 requests.
- Actix Web worker count = 20
- Mem Usage = 25.7Mb
- Cpu Usage = 27.05%
![image](https://github.com/user-attachments/assets/8ece02bd-8a83-49db-86c4-2c94883de329)


#

## Specs of the Development Laptop
```bash

neofetch

            .-/+oossssoo+/-.               @legion 
        `:+ssssssssssssssssss+:`           --------------- 
      -+ssssssssssssssssssyyssss+-         OS: Ubuntu 22.04.5 LTS x86_64 
    .ossssssssssssssssssdMMMNysssso.       Host: 82RB Legion 5 15IAH7H 
   /ssssssssssshdmmNNmmyNMMMMhssssss/      Kernel: 6.8.0-51-generic 
  +ssssssssshmydMMMMMMMNddddyssssssss+     Resolution: 2560x1440 
 /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/    DE: GNOME 42.9 
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   WM Theme: Adwaita 
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   Theme: Yaru-bark-dark [GTK2/3] 
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   CPU: 12th Gen Intel i7-12700H (20) @ 4.600GHz 
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   GPU: NVIDIA GeForce RTX 3060 Mobile / Max-Q 
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   GPU: Intel Alder Lake-P 
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   Memory: 9276MiB / 15714MiB 
 /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/     
  +sssssssssdmydMMMMMMMMddddyssssssss+      
   /ssssssssssshdmNNNNmyNMMMMhssssss/       
    .ossssssssssssssssssdMMMNysssso.        
      -+sssssssssssssssssyyyssss+-          
        `:+ssssssssssssssssss+:`            
            .-/+oossssoo+/-.
                                                                   
                                                                   
```

