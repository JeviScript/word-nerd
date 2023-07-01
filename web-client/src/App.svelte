<script lang="ts">
  import { me } from "./api";
  import Login from "./routes/Login.svelte";
  import NotFound from "./routes/NotFound.svelte";
  import { wrap } from "svelte-spa-router/wrap";
  import Router, { type RouteDetail } from "svelte-spa-router";
  import type { SvelteComponent } from "svelte";

  const home = () => {
    return import("./routes/Home.svelte") as Promise<{
      default: typeof SvelteComponent;
    }>;
  };

  const auth = async (_detail: RouteDetail) => {
    const gg = await me()
      .then((_res) => true)
      .catch((_) => false);
    console.log(gg)
    return gg;
  };

  const routes = {
    "/login": Login,
    "/": wrap({
      asyncComponent: () => home(),
      conditions: [auth],
    }),
    "*": NotFound,
  };
</script>

<main>
  <Router {routes} />
</main>

<style>
</style>
