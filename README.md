**find optimal optimal path for drones in a grid of assigned values**

*solution1*
* written in java
* single drone solution
* employs a floodfill algorithm
* where possible paths are always sorted in descending order of value (so far) of the points in the path
* and which uses backtracking to find alternative paths that may be of higher value
* non-recursivity ensures no issues for large grids
* the algorithm is embedded in a rest api
* it also has a html canvas frontend that draws the path on the grid

running:
* install jdk22, and apache maven
* run `mvn spring-boot:run`
* go to http://localhost:8080
* in the console in the ui type `fly` [enter]

*solution2*
* written in rust
* single and multiple drones
* see tests and main for validity of the algorithm
