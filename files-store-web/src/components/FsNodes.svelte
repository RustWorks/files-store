<script lang="typescript">
  export let parentUuid: string
  import type { FsNode as Node } from "../FsNode"
  import FsNode from "../components/FsNode.svelte"
  import DirectoryEdit from "../components/DirectoryEdit.svelte"
  import { fsNodesStore, wantCreateDirectory } from "../stores/store"

  let fsNodes: Node[]

  fsNodesStore.subscribe(v => (fsNodes = v))
</script>

<div class="fs-nodes">
  {#if $wantCreateDirectory}
    <DirectoryEdit name="" parentUuid="{parentUuid}" />
  {/if}
  {#if fsNodes.length === 0}
    <div class="empty">Empty</div>
  {/if}
  {#each fsNodes as fsNode (fsNode.uuid)}
    <FsNode fsNode="{fsNode}" />
  {/each}
</div>

<style>
  .fs-nodes {
    grid-area: content;
    flex: 1;
    overflow-y: auto;
    border-top: 1px solid var(--border);
  }

  .empty {
    padding: 20px;
    text-align: center;
    color: var(--secondary-text);
  }
</style>
