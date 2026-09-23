# Manage Me

A payment management platform for diaspora workers. Managers living abroad configure and automate payments to their workers back home — with scheduling, proof of payment, device (router) tracking, and a full audit trail.

---

## Core Concept

The platform revolves around two roles and the relationship between them:

**Manager** — A diaspora user (living abroad) who employs one or more workers back home. They configure payment schedules, amounts, and currencies. They may also ship physical routers (devices) to their workers to provide internet access.

**Link** — A worker back home. They have an employment relationship with a manager and receive payments according to the configured schedule. They may also hold a router shipped by their manager.

The employment relationship itself is called a **Link** — a Link record represents the contract between one manager and one worker. A manager can have many Links; a worker can appear in Links with different managers.

---

## User Personas

### Manager (Diaspora)
- Lives abroad (e.g. UK, USA, Canada)
- Employs 1–20 workers back home
- Wants to pay without relying on the worker to ask
- Needs proof of payment for accounting / disputes
- May manage physical routers shipped to workers
- Pain point: manually tracking payment due dates across multiple workers, different currencies, different schedules

### Link (Worker, back home)
- Lives in the manager's home country
- Receives salary on a configured schedule
- Wants visibility into when next payment is coming
- May have a router the manager shipped to them
- Pain point: not knowing when to expect payment, having to ask the manager

---

## Domain Glossary

| Term | Meaning |
|---|---|
| **Manager** | A verified diaspora user who employs workers |
| **Link** | An employment relationship record (manager ↔ worker) |
| **Payment Configuration** | Manager-set rules for how a link is paid (schedule, amount, currency). Versioned — old configs are kept when updated. |
| **Cycle Anchor Date** | The epoch of a payment schedule. All due dates are computed as multiples of the interval from this date. Prevents schedule drift from late/early payments. |
| **Payout** | An actual payment transaction — an immutable ledger entry recording what was paid, when, and by whom |
| **Payment Mandate** | A standing authorization for the system to auto-charge the manager and pay a link on schedule |
| **Scheduled Payment** | A generated queue entry for one upcoming auto-payment cycle |
| **Device** | A router owned by a manager, possibly shipped to and held by a link |
| **Device Assignment** | A custody record — who is physically holding a device, and for how long |
| **Overtime Policy** | A reusable rule set defining overtime multipliers, caps, and calculation method |

---

## Key Features

### Payment Management
- Configure payment schedule per link: weekly, biweekly, semi-monthly, monthly, or custom interval
- Schedule is anchored (not rolling) — due dates never drift regardless of when the manager last paid
- Next due date is always computable from the cycle anchor and schedule type — no stale state
- Full version history of payment configurations — you can reconstruct what terms applied on any past date

### Auto-Payment
- Manager enables auto-pay on a per-link basis
- Manager stores a payment method (card or bank debit via Paystack/Stripe)
- System generates scheduled payment queue entries ahead of time
- On due date, system charges manager's payment method and records a payout
- Retry logic with backoff on failure; manager notified on failure
- Both manager and link notified on each payment

### Proof of Payment
- Manager can upload a file (receipt, screenshot) when recording a manual payout
- Each payout record stores a reference to the proof file
- Links can view their payment history including proof documents

### Dashboard — Manager View
- Total links (active / inactive / pending)
- Upcoming payments in the next 7 / 14 / 30 days
- Overdue payments (past due date, not yet paid)
- Total paid this week / month / year
- Payment status by link (on schedule, overdue, auto-pay enabled/disabled)
- Device inventory and assignment status

### Dashboard — Link View
- Next payment due date and amount
- Payment history with proof documents
- Current employment details (job type, start date)
- Device held (if any)

### Device / Router Management
- Manager registers devices they own
- Manager assigns a device to a link (ships to them)
- Custody is tracked with full assignment history (`assigned_at`, `returned_at`)
- Device status: With Manager | In Transit | With Link | Offline | Lost
- Current holder always queryable without recomputing

### Overtime
- Overtime policies are reusable rule sets (e.g. "1.5× after 40h", "2× on weekends")
- A payment configuration can reference an overtime policy
- Overtime records track actual hours worked per period and require manager approval
- Approved overtime amounts flow into the payout calculation

### Notifications & Reminders
- Upcoming payment reminders (configurable: 1 day, 3 days, 7 days before due date)
- Overdue payment alerts
- Auto-payment failure alerts
- Payment received confirmation (to link)
- Device assignment / return notifications

### Audit Trail
- All configuration changes are versioned, not overwritten
- Payout records are immutable — never edited after creation
- Link manager history tracks all manager transfers with consent records
- Device assignment history tracks full custody chain
- Audit log table captures who changed what and when on all critical tables

---

## User Flows

### Manager: Add a Link and Configure Payment

1. Manager creates account → verification flow (manager status: pending → verified)
2. Manager adds a worker by email or invite link
3. Worker accepts → Link record created (status: pending → active)
4. Manager opens Link detail → Configure Payment
5. Selects schedule type (monthly), sets base amount and currency, sets cycle anchor date
6. Optionally attaches an overtime policy
7. Configuration saved → first scheduled payment generated
8. Manager optionally enables auto-pay → selects or adds a payment method

### Manager: Record a Manual Payment

1. Manager opens Link detail → Payments tab
2. Sees current due date and amount
3. Clicks "Record Payment"
4. Enters amount paid, date, optional notes
5. Uploads proof of payment (receipt image or PDF)
6. Payout record created → link notified

### Auto-Payment Flow (System)

1. Scheduled job runs daily — finds `ScheduledPayments` where `due_date <= today` and `status = Pending`
2. For each entry: charges manager's stored payment method
3. On success: creates `Payout` record, links to `ScheduledPayment`, generates next cycle entry
4. On failure: retries up to 3× with backoff, marks `Failed`, notifies manager
5. Manager sees failure in dashboard → can retry manually or update payment method

### Manager: Ship a Router to a Link

1. Manager opens Devices → Registers device (name, IP, routes_to location)
2. Manager opens Device detail → Assign to Link
3. Selects link, adds shipping notes / tracking number
4. `DeviceAssignment` record created with `assigned_at`, `returned_at = NULL`
5. Device status → `InTransit` → manually updated to `WithLink` on confirmation
6. Link sees assigned device in their dashboard

### Link Transfer Between Managers

1. Current manager initiates transfer → selects new manager
2. Worker consent captured (explicit approval required)
3. `LinkManagerHistory` record created: `from_manager_id`, `to_manager_id`, `transferred_at`, `worker_consented`
4. Active payment configuration `valid_until` set to transfer date
5. New manager prompted to set new payment configuration

---

## Screen Inventory

### Auth
- Sign Up (email + password)
- Log In
- Manager Verification (pending state — submit business details)
- Invite Accept (link follows invite URL)

### Manager Dashboard
- Overview (KPI tiles: total links, upcoming payments, overdue count, total paid MTD)
- Links List (table: name, status, next due date, amount, auto-pay on/off)
- Link Detail (tabs: Overview, Payments, Overtime, History)
- Add / Edit Payment Configuration (form: schedule, amount, currency, anchor date, overtime policy)
- Record Manual Payment (form: amount, date, proof upload)
- Devices List (table: name, status, current holder, IP, location)
- Device Detail (assign, unassign, edit, view assignment history)
- Auto-Pay Settings (payment methods, mandate status per link)
- Notifications / Reminders Settings
- Account Settings

### Link Dashboard
- Overview (next payment, amount, employment details)
- Payment History (list of payouts with proof download)
- My Device (if assigned)

### Shared
- Notification Center (in-app)
- Profile / Settings

---

## Data Model — High Level

```
users
  └── managers (one-to-one with users; has verification status)
        └── links (many manager → many workers)
              ├── payment_configurations (versioned; valid_from / valid_until)
              │     └── payouts (immutable ledger; references config)
              ├── link_payment_details (bank account / mobile money)
              ├── payment_mandates (auto-pay authorization)
              │     └── scheduled_payments (generated queue; one per cycle)
              ├── overtime_records (hours worked, requires approval)
              └── link_manager_history (transfer audit trail)

devices (owned by manager)
  └── device_assignments (custody: which link holds it, assigned/returned dates)

overtime_policies (reusable; manager-defined; referenced by payment_configurations)
manager_payment_methods (stored card/bank tokens from payment processor)
audit_log (append-only; captures every write on critical tables)
```

### Payment Schedule Logic

All due dates are derived — never stored and never drift:

```
next_due = cycle_anchor_date + N × interval
where N = ceil((today − cycle_anchor_date) / interval)
```

`cycle_anchor_date` is set once by the manager when creating the configuration. A monthly anchor of `2025-01-15` means payments are always due on the 15th of every month, regardless of when the manager last paid.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust (Axum web framework) |
| Database | PostgreSQL via SQLx (compile-time checked queries) |
| Frontend | HTMX / WASM |
| Background Jobs | Tokio async tasks (payment scheduler, reminder dispatch) |
| Payment Processor | Paystack (primary — NG/Africa support) or Stripe |
| File Storage | S3-compatible (proof of payment uploads) |

---

## Design Considerations for UI

**Primary users are non-technical.** The manager is a diaspora worker — not a developer. Language should be plain. Avoid financial jargon.

**Trust is the core value.** The link trusts the manager will pay. The manager trusts the system will remind them and eventually automate it. Every UI element should reinforce reliability: clear due dates, clear statuses, clear history.

**Two distinct dashboards.** Manager and link views are completely different. Manager has configuration power; link has read/transparency access.

**Status states to surface clearly:**
- Link: `Pending` (invite not yet accepted) | `Active` | `Inactive` | `Suspended` | `Terminated`
- Payment: `Upcoming` | `Due Today` | `Overdue` | `Paid` | `Auto-Pay On` | `Auto-Pay Failed`
- Device: `With Manager` | `In Transit` | `With Link` | `Offline` | `Lost`

**Proof of payment is a key trust feature.** Links should always be able to see receipts. Managers should always be able to attach them.

**Mobile-first.** Both roles are likely on mobile — managers checking from abroad, links checking from their phones. Dashboard should be usable at 375px width.

**Currency handling.** The system spans countries. Amounts should always display with currency code, never just a symbol. Format: `NGN 150,000` not `₦150,000` (symbols are ambiguous cross-border).
