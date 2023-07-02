<script lang="ts">
  import { Search } from "carbon-components-svelte";
  import { get_word } from "../api";
  let value = "";

  async function onSubmit() {
    await get_word(value);
  }
</script>

<div>
  <form on:submit|preventDefault={onSubmit}>
    <Search bind:value />
    <button class="button">Search</button>
  </form>
</div>

<style>
  div {
    margin-top: 24px;
    display: flex;
    justify-content: center;
  }

  form {
    width: 100%;
    display: flex;
  }
</style>
