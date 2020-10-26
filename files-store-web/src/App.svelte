<script lang="typescript">
  import { RouterView, initHashRouter } from "@bjornlu/svelte-router"

  import Files from "./screens/Files.svelte"
  import Bin from "./screens/Bin.svelte"

  initHashRouter([
    { path: "/", component: Files },
    { path: "/directory/:id", component: Files },
    { path: "/bin", component: Bin },
    { path: "/bin/:id", component: Bin }
  ])
</script>

<RouterView />
