<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import FileIcon from "../icons/FileIcon.svelte"

  export let fsNode: FsNode
</script>

<div class="fs-node">
  <div class="icon">
    <FileIcon />
  </div>
  <div class="name">{fsNode.name}</div>
</div>

<style>
  .fs-node {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
  }

  .icon {
    width: 60px;
    height: 60px;
    padding: 15px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .name {
    color: var(--primary-text);
  }
</style>
