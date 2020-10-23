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
  import Modal from "../components/Modal.svelte"
  import MoveFsNodeModal from "../components/MoveFsNodeModal.svelte"

  import { fsNodesStore, wantCreateDirectory, selectedFsNode, wantMoveFsNode } from "../stores/store"

  $: filesResponse = getFiles($route.params.id).then(response => {
    fsNodesStore.set(response.childrens)
    return response
  })
</script>

<svelte:window on:hashchange="{() => selectedFsNode.close()}" />

<main class="main">
  {#await filesResponse}
    <LoaderIcon size="{40}" />
  {:then files}
    <header class="header">
      <Breadcrumb ancestors="{files.ancestors}" />
      <div class="actions">
        <Uploader parent="{files.parent}" label="Upload" />
        <Button label="Create Directory" on:click="{() => wantCreateDirectory.update(v => !v)}" />
      </div>
    </header>
    <FsNodes parentUuid="{files.parent.uuid}" />
    <FlyingPanel selected="{$selectedFsNode.length > 0}" on:click="{() => selectedFsNode.close()}">
      <FsNodeSelection parent="{files.parent}" />
    </FlyingPanel>
    {#if $wantMoveFsNode}
      <Modal title="{`Move ${$wantMoveFsNode?.name}`}" on:close="{() => wantMoveFsNode.set(undefined)}">
        <MoveFsNodeModal parent="{files.parent}" fsNode="{$wantMoveFsNode || files.parent}" />
      </Modal>
    {/if}
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</main>

<style>
  .main {
    height: 100vh;
    display: grid;
    grid-template-rows: 60px auto;
    grid-template-columns: auto;
    grid-template-areas:
      "header"
      "content"
      "side-panel";
  }
  @media screen and (min-width: 700px) {
    .main {
      grid-template-rows: 60px auto;
      grid-template-columns: auto 300px;
      grid-template-areas:
        "header header"
        "content side-panel";
    }
  }
  .header {
    grid-area: header;
    display: flex;
    justify-content: space-between;
    flex-wrap: nowrap;
  }
  .actions {
    display: flex;
    padding: 15px;
  }
</style>
