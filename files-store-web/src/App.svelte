<script lang="typescript">
  import { RouterView, initHashRouter } from "@bjornlu/svelte-router"

  import Files from "./screens/Files.svelte"

  initHashRouter([
    { path: "/", component: Files },
    { path: "/directory/:id", component: Files }
  ])
</script>

<RouterView />
