<script lang="typescript">
  import format from "date-fns/format"
  import type { FsNode } from "../FsNode"
  import Button from "./Button.svelte"
  import { wantMoveFsNode, fsNodesStore } from "../stores/store"
  import { getThumbnailUri, deleteFsNode } from "../FileStoreApi"
  export let fsNode: FsNode
  export let unselected = false

  let loading = false

  $: handleDeleteFsNode = () => {
    loading = true
    deleteFsNode(fsNode.uuid)
      .then(() => {
        loading = false
        fsNodesStore.remove(fsNode)
      })
      .catch(error => {
        console.log("deleteFsNode error", error) // TODO handle error
      })
  }
</script>

<div class="single-fs-node-selected">
  <div class="name">{fsNode.name}</div>
  {#if fsNode.node_type === 'file' && fsNode.metadata.type === 'File' && (fsNode.metadata.content_type === 'image/jpeg' || fsNode.metadata.content_type === 'image/png')}
    <div class="thumbnail"><img src="{getThumbnailUri(fsNode.uuid)}" alt="thumbnail" /></div>
  {/if}
  <div class="uuid">Uuid: {fsNode.uuid}</div>
  <div class="created-at">Created at: {format(new Date(fsNode.created_at), 'dd/MM/yyyy')}</div>
  <div class="updated-at">Updated at: {format(new Date(fsNode.created_at), 'dd/MM/yyyy')}</div>
  <div class="user">User uuid: {fsNode.user_uuid}</div>

  {#if !unselected && fsNode.node_type !== 'root'}
    <Button label="Move" on:click="{() => wantMoveFsNode.set(fsNode)}" />
  {/if}
  {#if !unselected && fsNode.node_type !== 'root' && fsNode.node_type !== 'bin'}
    <Button label="Delete" loading="{loading}" on:click="{handleDeleteFsNode}" />
  {/if}
</div>

<style>
  .single-fs-node-selected {
    word-break: break-all;
    overflow: hidden;
  }
  .name {
    font-size: 1.1rem;
    font-weight: bold;
    margin-bottom: 5px;
  }
  .thumbnail {
    width: 200px;
  }
  .uuid,
  .user,
  .created-at,
  .updated-at {
    font-size: 0.9rem;
  }
</style>
