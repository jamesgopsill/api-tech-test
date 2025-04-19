# SB&G API Tech Test - Roulette in Rust

The application is my 3hr solution to the [SB&G API Tech Test][1]. It provides a roulette microservice for other services wishing to run a game of roulette for their players. It provides minimal requirements on the data required to run a game and expects the service providing the data to reconcile the result for their users. The service records every game played for auditing and billing purposes to the services offering the roulette game for their users.

The application written in Rust using the [Axum][5] framework. It can be run in either single or multi-threaded modes. A Vec is used as a database stub. There should be no issues in exchanging this for a connection to a database of the organisation's choice. The app could be launched multiple times over on a Kubernetes deployment or serveless stack. A JsonWebToken (JWT) stub is also used for authentication and provides an example of auth through middleware.

The brief is below and the *italics* are my interpretation of what it might mean in terms of requirements and my solution to those requirements for the software architecture. I also plan to record an impl stream on YouTube where I run through the development of this solution.

# The Brief *(and my interpretation)*

## Initial considerations

Our product team would like you to help us build a new [roulette][2] platform. Currently all the different variations of roulette work in different ways, some with more business logic in the front end than is preferred. The product team
aspire to have a single roulette platform which they can concentrate their focus on.

*Ok, so there is a need to manage different variations of the game. E.g., American and European tables. Have it extensible for future table variations. There is an intention is to aggregate lots of existing services that will use this as a core service. So a B2B API to other parts of the organisation providing roulette games. Therefore high-throughput, scalable and general applicability are some key take-aways.*

For this technical test we would like you to create a roulette API. This will be an API that receives requests from a user *(B2B services who offer roulette games to their users in my case)*, simulates a game of roulette, and returns the results. A front end user interface is not required, neither is any consideration of any services which you might expect to be shared; examples of these might be account verification or game history. If you do want to include something like this in your code, please write against a stub - there is certainly no need to write a full implementation.

*So, a REST API that can horizontally and vertically scale through either serverless or Kubernetes type deployment. Focus solely on the API that would be deployable through these methods. Performance and safety is critical so Rust is a great candidate for this type of service.*

We’d like you to consider:

- How bets are placed, how a win or a loss is communicated, how winnings will be shown. Does your implementation allow for single bets, colour bets, odd/even bets, etc.?
  - A `GameRequest` (see `src/game.rs`) is submitted defining the type of game and bets for the game an array. Each bet (`src/bet.rs`) consists of a player identifier, a comma-separated string of winning table values for the bet (this is verified against a pre-compiled hashmap of valid bet strings for the game type.), and the number of chips played. The api will return a `PlayedGame` json (`src/game.rs`) object that details the winning number, when the event occurred and the chips won by each bet. There is currently no limit on the number of bets on any one table or by any one player but these could be added.
- What API methods would be useful to other teams writing calling code (how you can make the API easy to use, is each method doing what someone else would expect it to do?).
  - *The stack uses [utoipa][4] with inline the api doc strings. There is a secondary binary to generate the OpenApi spec from these docstrings which can then be hosted with the other documentation from the other services. I thinkg each method is doing what someone else would expect. A few meetings with prospective users would confirm this before going into production.*
- Testing and maintainability - you should consider what testing is appropriate.
  - *__Correctness__ A set of tests for correctness would need to be implemented to evidence the statistical win rates to the gambling commissions. Third parties could also be brought in to independently test correctness.*
  - *__Regressions__ A set of tests need to be in place to ensure no regressions in the codebase.*
  - *__Robustness__ Fuzzing tests could be used to validate the API responds in a predictable and expected way.*
  - *__Benchmarking__* Load testing to identify the number of games that can be handled by the service to help provisioning.
  - *__Minimal Dependencies__ Relay on a minimal set of dependencies that can be monitored on a timely basis for updates/fixes that the service may benefit from. Bots could automatically track crate updates in the stack.*

## Further considerations

The expectation from the product team is that we produce a fully working system as soon as possible, then continue to add features. We hope that you will think about this expectation as you work.

*I think so. Just let me know which auth and database you would like the API to interface with and the base architecture is there. It would then be a case of building out the tests and games required.*

- As you add more features you might want to consider how they would be rolled out into production. Would your design easily allow feature toggles to be implemented?
  - *I have generalised the game to one of randomly generating a value from a range and checking whether that value exists in the different types of bet that are permitted in the game (i.e. range of values and accepted bets are hard-coded for speed and auditability). Games where these are static can easily be added through another enum and pre-compiled hashmap of bets for the game. I felt dynamically generated tables and bet types were out of the scope of this service. Feature flags could be added to the repo and pre-compiled variants of the service sold to others that could be ran on their infrastructure if that was a business model you wanted to pursue.*
- Could you easily load test your system?
  - *Sure, I have used [criterion][3] with an example of how one might benchmark aspects of the service. Staged deployments on the cloud infrastructure would be required for a true reflection of its load capacity. I think it's likely to be I/O rather than CPU bound and most likely waiting on other service connections (auth and DB) that will be the limiting factor rather than rolling a random value and computing the results of the bets.*
- Are there certain parts of the system you'd like to monitor? How would you monitor them?
  - *Something like AWS cloudwatch coupled with the DynamoDB for game history would provide great monitoring capability. Response times are key so we're not delaying the user experience when playing the game. I suspect this service would not be the bottleneck. Monitoring HTTP errors would provide insights into whether the B2B developers are interacting with the service in the intended way as well as detecting if their are some malicous actors trying to gain access to the system.*
- How would you deploy your system to an environment? Why would it be advantageous to automate this process?
  - *I think a serverless deployment (e.g., AWS Lambda + DynamoDB) would be a suitable candidate. A few lambda fcns could be made hot to handle the base load with suitable provisioning limits to spin up and handle peaks across the day. I would need some more information on the current and forecasted loads to know for sure. This set-up can be easily replicated for development, staging and production (live/test) environments through IAMs and automated deployments and testing through CI/CD (e.g., GitHub Actions). I feel this would be a fairly stable API as the game of roulette is unlikely change. I would use the same pattern for other games rather than try and extend this service beyond the scope of roulette. This would give fine-grained control over the lambda provisioning for different games. Automated deployments are great but if I have the project scoped correctly then I think this project would benefit from semi-automation. Given the criticality of the service, I think it is still useful for human-based checks/reviews in the process and I anticipate the new deployments would be few and far between and centred around maintenance and security.*


# Running the app

There are two binaries in the repo. The application can be run using:

```
cargo run --bin app
```

An OpenApi spec json file can be generated using:

```
cargo run --bin spec
```

The spec could then be hosted elsewhere with the specifications of all the other services perhaps. It could also be fed into some automated CI/CD pipeline on new deployments.

# Testing

Some example tests are provided in the `src` and can be run using the default toolchain. The tests are by no means complete but to give you an idea of how we can build them out.

```
cargo test
```

# Benchmarking

An example benchmark set-up is provided in benches using [criterion][3]. The numbers it provides are fairly arbitrary at the moment but it gives you the idea of how we can go about developing some more robust benchmarks.

```
cargo bench
```

[1]: https://github.com/skybet/api-tech-test
[2]: https://en.wikipedia.org/wiki/Roulette
[3]: https://crates.io/crates/criterion
[4]: https://docs.rs/utoipa/latest/utoipa/
[5]: https://docs.rs/axum/latest/axum/
