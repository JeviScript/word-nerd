<script lang="ts">
  import { user } from "../store/user";
  import Autocomplete from "../lib/Autocomplete.svelte";
  import Topbar from "../lib/Topbar.svelte";
</script>

<Topbar>
  {#if $user}
    {$user?.first_name}
    {$user?.last_name}
  {/if}
</Topbar>

<Autocomplete />

<style>
  :root {
    display: flex;
    justify-content: center;
  }

</style>
