---
sidebar_position: 1
---

# Intro

Let's get into starting to model the business domain. No low level systems programming here, let's build something real together.

If you recall, functionality wise you need to build a service that allows users to:

- Register a new account
- Login
- Retrieve their account information
- Update their account details

Which means you need to model a `User`. `Users` are also going to need properties, you know things like `first_name`, `last_name`. So let's look at the different data types available in Rust that you'll use.