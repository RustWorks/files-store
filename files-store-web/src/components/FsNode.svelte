<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import FileNode from "./FileNode.svelte"
  import DirectoryNode from "./DirectoryNode.svelte"

  export let fsNode: FsNode
</script>

{#if fsNode.node_type === "file"}
	<FileNode fsNode={fsNode} />
{:else if fsNode.node_type === "directory"}
	<DirectoryNode fsNode={fsNode} />
{:else}
	<p>root</p>
{/if}
