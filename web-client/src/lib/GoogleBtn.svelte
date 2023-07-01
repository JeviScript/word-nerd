<script lang="ts">
  import { onMount } from "svelte";
  import { googleSignIn } from "../api";
  import type { GoogleResponse } from "../types";

  let btnRef: HTMLDivElement;

  function onGoogleLogin(response: GoogleResponse) {
    googleSignIn(response.credential);
  }

  function initGoogleBtn() {
    const google = window["google"];
    if (!google) { return; }
    google.accounts.id.initialize({
      client_id:
        "310555099980-g3oicif2up21oalh4h58m7bedsm0crbd.apps.googleusercontent.com",
      callback: onGoogleLogin,
    });

    google.accounts.id.renderButton(btnRef, {
      theme: "filled_black",
      size: "large",
    });
  }

  onMount(() => {
    window.onload = () => {
      initGoogleBtn()
    }
    initGoogleBtn();
  });
</script>

<div bind:this={btnRef} />
