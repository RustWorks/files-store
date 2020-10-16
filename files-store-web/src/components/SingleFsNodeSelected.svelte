<script lang="typescript">
  import type { FsNode } from "../FsNode"
  export let fsNode: FsNode
</script>

<div class="single-fs-node-selected">{fsNode.name}</div>

<style>
  .single-fs-node-selected {
    word-break: break-all;
    overflow: hidden;
  }
</style>
