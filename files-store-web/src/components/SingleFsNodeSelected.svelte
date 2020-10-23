<script lang="typescript">
  import format from "date-fns/format"
  import type { FsNode } from "../FsNode"
  import Button from "./Button.svelte"
  import { wantMoveFsNode } from "../stores/store"
  export let fsNode: FsNode
  export let unselected = false
</script>

<div class="single-fs-node-selected">
  <div class="name">{fsNode.name}</div>
  <div class="uuid">Uuid: {fsNode.uuid}</div>
  <div class="created-at">Created at: {format(new Date(fsNode.created_at), 'dd/MM/yyyy')}</div>
  <div class="updated-at">Updated at: {format(new Date(fsNode.created_at), 'dd/MM/yyyy')}</div>
  <div class="user">User uuid: {fsNode.user_uuid}</div>

  {#if !unselected && fsNode.node_type !== 'root'}
    <Button label="Move" on:click="{() => wantMoveFsNode.set(fsNode)}" />
  {/if}
</div>

<style>
  .single-fs-node-selected {
    word-break: break-all;
    overflow: hidden;
  }
  .name {
    font-size: 1.1rem;
    margin-bottom: 5px;
  }
  .uuid,
  .user,
  .created-at,
  .updated-at {
    font-size: 0.9rem;
  }
</style>
