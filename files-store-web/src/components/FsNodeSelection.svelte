<script lang="typescript">
  import type { FsNode } from "../FsNode"

  import { selectedFsNode } from "../stores/store"
  import MultiFsNodeSelected from "./MultiFsNodeSelected.svelte"
  import SingleFsNodeSelected from "./SingleFsNodeSelected.svelte"

  export let parent: FsNode
</script>

{#if $selectedFsNode.length === 0}
  <SingleFsNodeSelected unselected fsNode="{parent}" />
{/if}
{#if $selectedFsNode.length === 1}
  <SingleFsNodeSelected fsNode="{$selectedFsNode[0]}" />
{/if}
{#if $selectedFsNode.length > 1}
  <MultiFsNodeSelected fsNodes="{$selectedFsNode}" />
{/if}
