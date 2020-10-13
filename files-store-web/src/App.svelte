<script lang="ts">
    import { getFiles } from "./FileStoreApi"
    const files = getFiles("")
</script>

<div>Files Store</div>

{#await files}
	<p>...waiting</p>
{:then files}
  {#each files as file (file.uuid)}
    <p>The number is {file.uuid}</p>
  {/each}
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

<style>

</style>
