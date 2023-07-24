<script lang="ts">
  import { me } from "./api";
  import Login from "./routes/Login.svelte";
  import NotFound from "./routes/NotFound.svelte";
  import { wrap } from "svelte-spa-router/wrap";
  import Router, { push, type RouteDetail } from "svelte-spa-router";
  import { onMount, type SvelteComponent } from "svelte";

  const home = () => {
    return import("./routes/Home.svelte") as Promise<{
      default: typeof SvelteComponent;
    }>;
  };

  const auth = async (_detail: RouteDetail) => {
    const user = await me()
      .then((_res) => true)
      .catch((_) => false);
    return user;
  };

  const routes = {
    "/login": Login,
    "/": wrap({
      asyncComponent: () => home(),
      conditions: [auth],
    }),
    "*": NotFound,
  };

  onMount(async () => {
    const user = await me();
    if (user) {
      push("/");
    }
  });
</script>

<main>
  <Router {routes} />
</main>

<style>
  main {
    padding: 24px;
  }
</style>
