# RUST_APP

The REST API application to calculate dissel usage and probability of unit injector fail.

## INSTALLATION

### git clone

### cd RUST

### cargo build

### cargo build --release

### cargo run

## Endpoints

# GET api/calculateDisselUsageForDistance

The fields distance, year of production and fuelUsagePer100KM are required. Endpoint calculate the diesel usage per distance given by customer.

http://localhost:8000/api/calculateDisselUsageForDistance

# GET api/probabilityOfUnitInjectorFail

The field VIN is required.
API function that returns a random percentage of the chance that the engine of the car will fail.

http://localhost:8000/api/probabilityOfUnitInjectorFail
