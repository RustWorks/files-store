<script lang="typescript">
  import { route } from "@bjornlu/svelte-router"

  import { Api } from "../services/Api"
  import { fsNodesStore, selectedFsNode } from "../stores/store"
  import Header from "../components/Header.svelte"
  import FsNodes from "../components/FsNodes.svelte"
  import LoaderIcon from "../icons/LoaderIcon.svelte"
  import Breadcrumb from "../components/Breadcrumb.svelte"
  import Button from "../components/Button.svelte"

  $: filesResponse = Api.fsNodes.getFiles($route.params.id, "bin").then(response => {
    fsNodesStore.set(response.childrens)
    return response
  })
</script>

<svelte:window on:hashchange="{() => selectedFsNode.close()}" />

<main class="main">
  {#await filesResponse}
    <LoaderIcon size="{40}" />
  {:then files}
    <div class="header">
      <Header />
      <div class="tools">
        <Breadcrumb ancestors="{files.ancestors}" />
        <div class="actions">
          <Button label="Cleanup" on:click="{() => {}}" />
        </div>
      </div>
    </div>
    <FsNodes parentUuid="{files.parent.uuid}" />
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</main>

<style>
  .main {
    height: 100vh;
    display: grid;
    grid-template-rows: 120px auto 40vh;
    grid-template-columns: auto;
    grid-template-areas:
      "header"
      "content"
      "side-panel";
  }
  @media screen and (min-width: 700px) {
    .main {
      grid-template-rows: 120px auto;
      grid-template-columns: auto 300px;
      grid-template-areas:
        "header header"
        "content side-panel";
    }
  }
  .header {
    grid-area: header;
    display: flex;
    flex-direction: column;
  }
  .tools {
    display: flex;
    justify-content: space-between;
    flex-direction: row;
    flex-wrap: nowrap;
  }
  .actions {
    display: flex;
    padding: 15px;
  }
</style>
