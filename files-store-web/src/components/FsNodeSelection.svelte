<script lang="typescript">
  import { selectedFsNode } from "../stores/store"
  import MultiFsNodeSelected from "./MultiFsNodeSelected.svelte"
  import SingleFsNodeSelected from "./SingleFsNodeSelected.svelte"
</script>

{#if $selectedFsNode.length === 1}
  <SingleFsNodeSelected fsNode="{$selectedFsNode[0]}" />
{/if}
{#if $selectedFsNode.length > 1}
  <MultiFsNodeSelected fsNodes="{$selectedFsNode}" />
{/if}
