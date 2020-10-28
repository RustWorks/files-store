<script lang="typescript">
  import { _ } from "svelte-intl"
  import { route } from "@bjornlu/svelte-router"

  import { getFiles } from "../FileStoreApi"
  import { fsNodesStore, wantCreateDirectory, selectedFsNode, wantMoveFsNode } from "../stores/store"
  import Header from "../components/Header.svelte"
  import FsNodes from "../components/FsNodes.svelte"
  import LoaderIcon from "../icons/LoaderIcon.svelte"
  import Uploader from "../components/Uploader.svelte"
  import Breadcrumb from "../components/Breadcrumb.svelte"
  import Button from "../components/Button.svelte"
  import FlyingPanel from "../components/FlyingPanel.svelte"
  import FsNodeSelection from "../components/FsNodeSelection.svelte"
  import Modal from "../components/Modal.svelte"
  import MoveFsNodeModal from "../components/MoveFsNodeModal.svelte"

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
    <div class="header">
      <Header />
      <div class="tools">
        <Breadcrumb ancestors="{files.ancestors}" />
        <div class="actions">
          <Uploader parent="{files.parent}" label="{$_('upload')}" />
          <Button label="{$_('createDirectory')}" on:click="{() => wantCreateDirectory.update(v => !v)}" />
        </div>
      </div>
    </div>
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
