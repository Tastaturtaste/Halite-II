# Roadmap for Halite-II rewrite
- The rewrite shall recreate the Halite-II competition in look and spirit as close as possible. 
- It shall NOT feature or aim at some "Finals", it is intended as a continuously running competition
- It shall remove as much legacy cruft as possible and use a modern dev stack

## Build a simplified version of the original Halite II website frontend
1. Technologies to use:
    - Rust with axum, tower, tokio for the backend. Initially this shall only serve the website and log requests
    - Vue 3.5 using composition API, TypeScript, Vite for the frontend
    - mise to manage all required dev-tools

2. Capabilities/Pages without backing functionality
    - Show available starterkits (airesources) with dummy download button
    - Show instructions/documentation for the challange/game
    - Login with passwordless login (user provided email, one-time-password will be send to him)
        - At this point no email is send, users are automatically logged in after entering their email without authorization
    - Upload area for bots only accessible for logged-in users (dummy/non-functional)
    - Page where users can see their bots including their stats (such as rank, games playes, upload data, etc.)
    - Page for visualization of a given game
    - The pages should look and feel as close as possible to the original Halite II websites
    - The backend should provide all necessary REST endpoints for the website, endpoints only log for now.
