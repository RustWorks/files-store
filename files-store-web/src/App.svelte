<script lang="typescript">
  import { RouterView, initHashRouter } from "@bjornlu/svelte-router"

  import _ from "./Intl.svelte"
  import Files from "./screens/Files.svelte"
  import Bin from "./screens/Bin.svelte"
  import Login from "./screens/Login.svelte"

  initHashRouter([
    { path: "/", component: Files },
    { path: "/login", component: Login },
    { path: "/directory/:id", component: Files },
    { path: "/bin", component: Bin },
    { path: "/bin/:id", component: Bin }
  ])
</script>

<RouterView />
