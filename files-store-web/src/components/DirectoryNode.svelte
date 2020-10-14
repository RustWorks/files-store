<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"
  export let fsNode: FsNode
</script>

<div class="fs-node">
  <div class="icon">
    <DirectoryIcon />
  </div>
  <a class="name" href="#/directory/{fsNode.uuid}">{fsNode.name}</a>
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
    text-decoration: none;
    color: var(--primary-text);
  }
</style>
