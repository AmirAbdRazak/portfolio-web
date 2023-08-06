# rust-backend
backend for a rust project, frontend is set on Svelte

Backend would probably end up as like a GraphQL endpoint, theres gonna be a few apps since its gonna be for a portfolio website with apps integrated in it
</br>
so main endpoints would be something like main {}, spotifyDataVisualizer {}, and the subsequent apps planned, if they ever make it


# How to setup
Run `cargo make dev` to run it on dev mode with cargo watch for hot reload (kinda), it'll run the cargo watch command locally.
</br></br>
Run `cargo make build` to run it with Dockerfile on release mode (optimized), once `cargo make build` is ran, you can access it with `docker compose exec axum` if need be.

Here is an example of it running with `cargo make build`, docker compose exec isn't really needed here but just to show that it works:
```
$ docker compose exec axum curl "http://localhost:3000/"                      
Hello, here is the result to your query: {"data":{"user":{"getUser":{"id":1337,"username":"amrrzk"}}}}% 
```

# Todo
[ ] Learn Svelte
</br>
[ ] Implment Docker Svelte setup
</br>
[ ] Implement Svelte data visualization
</br>

[ ] Work on Search Spotify New Releases next :D
