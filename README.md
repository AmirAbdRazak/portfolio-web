# rust-backend
backend for a rust project, maybe it would be turned into a docker container and jumbled with nextjs, or turned into some monolith leptos :D

Backend would probably end up as like a GraphQL endpoint, theres gonna be a few apps since its gonna be for a portfolio website with apps integrated in it
</br>
so main endpoints would be something like main {}, spotifyDataVisualizer {}, and the subsequent apps planned, if they ever make it


# How to setup
Run `cargo make dev` to run it on dev mode with cargo watch for hot reload (kinda), it'll run the cargo watch command locally so you don't have to access the docker yet (idk how this works havent tried postgres with it yet).
</br></br>
Run `cargo make build` to run it with Dockerfile on release mode (optimized), once `cargo make build` is ran, you can access it with `docker compose exec axum` and append it with any command of your choice.

Here is an example of it running with `cargo make build`:
```
$ docker compose exec axum curl "http://localhost:3000/"                      
Hello, here is the result to your query: {"data":{"user":{"getUser":{"id":1337,"username":"amrrzk"}}}}% 
```
