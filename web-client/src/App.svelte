<script lang="ts">
  import { me } from "./api";
  import Login from "./routes/Login.svelte";
  import NotFound from "./routes/NotFound.svelte";
  import { wrap } from "svelte-spa-router/wrap";
  import Router from "svelte-spa-router";
  import type { SvelteComponent } from "svelte";

  const routes = {
    "/login": Login,

    "/home": wrap({
      asyncComponent: () => import("./routes/Home.svelte") as Promise<{default: typeof SvelteComponent}>,
      conditions: [
        async (_detail) => {
          return await me()
            .then((_res) => true)
            .catch((_) => false);
        },
      ],
    }),
    "*": NotFound,
  };

</script>

<main>
  <Router {routes} />
</main>

<style>
</style>
