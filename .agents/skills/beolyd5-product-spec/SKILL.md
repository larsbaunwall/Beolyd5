---
name: beolyd5-product-spec
description: >-
  Use this skill to act as the Product Manager & spec steward for Beolyd5, driving
  spec-driven development and keeping requirement specifications in the repo. Owns the
  product vision, the roadmap, per-feature specs (WHAT/WHY), EARS-style requirements,
  acceptance criteria, and requirement→implementation→test traceability. Use when
  capturing or refining requirements, writing a feature spec, prioritising the roadmap,
  defining acceptance criteria, or checking whether work traces back to a spec.
license: Apache-2.0
---

# Beolyd5 product & specifications

Own the requirements so every feature is written down, testable, and traceable. The
technical **how** belongs to `beolyd5-architecture`; visual/interaction detail to
`beolyd5-ux-design`; this skill owns intent, scope, requirements, and acceptance.

## Repo layout for specs

Keep specs in-repo, versioned alongside code, one folder per coherent feature:

```text
specs/
  product.md              # Vision, target user, non-goals (the "constitution")
  roadmap.md              # Prioritised features per version (v1 audio → v2 hub)
  <feature>/
    spec.md               # WHAT/WHY: user stories, EARS requirements, acceptance
    plan.md               # optional HOW summary — defer to beolyd5-architecture
    tasks.md              # optional ordered, testable task breakdown
```

Rule: **no feature work without a `spec.md`.** A PR references the requirement/task IDs it
implements. Keep specs "function-cut" — each covers one end-to-end user journey (e.g.
`now-playing`, `mots`, `source-switching`, `wifi-onboarding`), not a grab-bag.

## Writing requirements (EARS)

Write each requirement atomically with a stable ID and the testable verb "**SHALL**". Use the
EARS templates:

- **Ubiquitous:** "The system SHALL …"
- **Event (WHEN):** "WHEN <trigger>, the system SHALL …"
- **State (WHILE):** "WHILE <state>, the system SHALL …"
- **Unwanted (IF/THEN):** "IF <bad condition>, THEN the system SHALL …"
- **Optional context (WHERE):** "WHEN <event> WHERE <context>, the system SHALL …"

Project-grounded examples (as they'd appear in `specs/now-playing/spec.md`):

```markdown
R-001  WHEN the angular wheel moves the pointer onto a menu item, the system SHALL
       select that item and emit the mechanical click.
R-002  WHILE no BS5 controller is connected, the system SHALL keep the simulator input
       path working and surface a "controller not found" diagnostic.
R-003  IF a playback daemon stops responding, THEN the system SHALL mark that source
       unavailable and continue serving the other sources.
R-004  The system SHALL display cover art, title, and artist for the active source.
```

Keep IDs stable even when wording is refined. Namespace per feature if useful (`NP-R-001`).

## Acceptance criteria (Given/When/Then)

Every requirement gets at least one acceptance check the tester can automate or run:

```markdown
AC-001 (verifies R-001)
  Given the menu is showing and the pointer is between items
  When the angular wheel advances past an item's threshold angle
  Then the item becomes selected and a single tick fires
```

## Traceability

Maintain an ID chain so anyone can see a requirement's status end-to-end:

- `spec.md` requirement `R-###` → `plan.md`/architecture component → `tasks.md` `T-###`
  → test case (`beolyd5-testing`) → commit/PR.
- Reference IDs in commits (`feat(now-playing): R-001 R-004 (T-001)`) and PR descriptions.
- For anything non-trivial, keep a lightweight table in `specs/<feature>/spec.md`:

```markdown
| Req   | Design / component      | Tasks | Tests    | Status |
|-------|-------------------------|-------|----------|--------|
| R-001 | MainMenu + arcs pointer | T-001 | TC-001   | done   |
```

## Product framing for Beolyd5

- **Vision:** a modern, open BeoSound 5 experience — the physical wheel + arc UI driving
  real audio, faithful to B&O's calm, minimal design.
- **v1 scope (audio):** the missing views made real — `NowPlaying`, `Music`, `Radio` bound to
  live state; source switching; volume/transport; MOTS; headless Wi-Fi onboarding.
- **v2 direction (information hub):** new domains enter as new *Sources on the arc* (home
  automation, weather) — reuse the same interaction grammar (see `beolyd5-ux-design`,
  `beolyd5-architecture`). Record these as future roadmap items, not v1 requirements.
- **Non-goals:** keep them explicit in `product.md` (e.g. not a touch/phone app; no audio
  engine in the UI).

## Gotchas

- A requirement that can't be turned into an acceptance check is too vague — rewrite it in
  EARS until it is testable.
- Don't specify the **how** in `spec.md` (no stack/algorithm detail) — that couples the spec
  to an implementation and belongs in architecture.
- Keep `specs/` in sync with reality: when behaviour changes, update the spec in the same PR.
