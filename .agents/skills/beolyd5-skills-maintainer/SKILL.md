---
name: beolyd5-skills-maintainer
description: >-
  Use this skill to maintain the Beolyd5 skill library — capture durable new knowledge
  from a conversation into skills, and author/edit SKILL.md files to the project's
  standard. Use when a session surfaces a reusable convention, gotcha, decision, or
  workflow worth persisting; when the user says "capture this", "update the skills",
  "write a skill", or "we learned something"; or when reviewing skills for overlap,
  drift, or quality. Owns the authoring rules (voice, conciseness, progressive
  disclosure, evergreen phrasing) and the routing of knowledge to the right skill.
license: Apache-2.0
---

# Beolyd5 skills maintainer

Keep the skill library accurate, non-overlapping, and evergreen, and turn hard-won
session knowledge into persistent skills. Treat a skill like an onboarding guide for the
next agent: capture what worked, the corrections made, and the non-obvious facts.

## When to capture knowledge

Persist a fact only if it is **durable, reusable, and non-obvious**. Capture when a session
produced: a verified codebase convention, a gotcha that cost time, an architectural or
tooling decision and its rationale, a corrected wrong assumption, or a repeatable procedure.

Do **not** capture: one-off task details, generic knowledge the model already has, anything
unverified, or transient state.

## Capture workflow

1. **Verify first.** Confirm the fact against the actual repo (read the file, run the check).
   Never persist an assumption — this project's standing rule is *verify, don't trust*.
2. **Route it.** Find the one skill that owns the domain (see map below). Prefer **updating**
   an existing skill over creating a new one.
3. **Create a new skill only** for a genuinely new, coherent domain not covered by any
   existing skill. A new skill must be a single coherent unit (one expert perspective), not
   a grab-bag. If it would overlap an existing skill, update that skill instead.
4. **Place it correctly** inside the target skill (its gotchas list, the relevant section).
   Cross-reference the canonical owner rather than duplicating a fact across skills.
5. Skills are **auto-discovered** from `.agents/skills/` — no registration in AGENTS.md is
   needed. The harness reads each skill's frontmatter `description` directly.
6. **Validate** against the checklist below and confirm markdown lint is clean.

## Authoring standard (apply to every SKILL.md)

**Frontmatter `description`** (third person; the routing beacon):
- State *what* the skill does and *when* to use it, key use case first; include trigger
  keywords a user would naturally say. Frame as "Use this skill to… / Use when…".
- Keep under ~1024 characters. No hard version numbers, dates, or ephemeral URLs.

**Body** (concise, third-person imperative addressing the agent):
- Encode only **project-specific, non-obvious** knowledge: conventions, contracts, gotchas,
  decisions. Omit anything the model already knows (language/framework basics, generic "best
  practices"). Every line is a recurring token cost once loaded — state what to do, don't
  narrate why at length.
- Active voice, present tense, direct commands ("Decode…", "Route volume to…", "Load
  `beolyd5-architecture` for contracts").
- Prefer a **Gotchas** section for the highest-value corrections.

**Evergreen** (avoid drift):
- Do not hard-code version numbers, patch levels, or dates in `SKILL.md`. Say "the current
  stable major" / "verify against the registry", or qualify with "currently". Quarantine any
  unavoidable time-sensitive fact into a `reference/` file or a dynamic check.

**Progressive disclosure & scope:**
- Keep `SKILL.md` under ~500 lines. If it covers rarely-co-occurring subtasks or carries large
  verbatim artifacts, move detail into `reference/*.md` and tell the agent *when* to load it.
- One coherent unit per skill. Split when two workflows rarely co-occur; keep whole when the
  concern is single-purpose (it only loads for that task anyway).

## Skill routing map

| Domain of the new knowledge | Owning skill |
|---|---|
| HID protocol, physical signal path, DAC/PowerLink/panel, MOTS *concept* | `beosound5-hardware` |
| The `beolyd5_controller` Rust crate | `beolyd5-rust-controller` |
| Tauri host, Vue UI, the event bridge, simulator | `beolyd5-tauri-app` |
| Visual/interaction/UX, B&O aesthetic | `beolyd5-ux-design` |
| Playback daemons, MPRIS, MOTS *implementation* | `beolyd5-audio-backend` |
| Image build, cage kiosk, systemd wiring, CI, OTA | `beolyd5-pi-image` |
| Cross-component contracts, data flow, process model, evolution | `beolyd5-architecture` |
| Rootfs/power-loss, watchdogs, hardening, observability, onboarding | `beolyd5-reliability` |
| Product vision, requirements/specs, roadmap, acceptance, traceability | `beolyd5-product-spec` |
| Test strategy, decode/unit tests, simulator/mocking, CI gates | `beolyd5-testing` |
| The build/test/layout facts every agent needs always | `AGENTS.md` (not a skill) |

AGENTS.md vs skills: put **always-true, always-needed** repo facts (layout, build, gotchas) in
`AGENTS.md`; put **on-demand procedural/domain** knowledge in skills.

## Validation checklist (before finishing)

- [ ] Fact verified against the repo, not assumed.
- [ ] Lives in exactly one owning skill; cross-referenced, not duplicated.
- [ ] Description: what+when, keywords, key use first, no hard versions/dates, <~1024 chars.
- [ ] Body: concise, imperative, project-specific only; no generic filler.
- [ ] No hard-coded versions/dates in `SKILL.md` (evergreen phrasing instead).
- [ ] `name` matches the folder name.
- [ ] Markdown lint clean (blank lines around headings/lists, fenced-code languages, no bare URLs).
