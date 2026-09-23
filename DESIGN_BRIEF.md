# Manage Me — UI Design Brief

## What This Product Is

**Manage Me** is a payment management platform for diaspora workers. A diaspora user (the **Manager**) lives abroad and employs workers back home (called **Links**). The manager is responsible for configuring how each link gets paid — schedule, amount, currency — and for recording proof of payment. The system tracks payout history, sends reminders before a payment is due, and provides a full audit trail of who managed who and when.

Workers (**Links**) have their own view: they can see their upcoming payment, their payment history, and manage their bank details.

---

## Roles

### Manager
- Lives abroad (diaspora)
- Employs one or more Links
- Configures payment schedules and amounts per Link
- Records payouts with proof of payment
- Owns physical devices (laptops, phones) and routers that are shipped to Links
- Tracks custody of those assets
- Must be verified before they can manage Links

### Link (Worker)
- Lives in the home country
- Receives payments from their Manager
- Can view upcoming payment dates and payment history
- Manages their own bank account details and preferred currency
- May hold physical devices/routers on behalf of the Manager

---

## Authentication & Onboarding

### Pages Required

**1. Landing / Sign-in gate**
- Clean marketing-style entry page
- "Sign In" and "Create Account" CTAs

**2. Register**
- Name, email, password
- Role selection: **Manager** or **Worker (Link)**
- Manager registration note: account requires verification before full access

**3. Login**
- Email + password
- "Forgot password?" link

**4. Forgot Password**
- Email input, send reset link

**5. Reset Password**
- New password + confirm

**6. Manager Verification Pending (state screen)**
- Shown after registration for Manager role
- Explains that account is under review
- Status badge: `Pending` → `Verified` → `Suspended` → `Revoked`

---

## Manager Dashboard

### Overview / Home
Key stats visible at a glance:
- Total active Links
- Payments due this week / this month
- Total paid out (current month, all time)
- Links with overdue payments (past `next_due_date`)
- Quick-action buttons: "Record Payment", "Add Link", "Assign Device"

Charts / data visualisations:
- Monthly payout trend (line or bar)
- Per-link payment status breakdown (on time, overdue, pending)
- Payout by currency split (if managing links in different currencies)

---

### Links

**Links List**
- Table or card grid of all Links
- Columns/fields: Name, Job Type, Status (`Pending`, `Active`, `Inactive`, `Suspended`, `Terminated`), Next Due Date, Last Paid
- Filter by status, search by name
- "Add Link" button

**Add / Invite Link**
- Worker's email or phone to invite
- Job type (text)
- Start date
- Location: Country (required), City (optional), State / Province (optional)
- This triggers the link relationship; payment config is set separately

**Link Detail Page**
Sections:
1. **Header** — worker name, job type, status badge, start date
2. **Current Payment Config** — schedule (daily/weekly/biweekly/semi-monthly/monthly/custom), base amount, currency, next due date, cycle anchor date
3. **Overtime Policy** (if enabled) — multiplier or flat rate, threshold hours
4. **Payment History** — table of payouts: period, base + overtime + bonus + total, currency, status, proof of payment link
5. **Assigned Devices** — list of devices currently with this Link
6. **Assigned Routers** — list of routers currently with this Link
7. **Manager History** — timeline of previous managers (with transfer dates and consent flag)
8. **Bank Details** — payment method, bank name, account number, preferred currency

**Configure Payment**
- Pay schedule selector (daily / weekly / biweekly / semi-monthly / monthly / custom)
- If custom: interval count input
- Pay day (day of week or day of month depending on schedule)
- Base amount + currency
- Cycle anchor date (date picker — this is the epoch all due dates are derived from)
- Overtime policy toggle → select existing policy or create new
- Valid from date

**Record Payout**
- Period start / end date pickers
- Base amount (auto-filled from config, editable)
- Overtime amount (optional)
- Bonus amount (optional)
- Total (auto-calculated)
- Currency
- Proof of payment upload
- Notes
- Status: `Pending` / `Completed` / `Failed` / `Cancelled`

---

### Overtime Policies

**Policies List**
- Manager's reusable policies
- Name, type (multiplier or flat rate), threshold hours, status

**Create / Edit Policy**
- Name
- Enable/disable overtime toggle
- Rate type: multiplier (e.g. 1.5×) or flat rate per hour
- Threshold hours per period before overtime applies

---

### Devices

**Devices List**
- All devices owned by this Manager
- Columns: Name, Type (Laptop / Phone / Tablet / Other), Model, Serial Number, Status (`With Manager` / `In Transit` / `With Link` / `Offline` / `Lost`), Current Link
- "Add Device" button

**Device Detail**
- Device info
- Current holder (Link name, assigned date)
- Full custody history (table: assigned to, assigned by, from/to dates, notes)

**Assign / Return Device**
- Select Link
- Assignment date
- Notes (e.g. tracking number)

---

### Routers

Same structure as Devices but with additional fields:
- Routes to (US/UK location the VPN tunnels back to)
- IP address
- Router type (`Home` / `Enterprise` / `Travel`)

---

### Settings (Manager)

- Profile: name, email, password change
- Notification preferences: enable email notifications, reminder days before due date
- Theme: light / dark / system

---

## Link (Worker) Dashboard

### Overview / Home
- Next payment date (large, prominent)
- Days until next payment countdown
- Last payment received (amount, date)
- Current manager name

### Payment History
- Table of all payouts received
- Period, amount, currency, status, proof of payment (downloadable)

### My Details
- Bank details: payment method, bank name, account number
- Preferred currency

### Assigned Assets
- Devices currently in their custody
- Routers currently in their custody

### Settings (Link)
- Profile: name, email, password change
- Notification preferences

---

## Status & State Reference

These map directly to the database enums — the UI must use these exact states:

| Entity | States |
|---|---|
| Manager | `Pending` · `Verified` · `Suspended` · `Revoked` |
| Link | `Pending` · `Active` · `Inactive` · `Suspended` · `Terminated` |
| Payout | `Pending` · `Completed` · `Failed` · `Cancelled` |
| Device / Router | `With Manager` · `In Transit` · `With Link` · `Offline` · `Lost` |
| Pay Schedule | `Daily` · `Weekly` · `Biweekly` · `Semi-Monthly` · `Monthly` · `Custom` |
| Payment Method | `Bank Transfer` · `Cash` · `Mobile Money` · `Crypto` |
| Currency | `NGN` · `USD` · `GBP` · `EUR` · `CAD` |

---

## Navigation Structure

### Manager
```
Sidebar:
├── Dashboard (home / analytics)
├── Links
│   └── [Link Detail]
├── Devices
├── Routers
├── Overtime Policies
└── Settings
```

### Link (Worker)
```
Sidebar:
├── Home (next payment + summary)
├── Payment History
├── My Details (bank info)
├── Assets (devices & routers)
└── Settings
```

---

## Design Direction

- **Tone**: trustworthy, professional, clear — this is a financial tool handling real money and employment relationships
- **Primary users**: diaspora workers who may switch between mobile and desktop; the Link dashboard in particular should be mobile-first
- **Data density**: the Manager dashboard is information-dense; the Link dashboard is simple and summary-focused
- **Key interaction**: recording a payout should feel fast and low-friction — it's the core action a Manager takes repeatedly
- **Proof of payment**: this is a critical trust feature — it must be easy to upload and easy to view/download
- **Next due date**: should be the most visually prominent piece of information on both dashboards
