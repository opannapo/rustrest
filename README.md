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
### Rust & Cargo
```
curl https://sh.rustup.rs -sSf | sh
```

### Postgres
```
docker pull postgres
```

### JMeter
```
[1] wget https://downloads.apache.org//jmeter/binaries/apache-jmeter-5.3.zip
[2] unzip apache-jmeter-5.3.zip
```
#### JSR223 PreProcessor - Random Variable for Signup Simulation
```java
import net.datafaker.Faker;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Random;

Faker faker = new Faker();

SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd");

// Inisialisasi Random
Random rand = new Random();
// Pilih negara secara acak
String[] countries = {"Indonesia", "Malaysia", "Singapore"};
String selectedCountry = countries[rand.nextInt(countries.length)];

// Tentukan batas koordinat masing-masing negara
double minLat, maxLat, minLon, maxLon;

switch (selectedCountry) {
    case "Indonesia":
        minLat = -11.0; maxLat = 6.0;
        minLon = 95.0; maxLon = 141.0;
        break;
    case "Malaysia":
        minLat = 0.85; maxLat = 7.4;
        minLon = 99.6; maxLon = 119.3;
        break;
    case "Singapore":
        minLat = 1.2; maxLat = 1.5;
        minLon = 103.6; maxLon = 104.0;
        break;
    default:
        throw new RuntimeException("Negara tidak valid");
}

// Generate koordinat acak dalam rentang
double randomLat = minLat + (maxLat - minLat) * rand.nextDouble();
double randomLon = minLon + (maxLon - minLon) * rand.nextDouble();



String randomName = faker.name().fullName();
String randomEmail = faker.internet().emailAddress();
String randomPassword = faker.internet().password();
String randomBirthday = dateFormat.format(faker.date().birthday(18, 65)); // Format ke String


// Simpan ke variabel JMeter
vars.put("randomName", randomName);
vars.put("randomEmail", randomEmail);
vars.put("randomPassword", randomPassword);
vars.put("randomBirthday", randomBirthday);
vars.put("randomLatitude", String.valueOf(randomLat));
vars.put("randomLongitude", String.valueOf(randomLon));
vars.put("selectedCountry", selectedCountry);

// Debugging log
log.info("Generated randomName: " + randomName);
log.info("Generated randomEmail: " + randomEmail);
log.info("Generated randomPassword: " + randomPassword);
log.info("Generated randomBirthday: " + randomBirthday);
log.info("Generated Country: " + selectedCountry);
log.info("Generated Latitude: " + randomLat);
log.info("Generated Longitude: " + randomLon);
```

### TypeSense
```
make build-typesense-compose
```
Dashboard (API_KEY : opannapoTESTapiKEY123):<br>
https://bfritscher.github.io/typesense-dashboard/#/apikeys

#### Create Collection : User
cURL
```
curl --location 'http://localhost:8108/collections' \
--header 'X-TYPESENSE-API-KEY: opannapoTESTapiKEY123' \
--header 'Content-Type: application/json' \
--data '{
    "name": "user",
    "fields": [
      { "name": "id", "type": "string" },
      { "name": "name", "type": "string", "optional": true, "facet": false, "index": true },
      { "name": "birthdate", "type": "string", "optional": true, "index": false },
      { "name": "gender", "type": "string", "optional": true, "facet": true },
      { "name": "location", "type": "geopoint", "facet": false, "index": true },
      { "name": "created_at", "type": "int64", "facet": false, "index": true }
    ]
  }'
```
Payload
```
{
    "name": "user",
    "fields": [
      { "name": "id", "type": "string" },
      { "name": "name", "type": "string", "optional": true, "facet": false, "index": true },
      { "name": "birthdate", "type": "string", "optional": true, "index": false },
      { "name": "gender", "type": "string", "optional": true, "facet": true },
      { "name": "location", "type": "geopoint", "facet": false, "index": true },
      { "name": "created_at", "type": "int64", "facet": false, "index": true }
    ]
  }
```

#### Search User
cURL
```
curl --location 'http://localhost:8108/multi_search?x-typesense-api-key=opannapoTESTapiKEY123' \
--header 'Accept: application/json, text/plain, */*' \
--header 'Accept-Language: en-US,en;q=0.9,id;q=0.8' \
--header 'Connection: keep-alive' \
--header 'Content-Type: text/plain' \
--header 'Origin: https://bfritscher.github.io' \
--header 'Sec-Fetch-Dest: empty' \
--header 'Sec-Fetch-Mode: cors' \
--header 'Sec-Fetch-Site: cross-site' \
--header 'User-Agent: Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Mobile Safari/537.36' \
--header 'sec-ch-ua: "Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99"' \
--header 'sec-ch-ua-mobile: ?1' \
--header 'sec-ch-ua-platform: "Android"' \
--data '{
    "searches": [
        {
            "exhaustive_search": true,
            "query_by": "name,gender",
            "highlight_full_fields": "name,gender",
            "collection": "user",
            "q": "opan",
            "facet_by": "gender",
            "max_facet_values": 10,
            "page": 1,
            "per_page": 100
        }
    ]
}' 
```

#### Search User By Location - Radius 20Km from Jakarta
cURL
```
curl --location 'http://localhost:8108/multi_search?x-typesense-api-key=1UISuGDjx1FADyRkr01TIAQbLi0suvlG' \
--header 'Content-Type: application/json' \
--data '{
    "searches": [
        {
            "exhaustive_search": true,
            "query_by": "name,gender",
            "highlight_full_fields": "name,gender",
            "collection": "user",
            "q": "",
            "facet_by": "gender",
            "max_facet_values": 10,
            "page": 1,
            "per_page": 100,
            "filter_by": "location:(-6.2088, 106.8456, 20 km)"
        }
    ]
}'
```
Example Result
```
{
    "results": [
        {
            "facet_counts": [
                {
                    "counts": [
                        {
                            "count": 11,
                            "highlighted": "M",
                            "value": "M"
                        }
                    ],
                    "field_name": "gender",
                    "sampled": false,
                    "stats": {
                        "total_values": 1
                    }
                }
            ],
            "found": 11,
            "hits": [
                {
                    "document": {
                        "birthdate": "1987-10-23",
                        "created_at": 1739564190,
                        "gender": "M",
                        "id": "d4151fb1-1f38-4a25-ae53-e564b41f7126",
                        "location": [
                            -6.2773045877828855,
                            106.75307093418604
                        ],
                        "name": "Miss Bertha Maggio"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1993-07-28",
                        "created_at": 1739564153,
                        "gender": "M",
                        "id": "f933bc2a-115f-47b1-8ec8-8584e6ee8a99",
                        "location": [
                            -6.1575376039061505,
                            106.79271478235609
                        ],
                        "name": "Marietta Corwin"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1966-09-21",
                        "created_at": 1739564068,
                        "gender": "M",
                        "id": "9b01c347-4956-40f6-b1fe-2a52c69d6a88",
                        "location": [
                            -6.361107028810105,
                            106.90698950922103
                        ],
                        "name": "Dale Von"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "2000-09-18",
                        "created_at": 1739564038,
                        "gender": "M",
                        "id": "1d4771c3-6b8e-4797-ae1b-317d01a3a30f",
                        "location": [
                            -6.199762270354816,
                            106.82928553951272
                        ],
                        "name": "Kenny Rosenbaum"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1967-09-12",
                        "created_at": 1739563957,
                        "gender": "M",
                        "id": "5c95558b-d305-401d-a348-2b88c0ec017a",
                        "location": [
                            -6.199906018885609,
                            106.8907922763888
                        ],
                        "name": "Kory Franecki DDS"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "2001-11-21",
                        "created_at": 1739563934,
                        "gender": "M",
                        "id": "17a3e44f-7d99-47b0-8a9a-59c613406bf0",
                        "location": [
                            -6.09314404559615,
                            106.86837746717852
                        ],
                        "name": "Doug Herman DDS"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1960-09-08",
                        "created_at": 1739563769,
                        "gender": "M",
                        "id": "f19316d8-0940-4f2e-90a2-a772d3178afb",
                        "location": [
                            -6.231700501300244,
                            106.67398401525512
                        ],
                        "name": "Faustino Doyle"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1995-09-05",
                        "created_at": 1739563623,
                        "gender": "M",
                        "id": "d0bb3be7-6273-47d2-8275-b833416593fa",
                        "location": [
                            -6.09730410213408,
                            106.93067136446145
                        ],
                        "name": "Leann Krajcik"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "2000-05-16",
                        "created_at": 1739563620,
                        "gender": "M",
                        "id": "1f8622fa-a96f-42be-afc9-9436420e2105",
                        "location": [
                            -6.384327640653132,
                            106.83853896477709
                        ],
                        "name": "Ms. Kurtis Barrows"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1982-11-25",
                        "created_at": 1739563591,
                        "gender": "M",
                        "id": "dcbc4b54-2d71-4f8f-84de-1706e6022595",
                        "location": [
                            -6.194732171864411,
                            106.89593038206074
                        ],
                        "name": "Dr. Freddy Turner"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                },
                {
                    "document": {
                        "birthdate": "1988-07-20",
                        "created_at": 1739563480,
                        "gender": "M",
                        "id": "02c3c145-192b-476d-9261-5fb8631608dd",
                        "location": [
                            -6.2349,
                            107.0008
                        ],
                        "name": "opannapo"
                    },
                    "highlight": {},
                    "highlights": [],
                    "text_match": 100,
                    "text_match_info": {
                        "best_field_score": "0",
                        "best_field_weight": 12,
                        "fields_matched": 4,
                        "num_tokens_dropped": 1,
                        "score": "100",
                        "tokens_matched": 0,
                        "typo_prefix_score": 255
                    }
                }
            ],
            "out_of": 195909,
            "page": 1,
            "request_params": {
                "collection_name": "user",
                "first_q": "",
                "per_page": 100,
                "q": ""
            },
            "search_cutoff": false,
            "search_time_ms": 1
        }
    ]
}
```

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

