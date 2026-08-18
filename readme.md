# Project Name - Manage Me

The software allows people with workers in diaspora manage the payment of their workers in a single place without their workers having to reach out to them for payments.

## How it works

A user in Diaspora creates an account and adds their links, the links can also invite a manager who will then add their payment amount and structure.
This will enable the "manager" get reminders on the payment of their links depending on their reminder settings. This also allows users to get proof of payment which could be a file uploaded from the device used for payment. It's a simple solution that will have a dashboard, that also allows them see how many links they have, manage their links (add/remove), see how much they payout/receive in the course of the week/month/year.

## Tech Stack for Implementation

- Rust - for back-end server.
- HTMX/WASM - for client-side dashboard.
- SQL (Postgres) - Data layer.

## Justification for using Rust Lang

I have not directly built a web-server in this language since i learnt it. Unlike Javascript and Golang, i have little knowledge on how Rust handles web based applications using Auxm.
