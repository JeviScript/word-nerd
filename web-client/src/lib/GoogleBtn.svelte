<script lang="ts">
  import { onMount } from "svelte";
  import { googleSignIn } from "../api";
  import type { GoogleResponse } from "../types";

  function onGoogleLogin(response: GoogleResponse) {
    googleSignIn(response.credential);
  }

  onMount(() => {
    console.log(import.meta.env);
    window["onGoogleLogin"] = onGoogleLogin;
  });
</script>

<div
  id="g_id_onload"
  data-client_id="310555099980-g3oicif2up21oalh4h58m7bedsm0crbd.apps.googleusercontent.com"
  data-context="signin"
  data-auto_select="true"
  data-ux_mode="popup"
  data-callback="onGoogleLogin"
  data-auto_prompt="false"
/>

<div
  class="g_id_signin"
  data-type="standard"
  data-shape="rectangular"
  data-theme="filled_black"
  data-text="continue_with"
  data-size="large"
  data-logo_alignment="left"
/>
