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
    <DirectoryEdit parentUuid="{parentUuid}" />
  {/if}
  {#each fsNodes as fsNode (fsNode.uuid)}
    <FsNode fsNode="{fsNode}" />
  {/each}
</div>

<style>
  .fs-nodes {
    border-top: 1px solid var(--border);
  }
</style>
