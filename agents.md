# Overview
## Abstract Systems
- gameEvent System
    - instantiated with builder syntax
        - has trigger parameter
            - use syntax ".with_trigger(Proximity)"
                - despite the exact example above, proximity will need to be instantiated with a location graph, to be created previously, and it will have to be aware of the inventory system
            - will need something like trigger frequency where events occur according to a Poisson distribution
        - something like "event_resolver" is run when the proximity trigger is set
            - gameEvents will take in whatever resources they need to, perform a calculation and then update some resource
            - the builder syntax for adding something like a "chatEvent" in which two people in close vicinity talk about once every 5 minutes and depending on their personalities either increases or decreases their relationship would look something like "GameEventSys::new().with_trigger(Proximity::new(location_graph)).with_freq(5 * MIN).with_resolver(ChatEvent::new(personality_resolver_matrix, relationship_graph)" -- although I'm iffy on the implementation and syntax of how the event resolver works and even if it's a system or whatever it is exactly.
 
- Graph
    - a graph implemented as a component which is little more than a hashmap associating each neighboring entity with some kind of state, such as a float, vector of numbers, bool, what-have-you.
    - it's important to remember that components are not attached to entities in an ECS, but that a component is a singular thing that maps entities to data and that "adding a component" is actually just telling the component to create an entry for this entity. Thus, the graph component is a single data structure that maps entities to a hashmap of relationships or state.
    - conventional graph terminology to be used: edge, vertex, weight, directed, etc.

- Activity and Decision System
    - agent has a list of actions it can engage in, weighs them according to some criteria and then chooses an action
    - this implies there must be some sort of action class/object whatever and then that there be an evaluation criteria and conditions and restrictions. It's got to be a whole thing.

- Status effects
    - are attached (to a component?) and affect a variable over time
    - system updates and removes the status effect

- Timescale
    - an internal speed parameter is used because the game is stated in terms of seconds, but you're going to want to simulate weeks or years in mere seconds, so we hit limits and try to draw from Poisson distributions instead of simulating each time tick

## Specific Systems
- Mortality
    - Death by Old Age System
        - probability of dying from old age follows a sigmoid curve
            - function of age component
                - age component merely increases each turn
            - parameterized by "steepness" and "sig_max"
                - default parameters cause the curve to max out at 50% probability at the age of 90
    - Disease System
        - contact gameEvent pops up every time there is potential "contact" between people in close proximity
        - each event is associated with air, skin, spit, feces, blood contact and represented as a vector
        - viral load is then updated on each entity's ImmuneComplex component [to do: come up with a better name than ImmuneComplex]
        - system grows as follows:
            - load grows according to a parameterized logistic curve
            - immune system reduces viral load each update
                - effectiveness increases over time
                - how much effectiveness increases over time is a function of how often the white blood cells interact with the disease, which is going to be some function of viral load and white_cell load on its ImmuneComplex component
            - viral load transfer is determined by a spread vector, which in this case is the affinity the disease has for spreading along the lines of air, skin, spit, feces, or blood. Simple linear algebra determines how much of the disease is likely to spread given the contact event vector.
                - a latency curve, another logistic curve parameterized by the specific disease is applied to the viral load transfer to determine what percentage of the spreader's viral load can be transferred from the contact
            - susceptibility reduces the effectiveness of the spread. Transmission will be reduced given the affinity of a particular disease to spreading to people with a compromised immune system, [come up with more examples]
            - examples of default diseases to add: flu, chickenpox, AIDS, malaria
                - malaria may be unique in that it will also infect locations and agents will be infected simply from being in a location
        - white_cell load also goes down each turn [since I'm imagining this as more like immune system health than literal white blood cell count, perhaps we need a better name]
            - the max value for this parameter will be reduced for children and the elderly, and by some accumulating factors such as times you've gotten sick, or other events we may program in later (malnutrition, injury, individual variation)
    - Accidents and Injuries
        - mortality is also modulated by various gameEvents
        - when a user is currently engaged in an activity, there will be some vector associated with it that calculates the probability that a given event will trigger
        - example: see car accident event under Driving
    - Driving
        - highways are modeled as nodes in the location graph
            - the inventory is actually a FIFO queue
            - the driving system dequeues cars from the highway exponentially slower the more cars are on the highway
            - when the inventory is full, the highway is "jammed" and no cars may be queued
        - car accident event
            - car accident event can trigger whenever an agent is engaged in the activity of driving
            - whether or not a traffic accident event is triggered is stochastically generated on the following factors
                - skill of driver
                    - decreased greatly by a drunkenness status effect
                    - decreased slightly by drowsiness status effect
                    - changes with age; should approximately match this data: ![65be8ee8f684e60b48c9725d09a57151.png](:/4621e160ce3e4af7a4e9b39b0858faee)
                - number of people in the current traffic node (linear or nearly linear function): ![18ffe2037d1b17be7a1c95435f4d9890.png](:/78e302d40c9b4d07aa8ace181af130b5)
                - road conditions determined by
                    - water saturation
                        - increases during rainy weather
                        - decreases faster during hot temperatures
                    - ice saturation
                        - water saturation is converted to ice saturation at a given rate by how cold it is
                        - ice saturation is converted to water saturation by warm temperatures
                - whether or not the driver is speeding, which is determined by a combination of orderliness, psychopathy and respect for authority personality traits
            - once the event triggers, it is a proximity event and another agent is chosen in that location
                - first, the severity of the accident is randomly chosen, then how each driver is affected by it and if the total exceeds some threshold, they die
    - Relationships
        - building friendships
            - chat is a proximity event in which two people compare a random subset of their personality, evaluate how they feel about it according to their own personality and increment the edge weight in the relationship graph (they modify their own relationship component)
            - some personality traits come off as negative even to people with that personality trait. So if we model personality as OCEAN plus intelligence, the interaction matrix comes off looking something like this:
                - [[ 1.0, 0.2, 0.0, 0.3, -0.5, 0.1],
                  [ 0.0, 1.0, 0.2, 0.5, -0.2, 0.1],
                  [ 0.1, 0.2, 1.0, 0.3, -0.3, 0.2],
                  [ 0.0, 0.5, 0.3, 1.0, -0.4, 0.0],
                  [-0.2,-0.1,-0.1,-0.2, 1.0,-0.1],
                  [ 0.2, 0.2, 0.2, 0.2,-0.2, 1.0]]
            - relationship score decays over time
            - [todo: make this a logistic curve or something so we have relationship affinity and types so that you don't stop loving your mom just because you haven't seen her in years]
        - marriage
            - [in the future the marriage system is based on the market system, but agents aren't that dissatisfied with being unemployed]
            - every once in a while, an agent will evaluate his relationships with all the women in his life, filter it out by relatedness and gender, and propose marriage to that person, after which they move in together and can have kids [explain this in more detail]
    - Home
        - every entity with a home will be placed in their home should their itinerary be empty
    - Jobs
        - A job is a collection of roles associated with a location
        - A role is a set of requirements along with a job contract
        - A job contract is typically a salary and a schedule requirement.
        - Each role has an associated maximum and minimum.
        - The hiring manager will submit job postings if the number of people in a role falls below a threshold and start firing people if it exceeds a given threshold.
        - Salary expectations gradually rise. If salary expectations exceed the current job's salary by some set percentage, then the agent will start applying to jobs. Remember that applying to jobs and not getting them will reduce their salary expectations.
    - Wantads
        - a pool of job postings
        - refreshes each week
        - seekers apply to jobs
            - a "resume" is generated and queued in the hiring manager's inbox
            - must have evaluation criteria such as minimum salary
            - apply to the X most desirable jobs
            - each week that no job is given, the evaluation criteria weakens
        - hiring manager evaluates resumes
            - removes any resume that does not meet minimum requirement
            - sorts remaining resumes by desirability
            - hires x people at a time where x is going to be initially a setting passed in
    - Schools
        - schools are jobs that hire children as students
        - they also hire teachers
        - perhaps a student/teacher ratio can be set
            - perhaps instead schools just get budgets and the hiring manager just keeps lowering max teachers until they're making even again

## UI
- conceived as a system of documents like a census, jobs report, etc.
- when children are born, etc., an event is sent to update the game statistics

## Coding
- Ontologically obvious main.rs
    - All parameters must be passed in through main in a manner that makes it sound like we're defining and describing the game, essentially like how this document looks. You should be able to read main and figure out exactly how the rest of the codebase works.
        - avoid default instantiations unless it's zero-like
        - favor builder syntax
- Avoid using Bevy Resources unless it actually makes sense that we're defining and creating something not associated with entities and that is special purpose and global.
- use singular terms whenever possible (don't call a file components.rs, call it component.rs even if there are multiple components in it)
- use gamey or common everyday terms when possible

## Design
- Eventually all system behavior will be emergent from simpler systems, which are essentially rough simulations. Always try to go a step more emergent in to get to the feature or statistic you want.
- "Cute ideas" are when you can say something like "a location is just an entity with an inventory system where people can be in the inventory". This allows us to design an inventory system and apply it to locations and get not just code reuse, but the ability to get better emergent behavior
    - other examples: pest infestations are just diseases that infect locations, a dead bedroom occurs when your wife friend-zones you, going to elementary school is just a job position that only hires children
    - Code should enable components and systems to act as "cute ideas" in the future if such a cute idea occurs to me.
- use usize whenever possible. Never use f32 unless you absolutely have to because of how the game is coded.
- Don't inline static functions; they're already inlined by the compiler.
