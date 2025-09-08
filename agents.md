# Decisions (outline)

1. ECS modeling

Components over resources; avoid global singletons.

"Events" are resolved in systems during the tick (no persistent event entities).

Graph lives as a component on each entity (not a resource).

2. Graph / Edge weights

Directed, per-entity outgoing edges.

Edge = { to, kind, channel, kernel_id, weight }.

Kinds: Relationship | Pathogen | Infestation | Meme.

3. Interaction kernel (unified)

Effect/intensity: κ = v_affinity ⋅ u_state (dot product).

Channels parametrize κ (talk, shared_air, sex, touch, media, presence, bedding, highway, …).

4. Processes (growth ↔ defense engine)

Node state: L (load) vs E (defense efficacy).

Update per tick: logistic-like growth; adaptive defense; clearance; optional fruiting/shedding.

Shedding drives spread pressure; severity (from L/E) feeds mortality.

5. Diseases & analogs (defaults)

Flu (agent): acute; high shedding; waning immunity.

Chickenpox (agent): childhood-biased kernel; sterilizing immunity.

AIDS (agent): slow; suppresses defense (E) / increases defense decay.

Malaria (people↔location): mosquito load as a location process; people acquire from presence.

Infestations (location): cockroach, bedbug use same L/E engine.

Memes/infatuations (agent): salience as L; boredom/skepticism as E; spreads via talk/media.

6. Transmission / contagion

For an edge: λ = κ × channel_coeff × shedding × susceptibility.

Poisson trial per tick: p = 1 − exp(−λ·Δt).

Success → seed/boost a Process on the target node.

7. Relationships

Same kernel; Δweight = κ − decay(kind, weight, Δt).

"Fruiting" concept applies (e.g., marriage as a high-output phase).

8. Status effects (vectors)

Many concurrent effects (e.g., intoxication, weather) as small vectors with decay/duration.

Aggregated each tick into a clamped StatusVector used by kernels and gates.

9. Locations & space

Locations are entities with inventories; typed as Residential | Business | Commercial | Junction | Highway | Border | Airport | Port.

Travel nodes are locations with FIFO queue + capacity + service_rate; they gate movement between locations.

10. Traffic & accidents

Accident risk evaluated during actions on relevant channels (e.g., Highway).

Risk vector factors: speed, sobriety, seatbelt, vehicle safety, weather; kernel → Poisson injury/death.

11. Decision & actions (per agent, per tick)

Generate options (context-shortlisted actions: Drive, Walk, Work, Shop, Chat, StayHome, QueueEnter/Exit).

Gate with predicates: hard Block or Penalty(risk/cost vector) (illegal/unsafe allowed but riskier).

Score & choose (utility with penalties; selection stochastic via softmax recommended).

Resolve immediately (movement, chats, contagion, accident rolls).

12. Mortality (unified)

All causes as hazards; combine per tick: p_total = 1 − Π(1 − p_i).

Old age: logistic hazard; ~50% near ~90 years.

Disease: hazard derived from Process severity (L/E) by kind.

Accidents: from action-time risk kernel (travel, etc.).

Starvation: simple scalar that weakens with repeated hunger (details deferred).

Suicide/anomie: from meme/meaning process crossing thresholds.

13. Age bias & immunity

Chickenpox adds age-weight to λ (youth-biased).

Per-disease immunity: sterilizing (bool) + waning rate.

14. Time & randomness

Δt is the global tick unit; all rates scale by Δt.

Poisson/Bernoulli for transmissions, accidents, mortality rolls.

15. Data ranges / conventions

Relationship weight in [-1, 1].

Edge weights (pathogen/infestation) in [0, 1]; node Process holds true state (L,E).

Clamp L,E ≥ 0; clamp status vectors to bounded range.

16. Open / intentionally deferred

Licensing/policy system (gates/predicates finalized later).

Exact parameter tables for each disease/infestation/meme.

Starvation detailed kinetics (kept as scalar for now).

Illegal-action policy toggle (soft-penalty vs hard-block) left configurable.


---

## Outline (decisions, terse)

1. Summary

Entities are agents and locations; both can carry state and directed graph edges.

All interactions use one kernel (dot product of source affinity and target state).

Ongoing phenomena (disease, infestations, memes) use one load/defense process on the node.

Events resolve in systems immediately (Poisson draws, no event entities).

Mortality is unified: multiple hazards computed per tick and combined.

---

2. Abstract Systems

**Events**

Systems compute rates from context, draw once, apply effects now.

Ordering via system schedule only; nothing persisted.

**Graph**

Per-entity component of outgoing edges.

Edge: { to, kind, channel, weight, kernel_id }.

Kinds: relationship | pathogen | infestation | meme (extensible).

Weights decay by kind-specific rule; updates come from the kernel.

**Kernel**

effect = v_affinity(kind, channel, source, ctx) ⋅ u_state(target, ctx).

Channels give context (e.g., talk, shared_air, sex, touch, media, presence, bedding, highway).

Status (see below) perturbs v and/or u.

**Load/Defense process (node-local)**

State: L (load), E (defense efficacy).

Per tick: growth raises L; adaptive defense raises E; clearance lowers L.

Optional fruiting/shedding phase boosts outward effect.

Used for: diseases (agents), infestations (locations), memes/infatuations (agents).

**Status Effects**

Many timed modifiers; each is a small vector with decay/duration.

Aggregated each tick → clamped StatusVector used by kernels/gates.

Examples: intoxication, fatigue, weather.

**Decisions**

Per agent per tick: generate options → gate → score → choose → resolve.

Gate = simple predicates against state/context; remove infeasible options.

Score = features·weights minus any penalties; choose (softmax or argmax).

Resolve immediately (movement, chats, contagion, accident checks).


---

3. Agents

**Mortality (unified)**

Per-cause hazard → probability: p_i = 1 − exp(−h_i·Δt).

Combine: p_total = 1 − Π(1 − p_i) (single death roll per tick).

Old age: logistic hazard; ~50% near ~90 (tunable).

Disease: hazard derived from current L,E (severity by disease kind).

Starvation: simple stamina-like scalar; repeated hunger worsens recovery (kept minimal for v1).

Accidents: hazards during risky actions (esp. travel); kernel mixes speed, sobriety, seatbelt, vehicle safety, weather.

Suicide / anomie: meme-style process; when meaning collapses, add hazard.

**Disease (design details)**

Where state lives: on the node (agent); edges only carry exposure pathways.

Parameters per disease kind (set in main):

growth {r,K}; defense {alpha, rho, gamma, n}; shedding {sigma, a, b, fruit?}; transmission {β_by_channel}; immunity {sterilizing, waning_kappa}; hazard mapping (L,E)→h.

Within-host update: rise–peak–fall from growth vs defense; shedding derived from load.

Transmission: per edge/channel: λ = β_channel · (v⋅u) · shedding; draw Poisson; on success, create/boost process on target.

Immunity: sterilizing or waning; affects future susceptibility/defense.

Note (examples are parameter sets only): flu, chickenpox (age-biased kernel, sterilizing), AIDS (suppresses defense), malaria (coupled to location mosquito load).

**Relationships**

Edge weight represents tie strength/valence.

Update on interaction: Δw = kernel − decay(weight, Δt); supports “fruiting” phases (e.g., marriage).

**Action/Status**

ActionPrefs (features/weights) guide scoring.

StatusVector summarizes current temporary effects for kernels and gates.


---

4. Locations

**Space & Movement**

Locations are nodes; travel nodes add { queue (FIFO), capacity, service_rate, channel }.

Movement: enqueue at origin, dequeue by capacity/rate to next hop; congestion = queue growth.

City/country structure is just graphs of these nodes (residential, business, commercial, junction, highway, border, airport, port).

**Infestations**

Same load/defense process on the location (e.g., roaches, bedbugs, mosquito load).

Shedding adds local exposure pressure for present agents via appropriate channels (presence, bedding, food_surface, etc.).

**Accident venues**

Certain channels (e.g., highway) add accident checks during movement resolution; risk uses the same kernel structure with location/agent factors.

---

5. Extra Notes / Undecided

Licensing/policy gates (age, sobriety, etc.) not finalized; action gating exists but policy content TBD.

Starvation kinetics intentionally simple for v1; richer nutrition model deferred.

Exact parameter tables for diseases/infestations/memes set via builders in main.

Tick unit Δt (hours/days) chosen in main; all rates scale accordingly.

## Design & Naming Guidelines (Agents & Jobs)

Files & Modules

Per domain folder: mod.rs, component.rs, plugin.rs, system.rs (optional).

UI goes in that domain under ui.rs and is always #[cfg(feature = "graphics")].

Builder-only types live with their component (e.g., jobs/component.rs has Job, RoleSpec, JobBuilder).

Features & Runtime Modes

Headless is default-safe. Only graphics code behind:

#[cfg(feature = "graphics")] mod view;

In main, gate DefaultPlugins + any UI plugins with #[cfg(feature = "graphics")].

Time: use Time<Real> everywhere unless explicitly sim-virtual later. Do not mix.

Components (nouns, data only)

Name with TitleCase nouns: Person, Job, Unemployed, Age, Inventory.

No behavior inside components. Keep them #[derive(Component, Debug)] if useful.

Boolean tags are single-word marker components (Unemployed, Student). Avoid is_* fields.

Resources (global state)

Name with TitleCase nouns: Records, Gregslist, GameRNG, ApplicationInbox.

Prefer small, purpose-built resources over god-objects.

If a resource broadcasts changes, add an event instead of storing flags.

Events

Past-tense, TitleCase: BabyBorn, Death, VacancyDirty.

Keep payloads minimal (entity ids, role index, timestamp).

Emit with .write(...) in systems; never read-modify-write events.

Systems (present-tense verbs)

Name with snake_case verbs describing one effect:

post_job_openings, apply_for_jobs, evaluate_and_assign, gregslist_expiration_system.

One responsibility per system; chain when needed: (A, B, C).chain().

Parameters are explicit; avoid broad Query<(&A, &B, Option<&C>)> unless necessary.

No global mutable state; mutate via Commands, ResMut<T>, or event writers only.

Plugins

One plugin per domain: GregslistPlugin, HiringManagerPlugin, RecordsPlugin.

Plugins register systems/events/resources—nothing else.

Constructor params reflect operational caps/config (e.g., HiringManagerPlugin::new(max_hires_per_role_per_cycle)).

Jobs & Hiring Domain Terms

Job: component on an entity that hosts roles: Vec<(RoleSpec, Vec<Entity>)>.

RoleSpec: { min, max, constraints } (data only).

Constraints enum lives with jobs; methods on JobBuilder are sugar: age_lt(n), age_gte(n).

Gregslist: ads: Vec<Advert>, index: HashSet<(job, role_index)>.

Advert: { job: Entity, role_index: usize, date_posted: f32 } (no logic).

ApplicationInbox: Vec<Resume> queue consumed by evaluate_and_assign.

Resume: { applicant, job, role_index }.

Unemployed: marker removed on hire; used to compute "employed" count.

UI (graphics feature only)

All UI systems gated with #[cfg(feature = "graphics")].

Text widgets are resources pointing to Entity ids (e.g., PopulationText, EmploymentText).

Update loops read world state only; no side-effects beyond text mutation.

Naming Patterns (consistency)

Functions: verb_object[_qualifier]

Good: record_births, update_employment_text, spawn_jobs.

Config types: ThingConfig (BabySpawnerConfig, GregslistConfig, HiringConfig).

Helpers inside a module: private fn constraints_ok(...) -> bool.

Style

Comments: sentence case, concise; align operators when it clarifies blocks.

Keep systems ≤ ~30 lines; factor helpers if they grow.

No placeholder names; if it’s not real, don’t add it.

Don’t introduce logging/validation/“future hooks” unless asked.

Data Flow (jobs search → hire)

1. gregslist_expiration_system trims stale ads and emits VacancyDirty.

2. post_job_openings reconciles each Job role vs min, (add/remove) Adverts.

3. apply_for_jobs reads Gregslist, filters by constraints, enqueues Resumes for Unemployed.

4. evaluate_and_assign admits up to HiringConfig.max_hires_per_role_per_cycle, removes Unemployed, and emits VacancyDirty.

Metrics & Counters

“Employed” = count<Person> - count<Unemployed>.

Counters live in records domain; UI mirrors via text resources.

Rolling stats (RollingMean) store timestamps; never compute by scanning history each frame.

