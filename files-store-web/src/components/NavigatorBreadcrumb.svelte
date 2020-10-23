<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import HomeIcon from "../icons/HomeIcon.svelte"

  export let ancestors: FsNode[]
  export let onClick: (fsNode: FsNode) => void
</script>

<div class="breadcrumb">
  {#each ancestors as ancestor (ancestor.uuid)}
    {#if ancestor.node_type === 'root'}
      <div class="root" on:click="{() => onClick(ancestor)}">
        <HomeIcon size="{20}" />
      </div>
    {:else}
      <div class="separaton">/</div>
      <div class="crumb" on:click="{() => onClick(ancestor)}">{ancestor.name}</div>
    {/if}
  {/each}
</div>

<style>
  .breadcrumb {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .root {
    cursor: pointer;
  }

  .separaton {
    color: var(--primary);
    margin-left: 5px;
    margin-right: 5px;
  }

  .crumb {
    color: var(--primary);
    text-decoration: none;
    cursor: pointer;
  }

  .crumb:hover {
    text-decoration: underline;
  }
</style>
