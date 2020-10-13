<script lang="typescript">
  import { route } from '@bjornlu/svelte-router'

  import FsNode from "../components/FsNode.svelte"
  
  import { getFiles } from "../FileStoreApi"

  $: files = getFiles($route.params.id).then(files => files.filter(f => f.node_type !== "root" && $route.params.id !== f.uuid))
</script>

{#await files}
	<p>...waiting</p>
{:then files}
  {#each files as file (file.uuid)}
    <FsNode fsNode={file} />
  {/each}
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

<style>

</style>
