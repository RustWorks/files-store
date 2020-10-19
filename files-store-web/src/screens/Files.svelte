<script lang="typescript">
  import { route } from "@bjornlu/svelte-router"
  import { getFiles } from "../FileStoreApi"
  import FsNodes from "../components/FsNodes.svelte"
  import LoaderIcon from "../icons/LoaderIcon.svelte"
  import Uploader from "../components/Uploader.svelte"
  import Breadcrumb from "../components/Breadcrumb.svelte"
  import Button from "../components/Button.svelte"
  import FlyingPanel from "../components/FlyingPanel.svelte"
  import FsNodeSelection from "../components/FsNodeSelection.svelte"

  import { fsNodesStore, wantCreateDirectory, selectedFsNode } from "../stores/store"

  $: filesResponse = getFiles($route.params.id).then(response => {
    fsNodesStore.set(response.childrens)
    return response
  })
</script>

<svelte:window on:hashchange="{() => selectedFsNode.close()}" />

<main class="main">
  {#await filesResponse}
    <LoaderIcon width="{40}" height="{40}" />
  {:then files}
    <div class="tools">
      <Breadcrumb ancestors="{files.ancestors}" />
      <div class="actions">
        <Uploader parent="{files.parent}" label="Upload" />
        <Button label="Create Directory" on:click="{() => wantCreateDirectory.update(v => !v)}" />
      </div>
    </div>
    <FsNodes parentUuid="{files.parent.uuid}" />
    {#if $selectedFsNode.length > 0}
      <FlyingPanel>
        <FsNodeSelection />
      </FlyingPanel>
    {/if}
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</main>

<style>
  .main {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }
  .tools {
    display: flex;
    justify-content: space-between;
    flex-wrap: wrap;
  }
  .actions {
    display: flex;
    padding: 15px;
  }
</style>
