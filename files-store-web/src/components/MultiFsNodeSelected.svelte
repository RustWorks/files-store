<script lang="typescript">
  import type { FsNode } from "../FsNode"
  export let fsNodes: FsNode[]
</script>

<div class="milti-fs-node-selected">Fs nodes selected {fsNodes.length}</div>

<style>
  .milti-fs-node-selected {
    word-break: break-all;
    overflow: hidden;
  }
</style>
